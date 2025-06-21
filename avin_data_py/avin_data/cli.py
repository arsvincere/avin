import click

from avin_data import Category, Manager


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
def find(iid):
    """Поиск информации об инструменте

    Формат идентификатора инструмента: <exchange>_<category>_<ticker>

        exchange: [moex]

        category: [index, share, bond, future, option, etf]

        ticker: [gazp, lkoh, rosn, ... ]

    Пример: moex_share_sber

    """
    print(f"iid={iid}")
    click.echo("Find ...")


@cli.command()
@click.option("--source", "-s", help="Источник рыночных данных: [moex]")
@click.option(
    "--instrument",
    "-i",
    help="Идентификатор инструмента: <exchange>_<category>_<ticker>",
)
@click.option(
    "--data",
    "-d",
    help="Тип данных: [BAR_M, BAR_W, BAR_D, BAR_1H, BAR_10M, BAR_1M, TIC]",
)
def download(source, iid, data):
    """Загрузка рыночных данных

    Примеры:

    1. Загрузить дневные бары Сбер банка за 2025г:

        avin_data download -s moex -i moex_share_sber -d bar_d -y 2025

    2. Загрузить все 1H бары Газпрома:

        avin_data download -s moex -i moex_share_gazp -d bar_1h

    """

    print(f"source={source} iid={iid} data={data}")
    click.echo("Downloading ...")


@cli.command()
def update():
    """Обновление имеющихся данных"""
    click.echo("Updating ...")


if __name__ == "__main__":
    cli()
