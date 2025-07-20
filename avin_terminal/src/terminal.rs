#![allow(unused_imports)]

use iced::{Theme, widget};

use avin_core::{Asset, AssetList};
use avin_utils::CFG;

use super::message::Message;

pub struct Terminal {
    #[allow(dead_code)]
    asset_list: AssetList,
}
impl Default for Terminal {
    fn default() -> Self {
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
        widget::container(header)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .into()
    }
    pub fn theme(&self) -> Theme {
        Theme::KanagawaDragon
    }
}

fn create_header() -> iced::widget::Row<'static, Message> {
    let sz = 14;
    widget::row![
        widget::text("Ticker").size(sz),
        widget::text("A").size(sz),
        widget::text("Day").size(sz),
        widget::text("1M-1").size(sz).width(100),
        widget::text("1M-2").size(sz),
        widget::text("1M-3").size(sz),
        widget::text("1M-4").size(sz),
        widget::text("1M-5").size(sz),
    ]
    .spacing(10)
}
