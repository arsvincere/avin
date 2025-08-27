#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

from datetime import timedelta as TimeDelta
from pathlib import Path

from avin_data.utils.cmd import Cmd
from avin_data.utils.exceptions import ConfigNotFound

__all__ = "cfg"


class Configuration:
    def __init__(self, file_path: Path):
        self.__path = file_path
        self.__cfg = Cmd.read_toml(file_path)

    @property
    def root(self) -> Path:
        return Path.home() / self.__cfg["dir"]["root"]

    @property
    def data(self) -> Path:
        return Path.home() / self.__cfg["dir"]["data"]

    @property
    def tinkoff_token(self) -> Path:
        return Path.home() / self.__cfg["connect"]["tinkoff"]

    @property
    def moex_account(self) -> Path:
        return Path.home() / self.__cfg["connect"]["moex_account"]

    @property
    def moex_token(self) -> Path:
        return Path.home() / self.__cfg["connect"]["moex_token"]

    @property
    def log(self) -> Path:
        return Path(self.root, "log")

    @property
    def res(self) -> Path:
        return Path(self.root, "res")

    @property
    def tmp(self) -> Path:
        return Path(self.root, "tmp")

    @property
    def connect(self) -> Path:
        return Path(self.root, "connect")

    @property
    def cache(self) -> Path:
        return Path(self.data, "cache")

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

    @classmethod
    def read_config(cls) -> Configuration:
        """Try find and read config

        First try in current dir, then in ~/.config/avin/config.toml,
        then use config.toml from res/

        Returns:
            Configuration.

        Raises:
            ConfigNotFound if config not exists.
        """

        file_name = "config.toml"

        # try find user config in current dir
        path = Path(Path.cwd(), file_name)
        if path.exists():
            return Configuration(path)

        # try find in user home ~/.config/avin/
        path = Path(Path.home(), ".config", "avin", file_name)
        if path.exists():
            return Configuration(path)

        # try use default config
        path = (
            Path(__file__).parent.parent.parent.parent / "res" / "config.toml"
        )
        if path.exists():
            return Configuration(path)

        raise ConfigNotFound(f"Config file not found: {path}")


if __name__ == "__main__":
    ...
else:
    cfg = Configuration.read_config()
