"""从命令标准输入读取 ctx；缺少可靠语义信息时返回空字符串。"""

import json
import sys


ctx = json.load(sys.stdin)
if ctx["schema_version"] != 1:
    raise ValueError("unsupported ctx schema_version")

symbol = ctx.get("symbol") or {}
sys.stdout.write(symbol.get("name") or "")
