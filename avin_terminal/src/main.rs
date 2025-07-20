#![allow(unused_imports)]

use avin_core::{Asset, AssetList};
use avin_utils::CFG;
use iced::{Theme, widget};

fn main() -> Result<(), iced::Error> {
    iced::application("AVIN - Terminal", Terminal::update, Terminal::view)
        .theme(Terminal::theme)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    // AssetChanget(Asset),
}
struct Terminal {
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
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Message::IncrementCount => self.count += 1,
            // Message::DecrementCount => self.count -= 1,
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        let header = create_header();
        widget::container(header)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .into()
    }
    fn theme(&self) -> Theme {
        Theme::KanagawaDragon
    }
}

fn create_header() -> iced::widget::Row<'static, Message> {
    widget::row![
        widget::text("Ticker"),
        widget::text("A"),
        widget::text("Day"),
        widget::text("1M-1"),
        widget::text("1M-2"),
        widget::text("1M-3"),
        widget::text("1M-4"),
        widget::text("1M-5"),
    ]
    .spacing(10)
}
