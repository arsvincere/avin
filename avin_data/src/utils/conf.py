#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import os
import sys
from datetime import timedelta as TimeDelta

from src.utils.cmd import Cmd

__all__ = "cfg"


class Configuration:
    def __init__(self, file_path: str):
        self.__path = file_path
        self.__cfg = Cmd.read_toml(file_path)

    @property
    def root(self) -> str:
        return self.__cfg["dir"]["root"]

    @property
    def data(self) -> str:
        return self.__cfg["dir"]["data"]

    @property
    def log(self) -> str:
        return Cmd.path(self.root, "log")

    @property
    def res(self) -> str:
        return Cmd.path(self.root, "res")

    @property
    def tmp(self) -> str:
        return Cmd.path(self.root, "tmp")

    @property
    def connect(self) -> str:
        return Cmd.path(self.root, "connect")

    @property
    def cache(self) -> str:
        return Cmd.path(self.data, "cache")

    @property
    def log_history(self) -> int:
        return self.__cfg["log"]["history"]

    @property
    def log_debug(self) -> bool:
        return self.__cfg["log"]["debug"]

    @property
    def log_info(self) -> bool:
        return self.__cfg["log"]["info"]

    @property
    def offset(self) -> TimeDelta:
        return TimeDelta(hours=self.__cfg["usr"]["offset"])

    @property
    def dt_fmt(self) -> str:
        return self.__cfg["usr"]["dt_fmt"]

    @property
    def tinkoff_token(self) -> str:
        return self.__cfg["connect"]["tinkoff"]

    @property
    def moex_account(self) -> str:
        return self.__cfg["connect"]["moexalgo"]

    @classmethod
    def read_config(cls) -> Configuration:
        file_name = "config.toml"

        # try find user config in current dir
        pwd = os.getcwd()
        path = Cmd.path(pwd, file_name)
        if Cmd.is_exist(path):
            return Configuration(path)

        # try find in user home ~/.config/avin/
        path = Cmd.path("~/.config/avin", file_name)
        if Cmd.is_exist(path):
            return Configuration(path)

        # try use default config
        path = "/home/alex/avin/res/default_config.toml"
        if Cmd.is_exist(path):
            return Configuration(path)

        # panic
        print(f"Config file not found: {path}", file=sys.stderr)
        exit(1)


if __name__ == "__main__":
    ...
else:
    cfg = Configuration.read_config()
