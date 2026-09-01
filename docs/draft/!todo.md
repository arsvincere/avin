    [ ] avin_system - запуск с невалидным data.toml даст ошибку про
        инвалид timeframe - то есть нихера не понятно что случилось..
        нужно сделать AvinError::Workspace хотя бы и в нем оборачивать
        ошибки и добавлять контекст как сделано в AvinError::InstrumentInfo
    [ ] DataProvider
        [ ] bind
        [ ] stub
        [ ] py
    [ ] MarketData
        [ ] bind
        [ ] stub
        [ ] py
    [ ] Asset Share Future
        [ ] py
    [ ] Watchlist
        [ ] py
