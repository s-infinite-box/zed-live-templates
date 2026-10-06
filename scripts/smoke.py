"""通过真实 LSP 标准输入输出验证模板展开，无需启动 Zed。"""

import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BINARY = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT / 'target/debug/server'


def send(process, method, params=None, request_id=None):
    message = {'jsonrpc': '2.0', 'method': method}
    if params is not None:
        message['params'] = params
    if request_id is not None:
        message['id'] = request_id
    data = json.dumps(message, ensure_ascii=False).encode()
    process.stdin.write(f'Content-Length: {len(data)}\r\n\r\n'.encode() + data)
    process.stdin.flush()


def receive(process):
    headers = {}
    while (line := process.stdout.readline()) != b'\r\n':
        if not line:
            raise RuntimeError('language server closed stdout')
        name, value = line.decode().split(':', 1)
        headers[name.lower()] = value.strip()
    response = json.loads(process.stdout.read(int(headers['content-length'])))
    assert 'error' not in response, response
    return response['result']


with tempfile.TemporaryDirectory(prefix='live-templates-') as directory:
    directory = Path(directory)
    examples = directory / 'examples'
    shutil.copytree(ROOT / 'examples', examples)
    config = examples / 'templates.toml'
    with subprocess.Popen([str(BINARY), '--config', str(config)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE) as process:
        try:
            send(process, 'initialize', {
                'processId': None,
                'rootUri': directory.as_uri(),
                'capabilities': {'textDocument': {'completion': {'completionItem': {
                    'snippetSupport': True, 'insertTextModeSupport': {'valueSet': [1]}
                }}}},
            }, 1)
            assert receive(process)['capabilities']['textDocumentSync'] == 1
            send(process, 'initialized', {})
            uri = (directory / 'note.md').as_uri()
            prefix = '记录🙂 '
            text = prefix + 'dt'
            send(process, 'textDocument/didOpen', {'textDocument': {
                'uri': uri, 'languageId': 'markdown', 'version': 1, 'text': text
            }})

            def complete(text, version, request_id):
                if version > 1:
                    send(process, 'textDocument/didChange', {
                        'textDocument': {'uri': uri, 'version': version},
                        'contentChanges': [{'text': text}],
                    })
                send(process, 'textDocument/completion', {
                    'textDocument': {'uri': uri},
                    'position': {'line': 0, 'character': len(text.encode('utf-16-le')) // 2},
                }, request_id)
                return receive(process)

            item = complete(text, 1, 2)[0]
            assert item['label'] == 'dt' and item['insertTextFormat'] == 2
            assert item['textEdit']['range']['start']['character'] == 5
            assert re.fullmatch(r'# 周[一二三四五六日] \d{4}/\d+/\d+ \d{2}:\d{2} .+\n\$0\n---', item['textEdit']['newText'])
            assert complete('todo', 2, 3)[0]['textEdit']['newText'] == '- [ ] $0'

            # 新模板在下一次补全生效；Python 读取真实 ctx，输出作为字面量插入。
            (examples / 'variables/probe.py').write_text(
                'import json, sys\nctx = json.load(sys.stdin)\n'
                'sys.stdout.write(ctx["template"]["trigger"] + ":" + ctx["variable"]["name"] + ":$0}")\n'
            )
            with config.open('a') as file:
                file.write('\n[variables.probe]\ncommand = ["python3", "variables/probe.py"]\n'
                           '\n[[templates]]\nid = "context"\ntrigger = "ctx"\nlanguages = ["Markdown"]\n'
                           'body = "$probe$|$probe$$END$"\n')
            assert complete('ctx', 3, 4)[0]['textEdit']['newText'] == 'ctx:probe:\\$0\\}|ctx:probe:\\$0\\}$0'

            # dt 在配置中排在前面，@dt 仍应匹配自身并替换整个触发串。
            with config.open('a') as file:
                file.write('\n[[templates]]\nid = "prefixed"\ntrigger = "@dt"\n'
                           'languages = ["Markdown"]\nbody = "PREFIXED$END$"\n')
            item = complete('@dt', 4, 5)[0]
            assert item['label'] == '@dt' and item['textEdit']['newText'] == 'PREFIXED$0'
            assert item['textEdit']['range']['start']['character'] == 0

            # 删除配置后停止提供模板，恢复相同内容后重新加载。
            source = config.read_text()
            config.unlink()
            assert complete('dt', 5, 6) is None
            config.write_text(source)
            assert complete('dt', 6, 7)[0]['label'] == 'dt'
            send(process, 'shutdown', request_id=8)
            receive(process)
            send(process, 'exit')
            process.stdin.close()
            process.wait(timeout=3)
            assert process.returncode == 0
        finally:
            if process.poll() is None:
                process.kill()
                process.wait()
            diagnostic = process.stderr.read().decode()
            if diagnostic:
                print(diagnostic, file=sys.stderr)

print('OK: dt / todo / ctx / @dt, Bash / Python, UTF-16 range, cursor, snippet escaping, configuration reload / removal / restoration')
