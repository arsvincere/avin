Методы округления цены по минимальному шагу надо прикручивать именно к Price

let price = Price::new(255.23);
let min_step = 0.5;

вопрос только мутабл прайс делать или возвращать новое значение?

price.round(min_step);
или
let price = price.round(min_step)
