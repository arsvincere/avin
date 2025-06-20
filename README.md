<h1 align="center">
  <img src="https://github.com/arsvincere/avin-rs/blob/master/res/splash/splash.png" alt="AVIN - Trade System">
</h1>

<div align="center">

  [![version](https://img.shields.io/badge/version-0.1.0-blue.svg)]()
  [![size](https://img.shields.io/crates/size/avin)]()
  [![lines](https://sloc.xyz/github/arsvincere/avin-rs/?badge-bg-color=E82424&lower=true&label=lines)]()
  [![doc](https://docs.rs/avin/badge.svg)](https://docs.rs/avin/)
  [![downloads](https://img.shields.io/crates/d/avin?label=crates.io)](https://crates.io/crates/avin)

</div>

## AVIN - open source cross-platform trading system
AVIN (от лат. Ars Vincere  -  искусство побеждать)  —  это кросплатформенная
трейдинговая система, написана на Rust, с GUI на egui/eframe.

Содержит все, что нужно для алготрейдинга: от загрузки исторических данных и
разработки стратегии, до запуска в боевой режим и построения отчетов.

Интерфейсы и принципы работы стремятся быть такими же простыми как Pine от TradingView, но реализация на Rust дает возможность создавать сложные алгоритмы, и обеспечивает скорость достаточную для работы с тиковыми данными, кластерами и стаканом на grpc стриме.

## Модули и возможности

- **data:** загрузка и обновление исторических данных. Пока только с Московской
  биржи
- **core:** структуры для удобной работы с данными на "трейдерском языке":
  график, таймфрейм, ордер...
- **extra:** продвинутые абстракции: экстремум, тренд, кластер...
- **analytic:** инструменты статистического анализа исторических данных
- **tester:** простой, но очень быстрый бэк-тестер
- **trader:** модуль управления роботами. Пока доступно подключение
  только к Тинькофф брокеру (Т-банк)
- **terminal:** GUI терминал для ручной торговли
- **report**: построение отчетов
- **informer:** уведомления в telegram
- **gui:** утилиты для просмотра результатов тестов и др.

## Цели проекта

### 1. Open-source фреймворк для алготрейдера

Каждый алготрейдер, так или иначе, решает для себя задачи: получения и
обновления исторических данных, проверки торговых гипотез, коннекторы к
брокерам и логику управления торговыми стратегиями. На разработку подобных
велосипедов уходит от пары месяцев до нескольких лет, смотря как делать.

Хорошо бы в мире быть бесплатному открытому проекту, который развивается
сообществом и предоставляет такой базовый инструментарий для алготрейдера.
В идеале должен получиться этакий фреймворк для трейдера, чтобы можно было
сосредоточиться на анализе данных и разработке стратегий, а не служебном коде.

### 2. Собрать команду алготрейдеров, программистов, математиков

Я ищу единомышленников. Чтобы вместе ~~пить хеннеси и трахать телочек~~
зарабатывать на бирже.

На данном этапе я открыт к общению с каждым заинтересованным. Не зависимо
от опыта и знаний. Главное - желание развиваться в сфере алготрейдинга,
программирования, математики и машинного обучения. Узкому кругу лиц я готов
показать свою "пользовательскую" часть: наработки по анализу данных и готовые
рабочие стратегии - с целью переопыления идеями и дальнейшей совместной
разработки. Пиши.

let contact = [email](mailto:mr.alexavin@gmail.com) || [telegram](https://t.me/mr_alexavin);

## Текущий прогресс

В 2023-2024г был сделан прототип на Python, с GUI на PyQt6.
Кодовая база составила 40к строк. Стало понятно:
1. Все это и очень хорошо, и нужно, и работает. И дальше хочется развивать.
2. Python не достаточно производительности для реал-тайм работы с тиками,
   стаканом и SuperCandles от Московской биржи.

Можно было использовать С, С++ или Cython для критических участков... Так
обычно и делают. Но в марте 2025 был выбран долгий и сложный путь - переписать
все на Rust. Это даст бОльшую надежность кода и лучшую производительность.
В перспективе библиотека может использоваться и для создания HFT стратегий.

Программа на этапе активной разработки.
Пока работоспособна только "для себя", нет документации, часто меняются
интерфейсы. На сегодня (2025-05-16) переписано около 50% python кода.

![image](https://github.com/arsvincere/avin-rs/blob/master/res/screenshot/Screenshot_2024-02-28_13-11-10.png)

## Getting start

1. Clone this repository

```
git clone --depth=1 https://github.com/arsvincere/avin.git
```

