#!/usr/bin/env bash

# TPL_WEEKDAY 来自 ctx.now.weekday；不要重新调用 date 获取另一个时间快照。
# 这里的中文映射由用户脚本提供，插件只传入周一为 1、周日为 7 的编号。
case "${TPL_WEEKDAY-}" in
    1) value=一 ;;
    2) value=二 ;;
    3) value=三 ;;
    4) value=四 ;;
    5) value=五 ;;
    6) value=六 ;;
    7) value=日 ;;
    *) printf '%s\n' 'TPL_WEEKDAY must be an integer from 1 to 7' >&2; exit 2 ;;
esac

printf '%s' "$value"
