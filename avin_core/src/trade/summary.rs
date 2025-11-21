/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

// TODO: идея
// 1. можно посмотреть общее время теста - 100500 часов например
// посмотреть суммарную таймдельту по всем трейдам - 100ч например
// суммарное время в позиции (между открытием и закрытием позиции)
// посчитать процент который стратегия активна - activity - допустим 10%
// 2. посчитать среднюю скорость трейда - avg_speed допустим 1% в день.
// ну можно еще max_speed min_speed. То есть берем результат трейда
// допустим входим на 100 рублей, закрываем по 101рублю. Профит 1%.
// Делим профит в процентах на количество времени в позиции = скорость трейда.
// Далее находим среднюю скорость трейдов.
// 3. Тогда можно посчитать activity * avg_speed = E (эффективность)
// E = это будет общая эффективность стратегии. То есть вот запустил ее
// на 100 дней = 10 дней она активна, и по 1% в день в это время
// делает. Тогда ее профит итоговый равен 10%.
// Или тоже самое 100 дней * activity * avg_speed =
// 100 * 0.1 * 0.01 * 100% = 10%
// окей... а вот если не умножать на количество дней а просто умножить
// activity * avg_speed = 0.1 * 0.01 * 100% = 0.1%
// То есть эффективность такой стратегии = 0.1% в день!
// По этому параметру можно будет сравнивать эффективность стратегий.
// И в зависимости от эффективности стратегии распределять депо между
// ними.

use crate::{Trade, TradeList};
use avin_utils::round;
use polars::prelude::*;

#[derive(Debug)]
pub struct Summary {
    /// Имя отчета == имя трейд листа.
    pub name: String,
    /// Чистая прибыль всех трейдов.
    pub profit: f64,
    /// Процент прибыльности.
    pub percent_profitable: f64,
    /// Количество трейдов.
    pub total_trades: u32,
    /// Количество прибыльных трейдов.
    pub win_trades: u32,
    /// Количество убыточных трейдов.
    pub loss_trades: u32,
    /// Отношение общей прибыли к общему убытку.
    pub ratio: f64,
    /// Математическое ожидание трейда.
    pub average_trade: f64,
    /// Максимальное количество последовательных выигрышей.
    pub win_seq: u32,
    /// Максимальное количество последовательных проигрышей.
    pub loss_seq: u32,
    /// Средний выигрыш.
    pub avg_win: f64,
    /// Средний проигрыш.
    pub avg_loss: f64,
    /// Максимальный выигрыш.
    pub max_win: f64,
    /// Максимальный проигрыш.
    pub max_loss: f64,
    /// Суммарная прибыль всех трейдов.
    pub gross_profit: f64,
    /// Суммарный убыток всех трейдов.
    pub gross_loss: f64,
}
impl Summary {
    // build
    pub fn new(trade_list: &TradeList) -> Self {
        // get results of trades
        let mut results = Vec::new();
        for i in trade_list.trades() {
            if let Trade::Closed(trade) = i {
                let r = trade.result();
                results.push(r);
            }
        }

        Self {
            name: trade_list.name().clone(),
            total_trades: total_trades(&results),
            win_trades: winning_trades(&results),
            loss_trades: losing_trades(&results),
            gross_profit: gross_profit(&results),
            gross_loss: gross_loss(&results),
            profit: total_net_profit(&results),
            ratio: ratio(&results),
            percent_profitable: percent_profitable(&results),
            average_trade: average_trade(&results),
            avg_win: average_win(&results),
            avg_loss: average_loss(&results),
            max_win: largest_win(&results),
            max_loss: largest_loss(&results),
            win_seq: max_win_series(&results),
            loss_seq: max_loss_series(&results),
        }
    }
}
impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trades = format!(
            "{}/{}/{}",
            self.total_trades, self.win_trades, self.loss_trades
        );
        let gross = format!(
            "{} / {}",
            round(self.gross_profit, 2),
            round(self.gross_loss, 2)
        );

        let table = df!(
            "Name" => [self.name.clone()],
            "Profit" => [round(self.profit, 2)],
            "%" => [round(self.percent_profitable, 2)],
            "Trades" => [trades],
            "Ratio" => [round(self.ratio, 2)],
            "Avg" => [round(self.average_trade, 2)],
            "Gross profit/loss" => [gross],
        )
        .unwrap();

        write!(f, "{table}")
    }
}

fn total_trades(results: &[f64]) -> u32 {
    results.len() as u32
}
fn winning_trades(results: &[f64]) -> u32 {
    let mut count = 0;
    for i in results.iter() {
        if *i > 0.0 {
            count += 1;
        }
    }

    count
}
fn losing_trades(results: &[f64]) -> u32 {
    let mut count = 0;
    for i in results.iter() {
        if *i < 0.0 {
            count += 1;
        }
    }

    count
}
fn gross_profit(results: &[f64]) -> f64 {
    let mut value = 0.0;
    for i in results.iter() {
        if *i > 0.0 {
            value += i;
        }
    }

    value
}
fn gross_loss(results: &[f64]) -> f64 {
    let mut value = 0.0;
    for i in results.iter() {
        if *i < 0.0 {
            value += i;
        }
    }

    value
}
fn total_net_profit(results: &[f64]) -> f64 {
    let mut value = 0.0;
    for i in results {
        value += i;
    }

    value
}
fn ratio(results: &[f64]) -> f64 {
    let loss = gross_loss(results).abs();

    if loss == 0.0 {
        100.0
    } else {
        gross_profit(results) / loss
    }
}
fn percent_profitable(results: &[f64]) -> f64 {
    let win = winning_trades(results) as f64;
    let total = total_trades(results) as f64;

    if total == 0.0 {
        0.0
    } else {
        win / total * 100.0
    }
}
fn largest_win(results: &[f64]) -> f64 {
    let mut max_win = 0.0;
    for i in results.iter() {
        if *i > max_win {
            max_win = *i;
        }
    }

    max_win
}
fn largest_loss(results: &[f64]) -> f64 {
    let mut max_loss = 0.0;
    for i in results.iter() {
        if *i < max_loss {
            max_loss = *i;
        }
    }

    max_loss
}
fn average_win(results: &[f64]) -> f64 {
    let win_count = winning_trades(results);

    if win_count == 0 {
        0.0
    } else {
        gross_profit(results) / win_count as f64
    }
}
fn average_loss(results: &[f64]) -> f64 {
    let loss_count = losing_trades(results);

    if loss_count == 0 {
        0.0
    } else {
        gross_loss(results) / loss_count as f64
    }
}
fn average_trade(results: &[f64]) -> f64 {
    let count = total_trades(results);

    if count == 0 {
        0.0
    } else {
        total_net_profit(results) / count as f64
    }
}
fn max_win_series(results: &[f64]) -> u32 {
    let mut max_series = 0;
    let mut series = 0;

    for i in results.iter() {
        if *i >= 0.0 {
            series += 1;
        } else {
            series = 0;
        }

        max_series = avin_utils::max(max_series, series);
    }

    max_series
}
fn max_loss_series(results: &[f64]) -> u32 {
    let mut max_series = 0;
    let mut series = 0;

    for i in results.iter() {
        if *i < 0.0 {
            series += 1;
        } else {
            series = 0;
        }

        max_series = avin_utils::max(max_series, series);
    }

    max_series
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics() {
        let results = [10.0, 11.0, -1.0];

        assert_eq!(total_trades(&results), 3);
        assert_eq!(winning_trades(&results), 2);
        assert_eq!(losing_trades(&results), 1);
        assert_eq!(gross_profit(&results), 21.0);
        assert_eq!(gross_loss(&results), -1.0);
        assert_eq!(total_net_profit(&results), 20.0);
        assert_eq!(ratio(&results), 21.0);
        assert_eq!(percent_profitable(&results), 66.66666666666666);
        assert_eq!(largest_win(&results), 11.0);
        assert_eq!(largest_loss(&results), -1.0);
        assert_eq!(average_win(&results), 10.5);
        assert_eq!(average_loss(&results), -1.0);
        assert_eq!(average_trade(&results), 20.0 / 3.0);
        assert_eq!(max_win_series(&results), 2);
        assert_eq!(max_loss_series(&results), 1);
    }
}
