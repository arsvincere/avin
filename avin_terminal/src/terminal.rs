#![allow(unused_imports)]

use iced::{
    Theme,
    widget::{self, column},
};

use avin_core::{Asset, AssetList};
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

const FONT: u16 = 11;

// column width
const TICKER: u16 = 40;
const ACTIVE: u16 = 15;
const DAY: u16 = 30;

fn create_header() -> iced::widget::Row<'static, Message> {
    widget::row![
        widget::text("Ticker").size(FONT).width(TICKER),
        widget::text("A").size(FONT).width(ACTIVE),
        widget::text("Day").size(FONT).width(DAY),
        widget::text("1M-1").size(FONT),
        widget::text("1M-2").size(FONT),
        widget::text("1M-3").size(FONT),
        widget::text("1M-4").size(FONT),
        widget::text("1M-5").size(FONT).width(50),
        widget::text("10M-1").size(FONT),
        widget::text("10M-2").size(FONT),
        widget::text("10M-3").size(FONT),
        widget::text("10M-4").size(FONT),
        widget::text("10M-5").size(FONT).width(50),
        widget::text("1H-1").size(FONT),
        widget::text("1H-2").size(FONT),
        widget::text("1H-3").size(FONT),
        widget::text("1H-4").size(FONT),
        widget::text("1H-5").size(FONT).width(50),
        widget::text("D-1").size(FONT),
        widget::text("D-2").size(FONT),
        widget::text("D-3").size(FONT),
    ]
    .spacing(10)
}
fn create_asset_row(asset: &Asset) -> iced::widget::Row<'static, Message> {
    widget::row![
        widget::text(asset.ticker().clone())
            .size(FONT)
            .width(TICKER),
        widget::checkbox("", false).size(10).text_size(FONT),
    ]
    .spacing(10)
}
