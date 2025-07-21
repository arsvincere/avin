/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused_imports)]

use iced::{Theme, widget};

use avin_terminal::Terminal;
use avin_utils as utils;

fn main() -> Result<(), iced::Error> {
    utils::init_logger();
    iced::application("AVIN - Terminal", Terminal::update, Terminal::view)
        .theme(Terminal::theme)
        .run()
}
