# Loader
Loader::chart(asset, tf) -> Chart  // latest default bars count
Loader::chart_latest(asset, tf, quantity) -> Chart
Loader::chart_period(asset, tf, range) -> Chart

Вообще кажется вот здесь как раз хер придумаешь линейный список нормально покрывающий все юзер кейсы... и тут то билдер синтакс как раз может быть оправдан... Попробуем:

Loader::chart(asset, tf)
    .latest() -> Chart
Loader::chart(asset, tf)
    .latest_n(n: Quantity) -> Chart
Loader::chart(asset, tf)
    .period(range) -> Chart

или

Loader::chart(asset, tf)
    .default() -> Chart
Loader::chart(asset, tf)
    .latest(n: Quantity) -> Chart
Loader::chart(asset, tf)
    .period(range) -> Chart

Это особенно становится полезно в footprint где будет time_footprint / tick_footprint / volume_footprint / value_footprint и еще это все перемножить на range / latest / default latest = ебейших список методов. А можно... Разные варианты:

Loader::footprint(asset, tf)
    .tick()
    .period(range)

Loader::footprint(asset, tf, type)
    .period(range)
