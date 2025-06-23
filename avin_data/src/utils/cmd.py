#!/usr/bin/env python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

import json
import os
import shutil
import subprocess
import tomllib
import zipfile
from collections import deque

import polars as pl


class Cmd:
    @staticmethod
    def path(*path_parts) -> str:
        path = os.path.join(*path_parts)
        return path

    @staticmethod
    def make_dirs(path: str) -> None:
        """Создает все необходимые папки для этого пути"""

        if not os.path.exists(path):
            os.makedirs(path)

    @staticmethod
    def name(file_path: str, extension: bool = False) -> str:
        """Отделяет имя файла из пути к файлу
        /home/file_name.txt -> file_name.txt  # extension=True
        /home/file_name.txt -> file_name  # extension=False
        """

        file_name = os.path.basename(file_path)  # == somename.xxx

        if extension:
            return file_name

        name = os.path.splitext(file_name)[0]  # == somename
        return name

    @staticmethod
    def dir_name(file_path: str) -> str:
        assert Cmd.is_file(file_path)

        file_path = os.path.dirname(file_path)
        dir_name = os.path.basename(file_path)

        return dir_name

    @staticmethod
    def dir_path(file_path: str) -> str:
        assert Cmd.is_file(file_path)

        dir_path = os.path.dirname(file_path)

        return dir_path

    @staticmethod
    def is_exist(path: str) -> bool:
        return os.path.exists(path)

    @staticmethod
    def is_file(path: str) -> bool:
        return os.path.isfile(path)

    @staticmethod
    def is_dir(path: str) -> bool:
        return os.path.isdir(path)

    @staticmethod
    def content(dir_path: str, full_path=False) -> list[str]:
        names = os.listdir(dir_path)

        contents = list()
        for name in names:
            if full_path:
                path = Cmd.path(dir_path, name)
                contents.append(path)
            else:
                contents.append(name)

        return contents

    @staticmethod
    def get_files(
        dir_path: str, full_path=False, include_sub_dir=False
    ) -> list[str]:
        if include_sub_dir:
            return Cmd.__get_files_in_dir_include_subdir(dir_path, full_path)

        return Cmd.__get_files_in_dir(dir_path, full_path)

    @staticmethod
    def get_dirs(dir_path: str, full_path=False) -> list[str]:
        """Возвращает список папок в 'dir_path' без обхода подпапок"""
        names = os.listdir(dir_path)

        list_dirs = list()
        for name in names:
            path = Cmd.path(dir_path, name)
            if os.path.isdir(path):
                if full_path:
                    list_dirs.append(path)
                else:
                    list_dirs.append(name)

        return list_dirs

    @staticmethod
    def find_file(file_name: str, dir_path: str) -> str | None:
        for root, _dirs, files in os.walk(dir_path):
            if file_name in files:
                return os.path.join(root, file_name)

        return None

    @staticmethod
    def find_dir(dir_name: str, root_dir: str) -> str | None:
        for root, dirs, _files in os.walk(root_dir):
            if dir_name in dirs:
                return os.path.join(root, dir_name)

        return None

    @staticmethod
    def select(files: list[str], name=None, extension=None) -> list[str]:
        """Список файлов c именем 'name', и/или расширением 'extension'"""

        selected = list()
        for file in files:
            if name is not None and name == Cmd.name(file):
                selected.append(file)
            if extension is not None and file.endswith(extension):
                selected.append(file)

        return selected

    @staticmethod
    def rename(old_path: str, new_path: str) -> None:
        """Переименовывает old_path в new_path"""

        os.rename(old_path, new_path)

    @staticmethod
    def replace(src: str, dest: str, create_dirs=True) -> None:
        """Перемещает src в dest"""

        if create_dirs:
            Cmd.__create_dirs_for_filepath(dest)

        os.replace(src, dest)

    @staticmethod
    def copy(src_file: str, dest_file: str) -> None:
        """Копирует src в dest"""

        shutil.copy(src_file, dest_file)

    @staticmethod
    def copy_dir(src: str, dest: str) -> None:
        """Копирует src в dest"""

        shutil.copytree(src, dest)

    @staticmethod
    def delete(file_path: str) -> None:
        """Удаляет файла по указанному пути"""

        os.remove(file_path)

    @staticmethod
    def delete_dir(path: str) -> None:
        shutil.rmtree(path)

    @staticmethod
    def extract(archive_path: str, dest_dir: str) -> None:
        with zipfile.ZipFile(archive_path, "r") as file:
            file.extractall(dest_dir)

    @staticmethod
    def read(file_path: str) -> str:
        """Read file as one string"""

        with open(file_path, encoding="utf-8") as file:
            string = file.read()

        return string

    @staticmethod
    def write(string: str, file_path: str, create_dirs=True) -> None:
        """Write string in file (overwrite)"""
        if create_dirs:
            Cmd.__create_dirs_for_filepath(file_path)

        with open(file_path, "w", encoding="utf-8") as file:
            file.write(string)

    @staticmethod
    def append(text: list[str], file_path: str) -> None:
        """Append text in file"""

        with open(file_path, "a", encoding="utf-8") as file:
            for line in text:
                file.write(line)

    @staticmethod
    def get_tail(file_path: str, n: int) -> list[str]:
        # Читает весь файл построчно и добавляет его в
        # очередь, у которой максимальная длина n... таким образом
        # дойдя до конца файла в очереди останется n последних строк

        with open(file_path, encoding="utf-8") as file:
            text = list(deque(file, n))

        return text

    @staticmethod
    def read_text(file_path: str) -> list[str]:
        """Read file by row, return list[str]"""

        text = list()
        with open(file_path, encoding="utf-8") as file:
            for line in file:
                text.append(line)

        return text

    @staticmethod
    def write_text(text: list[str], file_path: str, create_dirs=True) -> None:
        if create_dirs:
            Cmd.__create_dirs_for_filepath(file_path)

        with open(file_path, "w", encoding="utf-8") as file:
            for line in text:
                file.write(line)

    @staticmethod
    def read_json(file_path: str, decoder=None):
        with open(file_path, encoding="utf-8") as file:
            obj = json.load(
                fp=file,
                object_hook=decoder,
            )

        return obj

    @staticmethod
    def write_json(
        obj, file_path, encoder=None, indent=4, create_dirs=True
    ) -> None:
        if create_dirs:
            Cmd.__create_dirs_for_filepath(file_path)

        with open(file_path, "w", encoding="utf-8") as file:
            json.dump(
                obj=obj,
                fp=file,
                indent=indent,
                default=encoder,
                ensure_ascii=False,
            )

    @staticmethod
    def from_json_str(string: str, decoder=None):
        obj = json.loads(
            string,
            object_hook=decoder,
        )

        return obj

    @staticmethod
    def to_json_str(obj, encoder=None, indent=0) -> str:
        string = json.dumps(
            obj=obj,
            indent=indent,
            default=encoder,
            ensure_ascii=False,
        )

        return string

    @staticmethod
    def read_toml(file_path: str):
        with open(file_path, "rb") as f:
            data = tomllib.load(f)

        return data

    @staticmethod
    def read_pqt(path: str) -> pl.DataFrame:
        df = pl.read_parquet(path)

        return df

    @staticmethod
    def write_pqt(
        df: pl.DataFrame, path: str, create_dirs: bool = True
    ) -> None:
        if create_dirs:
            Cmd.__create_dirs_for_filepath(path)

        df.write_parquet(path)

    @staticmethod
    def subprocess(command: list[str]) -> None:
        """
        import platform
        import subprocess
        # define a command that starts new terminal
        if platform.system() == "Windows":
            new_window_command = "cmd.exe /c start".split()
        else:
            new_window_command = "x-terminal-emulator -e".split()
        subprocess.check_call(new_window_command + command)
        """
        subprocess.call(command)

    @staticmethod
    def __get_files_in_dir(dir_path, full_path):
        all_files = list()
        names = os.listdir(dir_path)
        for name in names:
            path = Cmd.path(dir_path, name)
            if os.path.isfile(path):
                if full_path:
                    all_files.append(path)
                else:
                    all_files.append(name)
        return all_files

    @staticmethod
    def __get_files_in_dir_include_subdir(dir_path, full_path):
        all_files = list()
        for root, _dirs, files in os.walk(dir_path):
            if full_path:
                for f in files:
                    path = Cmd.path(root, f)
                    all_files.append(path)
            else:
                all_files += files

        return all_files

    @staticmethod
    def __create_dirs_for_filepath(file_path) -> None:
        dir_path = os.path.dirname(file_path)

        if not os.path.exists(dir_path):
            os.makedirs(dir_path)

    # @staticmethod
    # def write_bin(
    #     obj: object, path: str, compres=False, create_dirs=True
    # ) -> None:
    #     if create_dirs:
    #         Cmd.__create_dirs_for_filepath(path)
    #
    #     fh = None
    #     try:
    #         fh = gzip.open(path, "wb") if compres else open(path, "wb")
    #         pickle.dump(obj, fh, pickle.HIGHEST_PROTOCOL)
    #
    #     except (OSError, pickle.PicklingError):
    #         exit(1)
    #
    #     finally:
    #         if fh is not None:
    #             fh.close()

    # @staticmethod
    # def read_bin(file_path) -> object:
    #     GZIP_MAGIC = b"\x1f\x8b"  # метка .gzip файлов
    #
    #     try:
    #         fh = open(file_path, "rb")
    #         magic = fh.read(len(GZIP_MAGIC))
    #         if magic == GZIP_MAGIC:
    #             fh.close()
    #             fh = gzip.open(file_path, "rb")
    #         else:
    #             fh.seek(0)
    #         obj = pickle.load(fh)
    #         return obj
    #
    #     except (OSError, pickle.UnpicklingError):
    #         exit(1)
    #
    #     finally:
    #         if fh is not None:
    #             fh.close()


if __name__ == "__main__":
    ...
