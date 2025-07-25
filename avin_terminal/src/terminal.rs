/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused_imports)]

use iced::{
    Theme,
    widget::{self, column},
};

use avin_analyse::TrendAnalytic;
use avin_connect::Tinkoff;
use avin_core::{
    Action, Asset, AssetList, DataAction, Event, ExtremumIndicator, Term,
    TimeFrame,
};
use avin_utils::CFG;

use super::message::Message;

type EventSender = tokio::sync::mpsc::UnboundedSender<Event>;
type EventReceiver = tokio::sync::mpsc::UnboundedReceiver<Event>;
type ActionSender = tokio::sync::mpsc::UnboundedSender<Action>;

pub struct Terminal {
    #[allow(dead_code)]
    asset_list: AssetList,

    is_connect: bool,
    broker_tx: Option<ActionSender>,
    event_tx: EventSender,
    _event_rx: EventReceiver,
}
impl Default for Terminal {
    fn default() -> Self {
        // channel for maker events
        let (event_tx, _event_rx) = tokio::sync::mpsc::unbounded_channel();

        // load asset list
        let name = &CFG.core.default_asset_list;
        let asset_list = AssetList::load_name(name).unwrap();

        Self {
            asset_list,

            is_connect: false,
            broker_tx: None,
            event_tx,
            _event_rx,
        }
    }
}
impl Terminal {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Connect(toggler) => {
                self.is_connect = toggler;
                if self.is_connect {
                    self.connect_broker();
                } else {
                    self.disconnect_broker();
                };
            }
        }

        iced::Task::none()
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let connect_toggler = create_connect_toggler(self.is_connect);
        let header = create_header();

        let mut content = column![connect_toggler, header].spacing(4);
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

    // private
    fn connect_broker(&mut self) {
        let broker = Tinkoff::new(self.event_tx.clone());
        self.broker_tx = Some(broker.get_sender());

        let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        // start tokio main loop, broker in there
        tokio_runtime.block_on(async {
            start_broker(broker).await;
        });

        // subscribe market data for all asset list
        for asset in self.asset_list.assets().iter() {
            let iid = asset.iid();
            let mut market_data = Vec::new();
            for tf in TimeFrame::all() {
                // collect market data types
                let md = tf.market_data();
                market_data.push(md);
            }

            // create action
            let action =
                Action::Subscribe(DataAction::new(iid.clone(), market_data));

            self.broker_tx.as_ref().unwrap().send(action).unwrap();
        }
    }
    fn disconnect_broker(&mut self) {
        self.broker_tx = None;
    }
}

fn create_connect_toggler(
    state: bool,
) -> iced::widget::Toggler<'static, Message> {
    widget::toggler(state)
        .label("Connect")
        .size(10)
        .text_size(HEADER)
        .on_toggle(Message::Connect)
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

    // // active col
    // let active = widget::checkbox("", false)
    //     .size(CHECK)
    //     .text_size(FONT)

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
        // active,
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

async fn start_broker(mut broker: Tinkoff) {
    broker.connect().await.unwrap();
    log::debug!(":: Broker connected!");

    broker.create_marketdata_stream().await.unwrap();
    log::debug!(":: Data stream started!");

    broker.create_transactions_stream().await.unwrap();
    log::debug!(":: Transaction stream started!");

    tokio::spawn(async move {
        broker.start().await;
    });
    log::debug!(":: Broker started!");
}

// sizes
const HEADER: u16 = 12; // header font size
const FONT: u16 = 11; // font size
// const CHECK: u16 = 10; // check box size

// column width
const TICKER: u16 = 40;
const ACTIVE: u16 = 15;
const DAY: u16 = 32;
const TFT__: u16 = 40;
const TFT_L: u16 = 50;
