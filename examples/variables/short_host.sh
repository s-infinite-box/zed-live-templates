#!/usr/bin/env bash

# TPL_HOST 是执行器从同一份 ctx.builtins.host 快照提供的内置值。
# printf 不额外添加换行；这里只保留 hostname 的第一段。
printf '%s' "${TPL_HOST%%.*}"
