#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import polars as pl

from src.manager.category import Category
from src.manager.source import Source
from src.utils import Cmd, cfg, log


class IidCache:
    def __init__(
        self,
        source: Source,
        category: Category,
        iid_df: pl.DataFrame,
    ):
        assert isinstance(source, Source)
        assert isinstance(category, Category)
        assert isinstance(iid_df, pl.DataFrame)

        self.__source = source
        self.__category = category
        self.__iid_df = iid_df

    def source(self) -> Source:
        return self.__source

    def category(self) -> Category:
        return self.__category

    def df(self) -> pl.DataFrame:
        return self.__iid_df

    def path(self) -> str:
        file_path = self.__create_file_path(self.__source, self.__category)
        return file_path

    @classmethod
    def save(cls, cache: IidCache) -> None:
        assert isinstance(cache, IidCache)

        path = cache.path()
        df = cache.df()
        Cmd.write_pqt(df, path)

        log.info(f"Cache save: {path}")

    @classmethod
    def load(cls, source: Source, category: Category) -> IidCache:
        path = cls.__create_file_path(source, category)
        df = Cmd.read_pqt(path)
        cache = IidCache(source, category, df)

        return cache

    @classmethod
    def __create_file_path(cls, source: Source, category: Category) -> str:
        cache_path = Cmd.path(
            cfg.cache,
            source.name,
            f"{category.name}.pqt",
        )

        return cache_path


if __name__ == "__main__":
    ...
