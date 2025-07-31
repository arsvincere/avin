/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused_imports)]

use iced::{Theme, widget};

use avin_simulator::UiSimulator;
use avin_utils as utils;

fn main() -> Result<(), iced::Error> {
    utils::init_logger();
    iced::application(
        UiSimulator::default,
        UiSimulator::update,
        UiSimulator::view,
    )
    .theme(UiSimulator::theme)
    .run()
}
