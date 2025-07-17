import click

from src.manager import Manager, MarketData, Source


@click.group()
def cli():
    """Консольная утилита для загрузки рыночных данных

    Первое что нужно сделать - кэшировать информацию о доступных инструментах.
    Выполните:

        avin-data cache

    Теперь вы можете выполнять поиск инструментов и просматривать информацию
    о них. Например:

        avin-data find -i moex_share_sber

    Загрузка дневных баров Сбер банка за 2025г:

        avin-data download -s moex -i moex_share_sber -d bar_d --year 2025

    Обновить все имеющиеся данные:

        avin-data update

    Подробнее об использовании команд:

        avin-data <command> --help

    Программа может использоваться отдельно, или как часть AVIN Trade System
    Подробнее: https://github.com/arsvincere/avin
    """
    pass


@cli.command()
@click.option("--source", "-s", default="all", help="Источник данных")
def cache(source: str):
    """Кэширование информации об инструментах

    Пока доступны данные только с Московской биржи.
    """

    match source.upper():
        case "ALL":
            for i in Source:
                Manager.cache(i)
        case "MOEX":
            Manager.cache(Source.MOEX)
        case "TINKOFF":
            Manager.cache(Source.TINKOFF)


@cli.command()
@click.option("--instrument", "-i", help="Идентификатор инструмента")
def find(instrument: str):
    """Поиск информации об инструменте

    Формат идентификатора инструмента: <exchange>_<category>_<ticker>

        exchange: [moex]

        category: [index, share, bond, future, option, etf]

        ticker: [gazp, lkoh, rosn, ... ]

    Пример: avin-data find -i moex_share_sber

    """

    result = Manager.find(instrument)
    if result:
        print(result.pretty())
    else:
        print("not found")


@cli.command()
@click.option("--instrument", "-i", help="Идентификатор инструмента")
@click.option("--source", "-s", default="moex", help="Источник данных")
@click.option("--data", "-d", default="all", help="Тип данных")
@click.option("--year", "-y", help="Год")
def download(source, instrument, data, year):
    """Загрузка рыночных данных

    Примеры:

    1. Загрузить дневные бары Сбер банка за 2025г:

        avin-data download -i moex_share_sber -s moex  -d bar_d -y 2025

    2. Загрузить все 1H бары Газпрома:

        avin-data download -i moex_share_gazp -s moex -d bar_1h

    3. Загрузить тиковые данные Роснефть за сегодня:

        avin-data download -i moex_share_rosn -s moex -d tic

    4. Загрузить все типы данных Яндекс за все годы:

        avin-data download -i moex_share_ydex
    """
    ALL = ["TIC", "1M", "10M", "1H", "D", "W", "M"]

    iid = Manager.find(instrument)
    source = Source.from_str(source)
    data = ALL if data == "all" else [data]

    for i in data:
        market_data = MarketData.from_str(i)
        if year is None:
            Manager.download(source, iid, market_data)
        else:
            Manager.download(source, iid, market_data, year=int(year))


@cli.command()
def update():
    """Обновление имеющихся данных"""

    Manager.update_all()


if __name__ == "__main__":
    cli()
