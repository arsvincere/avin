import click

from avin_data import Manager, MarketData, Source


@click.group()
def cli():
    """Консольная утилита для загрузки рыночных данных

    Первое что нужно сделать - кэшировать информацию о доступных инструментах.
    Выполните:

        avin_data cache

    Теперь вы можете выполнять поиск инструментов и просматривать информацию
    о них. Например:

        avin_data find -i moex_share_sber

    Загрузка дневных баров Сбер банка за 2025г:

        avin_data download -s moex -i moex_share_sber -d bar_d --year 2025

    Обновить все имеющиеся данные:

        avin_data update

    Подробнее об использовании команд:

        avin_data <command> --help

    Программа может использоваться отдельно, или как часть AVIN Trade System
    Подробнее: https://github.com/arsvincere/avin
    """
    pass


@cli.command()
def cache():
    """Кэширование информации об инструментах

    Пока доступны данные только с Московской биржи.
    """

    Manager.cache()


@cli.command()
@click.option("--instrument", "-i", help="Идентификатор инструмента")
def find(instrument):
    """Поиск информации об инструменте

    Формат идентификатора инструмента: <exchange>_<category>_<ticker>

        exchange: [moex]

        category: [index, share, bond, future, option, etf]

        ticker: [gazp, lkoh, rosn, ... ]

    Пример: avin_data find -i moex_share_sber

    """

    result = Manager.find(instrument)
    print(result.pretty())


@cli.command()
@click.option("--instrument", "-i", help="Идентификатор инструмента")
@click.option("--source", "-s", help="Источник рыночных данных")
@click.option("--data", "-d", help="Тип данных")
@click.option("--year", "-y", help="Год")
def download(source, instrument, data, year):
    """Загрузка рыночных данных

    Примеры:

    1. Загрузить дневные бары Сбер банка за 2025г:

        avin_data download -i moex_share_sber -s moex  -d bar_d -y 2025

    2. Загрузить все 1H бары Газпрома:

        avin_data download -i moex_share_gazp -s moex -d bar_1h

    """

    source = Source.from_str(source)
    iid = Manager.find(instrument)
    market_data = MarketData.from_str(data)

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
