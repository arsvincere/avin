#![allow(unused_imports)]

use iced::{Theme, widget};

use avin_terminal::Terminal;

fn main() -> Result<(), iced::Error> {
    iced::application("AVIN - Terminal", Terminal::update, Terminal::view)
        .theme(Terminal::theme)
        .run()
}
