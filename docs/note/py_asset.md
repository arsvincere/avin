Да, это как раз решается на уровне typing, без runtime Asset класса.

Самый простой вариант:

type Asset = Share | Future | Bond

И дальше:

class AssetList:
    def add(self, asset: Asset) -> None:
        ...
class Strategy:
    def process(self, asset: Asset) -> None:
        ...

Для mypy/pyright это нормальный общий тип:

share = Share(...)
future = Future(...)

asset_list.add(share)   # ok
asset_list.add(future)  # ok

strategy.process(share)   # ok
strategy.process(future)  # ok

А вот:

asset_list.add(Chart(...))

уже даст type error.

И при возврате из Rust тоже все красиво

Допустим:

def asset(self, index: int) -> Asset:
    ...

Runtime реально вернет:

Share

или:

Future

Но статический тип метода:

Share | Future | Bond

То есть:

asset = asset_list[0]

reveal_type(asset)
# Share | Future | Bond

После проверки тип сужается:

if isinstance(asset, Future):
    reveal_type(asset)
    # Future

    print(asset.expiration)

Это называется narrowing.

А если нужен только общий interface

Тогда есть второй вариант — Protocol.

Например:

from typing import Protocol

class Asset(Protocol):
    @property
    def iid(self) -> InstrumentId: ...

    @property
    def ticker(self) -> Ticker: ...

    def chart(self, tf: TimeFrame) -> Chart: ...

И:

class Strategy:
    def process(self, asset: Asset) -> None:
        ...

Share, Future, Bond даже не обязаны наследоваться от Asset.

Если у них есть нужные методы/properties:

class Share:
    @property
    def iid(...): ...

    @property
    def ticker(...): ...

    def chart(...): ...

то mypy считает:

Share conforms to Asset

Это Python-аналог Rust trait по смыслу.
