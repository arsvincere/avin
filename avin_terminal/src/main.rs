use avin_core::AssetList;
use avin_utils::CFG;
use iced::{Theme, widget};

fn main() -> Result<(), iced::Error> {
    iced::application("AVIN - Terminal", Terminal::update, Terminal::view)
        .theme(Terminal::theme)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    //
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
    fn theme(&self) -> Theme {
        Theme::KanagawaDragon
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        // handle emitted messages
        match message {
            // Message::IncrementCount => self.count += 1,
            // Message::DecrementCount => self.count -= 1,
        }
        // iced::Task::none()
    }
    fn view(&self) -> iced::Element<'_, Message> {
        // create the View Logic (UI)
        let header = widget::row![
            widget::text("Ticker"),
            widget::text("A"),
            widget::text("Day"),
        ]
        .spacing(10);
        widget::container(header)
            // .center_x(iced::Length::Fill)
            // .center_y(iced::Length::Fill)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .into()
    }
}
