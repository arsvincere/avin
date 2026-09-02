# Navigation

`core.md` — overview types of crate avin_core;

# Overview

## Core

Атомарные объекты: Time, Price, Quantity, DataProvider...
Небольшие составные объекты: TimeRange, PriceRange...

Фундаментальный общий словарь AVIN, на котором говорят все модули системы. В отличии от доменных (трейдерских) объектов нужны так же служебным модулям:
* `avin_system`
* `avin_storage`
* `avin_data`
* `avin_connect`
* `cli`

## Domain

Market model - модель предметной области трейдинга.

Опирается не объекты `avin_core`, больше ни от чего не зависит.
