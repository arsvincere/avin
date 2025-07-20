#![allow(unused_imports)]

use avin_analyse::TrendAnalytic;
use iced::{
    Theme,
    widget::{self, column},
};

use avin_core::{Asset, AssetList, ExtremumIndicator, Term, TimeFrame};
use avin_utils::CFG;

use super::message::Message;

pub struct Terminal {
    #[allow(dead_code)]
    asset_list: AssetList,
}
impl Default for Terminal {
    fn default() -> Self {
        // load asset list
        let name = &CFG.core.default_asset_list;
        let asset_list = AssetList::load_name(name).unwrap();

        Self { asset_list }
    }
}
impl Terminal {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Message::IncrementCount => self.count += 1,
            // Message::DecrementCount => self.count -= 1,
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let header = create_header();

        let mut content = column![header].spacing(4);
        for asset in self.asset_list.assets().iter() {
            let row = create_asset_row(asset);
            content = content.push(row);
        }

        widget::container(content)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .into()
    }
    pub fn theme(&self) -> Theme {
        Theme::KanagawaDragon
    }
}

fn create_header() -> iced::widget::Row<'static, Message> {
    widget::row![
        widget::text("Ticker").size(FONT).width(TICKER),
        widget::text("A").size(FONT).width(ACTIVE),
        widget::text("Day").size(FONT).width(DAY),
        widget::text("1M-1").size(FONT).width(TFT__),
        widget::text("1M-2").size(FONT).width(TFT__),
        widget::text("1M-3").size(FONT).width(TFT__),
        widget::text("1M-4").size(FONT).width(TFT__),
        widget::text("1M-5").size(FONT).width(TFT_L),
        widget::text("10M-1").size(FONT).width(TFT__),
        widget::text("10M-2").size(FONT).width(TFT__),
        widget::text("10M-3").size(FONT).width(TFT__),
        widget::text("10M-4").size(FONT).width(TFT__),
        widget::text("10M-5").size(FONT).width(TFT_L),
        widget::text("1H-1").size(FONT).width(TFT__),
        widget::text("1H-2").size(FONT).width(TFT__),
        widget::text("1H-3").size(FONT).width(TFT__),
        widget::text("1H-4").size(FONT).width(TFT__),
        widget::text("1H-5").size(FONT).width(TFT_L),
        widget::text("D-1").size(FONT).width(TFT__),
        widget::text("D-2").size(FONT).width(TFT__),
        widget::text("D-3").size(FONT).width(TFT_L),
    ]
    .spacing(10)
}
fn create_asset_row(asset: &Asset) -> iced::widget::Row<'static, Message> {
    let bear_color = iced::Color::parse(&CFG.gui.color.bear).unwrap();
    let bull_color = iced::Color::parse(&CFG.gui.color.bull).unwrap();

    // ticker col
    let ticker = widget::text(asset.ticker().clone())
        .size(FONT)
        .width(TICKER);

    // active col
    let active = widget::checkbox("", false).size(CHECK).text_size(FONT);

    // delta day col
    let delta_day = asset.delta_day().unwrap_or(0.0);
    let delta = if delta_day < 0.0 {
        widget::text(delta_day.abs())
            .size(FONT)
            .width(DAY)
            .color(bear_color)
    } else if delta_day > 0.0 {
        widget::text(delta_day)
            .size(FONT)
            .width(DAY)
            .color(bull_color)
    } else {
        widget::text(delta_day).size(FONT).width(DAY)
    };

    // posterior 1m T1
    let posterior_1m_t1 = || {
        let term = Term::T1;
        if let Some(chart) = asset.chart(TimeFrame::M1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1m T2
    let posterior_1m_t2 = || {
        let term = Term::T2;
        if let Some(chart) = asset.chart(TimeFrame::M1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1m T3
    let posterior_1m_t3 = || {
        let term = Term::T3;
        if let Some(chart) = asset.chart(TimeFrame::M1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1m T4
    let posterior_1m_t4 = || {
        let term = Term::T4;
        if let Some(chart) = asset.chart(TimeFrame::M1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1m T5
    let posterior_1m_t5 = || {
        let term = Term::T5;
        if let Some(chart) = asset.chart(TimeFrame::M1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT_L)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT_L)
    };

    // posterior 10m T1
    let posterior_10m_t1 = || {
        let term = Term::T1;
        if let Some(chart) = asset.chart(TimeFrame::M10) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 10m T2
    let posterior_10m_t2 = || {
        let term = Term::T2;
        if let Some(chart) = asset.chart(TimeFrame::M10) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 10m T3
    let posterior_10m_t3 = || {
        let term = Term::T3;
        if let Some(chart) = asset.chart(TimeFrame::M10) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 10m T4
    let posterior_10m_t4 = || {
        let term = Term::T4;
        if let Some(chart) = asset.chart(TimeFrame::M10) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 10m T5
    let posterior_10m_t5 = || {
        let term = Term::T5;
        if let Some(chart) = asset.chart(TimeFrame::M10) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT_L)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT_L)
    };

    // posterior 1h T1
    let posterior_1h_t1 = || {
        let term = Term::T1;
        if let Some(chart) = asset.chart(TimeFrame::H1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1h T2
    let posterior_1h_t2 = || {
        let term = Term::T2;
        if let Some(chart) = asset.chart(TimeFrame::H1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1h T3
    let posterior_1h_t3 = || {
        let term = Term::T3;
        if let Some(chart) = asset.chart(TimeFrame::H1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1h T4
    let posterior_1h_t4 = || {
        let term = Term::T4;
        if let Some(chart) = asset.chart(TimeFrame::H1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior 1h T5
    let posterior_1h_t5 = || {
        let term = Term::T5;
        if let Some(chart) = asset.chart(TimeFrame::H1) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT_L)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT_L)
    };

    // posterior d T1
    let posterior_d_t1 = || {
        let term = Term::T1;
        if let Some(chart) = asset.chart(TimeFrame::Day) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior d T2
    let posterior_d_t2 = || {
        let term = Term::T2;
        if let Some(chart) = asset.chart(TimeFrame::Day) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // posterior d T3
    let posterior_d_t3 = || {
        let term = Term::T3;
        if let Some(chart) = asset.chart(TimeFrame::Day) {
            if let Some(trend) = chart.trend(term, 0) {
                if let Some(probability) = chart.trend_posterior(term) {
                    let color = if trend.is_bear() {
                        bear_color
                    } else {
                        bull_color
                    };
                    return widget::text(probability)
                        .size(FONT)
                        .width(TFT__)
                        .color(color);
                }
            }
        }
        // else
        widget::text(0.0).size(FONT).width(TFT__)
    };
    // all row
    widget::row![
        ticker,
        active,
        delta,
        posterior_1m_t1(),
        posterior_1m_t2(),
        posterior_1m_t3(),
        posterior_1m_t4(),
        posterior_1m_t5(),
        posterior_10m_t1(),
        posterior_10m_t2(),
        posterior_10m_t3(),
        posterior_10m_t4(),
        posterior_10m_t5(),
        posterior_1h_t1(),
        posterior_1h_t2(),
        posterior_1h_t3(),
        posterior_1h_t4(),
        posterior_1h_t5(),
        posterior_d_t1(),
        posterior_d_t2(),
        posterior_d_t3(),
    ]
    .spacing(10)
}

// sizes
const FONT: u16 = 11; // font size
const CHECK: u16 = 10; // check box size

// column width
const TICKER: u16 = 40;
const ACTIVE: u16 = 15;
const DAY: u16 = 32;
const TFT__: u16 = 32;
const TFT_L: u16 = 50;
