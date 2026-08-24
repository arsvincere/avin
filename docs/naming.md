# Primary rule

Используется самое короткое имя, сохраняющее однозначный смысл в текущем контексте.

Не добавлять в имя информацию, уже очевидную из module, class, types или docstring.

Более длинное имя используется только для устранения реальной неоднозначности.

# Abbreviations

Устоявшиеся в ходе разработки сокращения:

```text
iid - Instrument ID
dt  - datetime
ts  - timestamp
tf  - timeframe
md  - market data
fp  - footprint
ws  - workspace
```

# Python Exceptions

Все custom exception classes AVIN используют suffix `Error`.

# Temporary development names

Временные development/debugging elements маркируются:

```text
DEV_*  | DBG_*    - constants
dev_*  | dbg_*    - functions and variables
# dev  | # dbg    - py code blocks
// dev | // dbg   - rs code blocks
```

Маркировка означает, что element не является продакшен кодом, и если не помнишь
нахер он тут остался - скорее всего просто забыл его удалить после работы.
