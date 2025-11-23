# Быстрое развертывание системы AVIN на Linux

**Дата создания:** 2025-11-22  
**Последнее обновление:** 2025-11-22  
**Версия:** 2.0

**Примечание:** Для развертывания на Windows см. отдельную инструкцию [deployment_windows.md](./deployment_windows.md).

---

## Требования

- **ОС**: Ubuntu 20.04+ / Debian 11+ / Fedora 34+ / Arch Linux
- **Python**: версия 3.13.1
- **Rust**: версия 1.91.1
- **RAM**: минимум 4 GB
- **Дисковое пространство**: минимум 10 GB

---

## Быстрая установка

### 1. Установка системных зависимостей

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  libglib2.0-dev \
  libgdk-pixbuf2.0-dev \
  libxdo-dev \
  libayatana-appindicator3-dev \
  make \
  curl \
  wget \
  git \
  libbz2-dev \
  libreadline-dev \
  libsqlite3-dev \
  libncursesw5-dev \
  xz-utils \
  tk-dev \
  libxml2-dev \
  libxmlsec1-dev \
  libffi-dev \
  liblzma-dev
```

### 2. Установка Python 3.13.1 через pyenv

```bash
# Установка pyenv
curl https://pyenv.run | bash

# Добавление в PATH
echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.bashrc
echo 'command -v pyenv >/dev/null || export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.bashrc
echo 'eval "$(pyenv init -)"' >> ~/.bashrc

# Применение изменений
source ~/.bashrc

# Установка Python 3.13.1
pyenv install 3.13.1
pyenv local 3.13.1

# Проверка
python --version
```

### 3. Установка Rust 1.91.1

```bash
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Установка конкретной версии
rustup install 1.91.1
rustup default 1.91.1

# Проверка
rustc --version
cargo --version
```

### 4. Клонирование и настройка проекта

```bash
# Клонирование репозитория
git clone --depth=1 https://github.com/arsvincere/avin.git
cd avin

# Создание виртуального окружения Python
python -m venv .venv
source .venv/bin/activate

# Установка зависимостей Python
pip install --upgrade pip
pip install --upgrade -r avin_data_py/requirements.txt
```

### 5. Настройка токена Tinkoff

```bash
# Создание директории для токена
mkdir -p ~/trading/usr/connect/tinkoff

# Создание файла с токеном (замените YOUR_TOKEN на реальный токен)
echo "YOUR_TOKEN" > ~/trading/usr/connect/tinkoff/token.txt
chmod 600 ~/trading/usr/connect/tinkoff/token.txt
```

### 6. Настройка конфигурации

```bash
# Создание директории для конфигурации
mkdir -p ~/.config/avin

# Копирование шаблона
cp res/config.toml ~/.config/avin/config.toml

# Редактирование конфигурации (укажите правильные пути)
nano ~/.config/avin/config.toml
```

**Важно:** В `config.toml` укажите правильные пути:
- `tinkoff_token = "/home/YOUR_USER/trading/usr/connect/tinkoff/token.txt"`
- `root = "/home/YOUR_USER/trading"`
- `data = "/home/YOUR_USER/trading/usr/data"`

### 7. Сборка проекта

```bash
# Активация виртуального окружения (если еще не активировано)
source .venv/bin/activate

# Сборка в release режиме
make release
# или
cargo build --release
```

### 8. Загрузка данных

```bash
# Загрузка минутных данных для одного инструмента
avin-data download MOEX_SHARE_SBER TINKOFF BAR_1M

# Конвертация в другие таймфреймы
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_5M
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_10M
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_15M
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_1H
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_4H
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_DAY
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_WEEK
avin-data convert MOEX_SHARE_SBER TINKOFF BAR_1M BAR_MONTH
```

### 9. Настройка данных для GUI (опционально)

```bash
# Создание файла списка инструментов
mkdir -p ~/trading/usr/assets
cat > ~/trading/usr/assets/xxx.csv << EOF
MOEX;SHARE;SBER;
MOEX;SHARE;GAZP;
MOEX;SHARE;LKOH;
EOF

# Копирование данных для GUI
cp -r ~/trading/usr/data/TINKOFF ~/trading/usr/data/MOEXALGO
```

### 10. Запуск приложений

```bash
# Активация виртуального окружения
source .venv/bin/activate

# Запуск торгового терминала
make terminal
# или
cargo run --release --bin avin-terminal

# Другие приложения
make data      # Загрузка данных
make trader    # Запуск трейдера
make tester    # Бэк-тестирование
make scanner   # Сканер инструментов
```

---

## Массовая конвертация данных

Для конвертации данных для нескольких инструментов:

```bash
# Создание файла со списком инструментов
mkdir -p ~/trading/usr/sh
cat > ~/trading/usr/sh/iid.txt << EOF
MOEX_SHARE_AFKS
MOEX_SHARE_AFLT
MOEX_SHARE_SBER
MOEX_SHARE_GAZP
MOEX_SHARE_LKOH
EOF

# Скрипт для массовой конвертации
IID="$HOME/trading/usr/sh/iid.txt"

for iid in $(cat $IID)
do
    echo "Конвертация данных для $iid..."
    avin-data convert $iid TINKOFF BAR_1M BAR_5M
    avin-data convert $iid TINKOFF BAR_1M BAR_10M
    avin-data convert $iid TINKOFF BAR_1M BAR_15M
    avin-data convert $iid TINKOFF BAR_1M BAR_1H
    avin-data convert $iid TINKOFF BAR_1M BAR_4H
    avin-data convert $iid TINKOFF BAR_1M BAR_DAY
    avin-data convert $iid TINKOFF BAR_1M BAR_WEEK
    avin-data convert $iid TINKOFF BAR_1M BAR_MONTH
done
```

---

## Проверка готовности

```bash
# Проверка версий
python --version  # должно быть 3.13.1
rustc --version    # должно быть 1.91.1

# Проверка виртуального окружения
which python  # должно указывать на .venv/bin/python

# Проверка токена
ls -la ~/trading/usr/connect/tinkoff/token.txt

# Проверка данных
ls -la ~/trading/usr/data/TINKOFF/

# Проверка сборки
cargo build --release
```

---

## Устранение неполадок

### Ошибка: "linker `cc` not found"
```bash
sudo apt-get install build-essential
```

### Ошибка: "OpenSSL not found"
```bash
sudo apt-get install libssl-dev
```

### Ошибка: "Permission denied"
```bash
chmod +x ~/trading/usr/connect/tinkoff/token.txt
```

### Ошибка: "Token not found"
Проверьте путь к токену в `~/.config/avin/config.toml` и убедитесь, что файл существует.

### Ошибка: "Data directory not found"
```bash
mkdir -p ~/trading/usr/data
```

---

## Обновление системы

```bash
cd ~/avin
git pull origin main
source .venv/bin/activate
make release
```

---

## Дополнительные ресурсы

- [Документация Rust](https://doc.rust-lang.org/)
- [Документация Tauri](https://tauri.app/)
- [Prerequisites for Tauri v2](https://v2.tauri.app/start/prerequisites/)
