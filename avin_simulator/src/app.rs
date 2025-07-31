/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused)]

use iced::{
    Alignment, Color, Font, Length, Point, Rectangle, Renderer, Theme, mouse,
    widget::{
        self, Canvas,
        canvas::{Frame, Geometry, Path, Program, Stroke},
        column, row,
    },
};

use avin_core::{Asset, AssetList};
use avin_utils::CFG;

#[derive(Debug, Clone)]
pub enum Message {}

#[derive()]
pub struct UiSimulator {
    asset_list: AssetList,
}
impl Default for UiSimulator {
    fn default() -> Self {
        let name = &CFG.core.default_asset_list;
        let asset_list = AssetList::load_name(name).unwrap();

        Self { asset_list }
    }
}
impl<Message> Program<Message> for UiSimulator {
    type State = UiSimulator;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgb(0.1, 0.1, 0.1),
        );

        frame.stroke(
            &Path::line(
                // frame.center() + Vector::new(250.0, -100.0),
                Point::new(100.0, 100.0),
                Point::new(100.0, 200.0),
            ),
            Stroke {
                style: Color::WHITE.into(),
                width: 1.0,
                ..Default::default()
            },
        );
        frame.stroke(
            &Path::line(
                // frame.center() + Vector::new(250.0, -100.0),
                Point::new(90.0, 190.0),
                Point::new(100.0, 190.0),
            ),
            Stroke {
                style: Color::WHITE.into(),
                width: 1.0,
                ..Default::default()
            },
        );
        frame.stroke(
            &Path::line(
                // frame.center() + Vector::new(250.0, -100.0),
                Point::new(100.0, 110.0),
                Point::new(110.0, 110.0),
            ),
            Stroke {
                style: Color::WHITE.into(),
                width: 1.0,
                ..Default::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
impl UiSimulator {
    pub fn update(&mut self, message: Message) {
        match message {}
    }
    pub fn view(&self) -> iced::Element<Message> {
        let asset_table = create_asset_table(&self.asset_list);

        let chart = column![
            "A Canvas",
            Canvas::new(self).width(Length::Fill).height(Length::Fill)
        ]
        .align_x(Alignment::Center);

        widget::row![asset_table, chart].into()
    }
    pub fn theme(&self) -> Theme {
        Theme::KanagawaDragon
    }
}

fn create_asset_table<'a>(
    asset_list: &'a AssetList,
) -> iced::widget::table::Table<'a, Message, Theme, Renderer> {
    let bold = |header| {
        widget::text(header)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Font::DEFAULT
            })
            .size(HEADER)
    };
    let columns = [
        widget::table::column(bold("Ticker"), |asset: &Asset| {
            widget::text(asset.ticker()).size(FONT)
        }),
        widget::table::column(bold("Name"), |asset: &Asset| {
            widget::text(asset.name()).size(FONT)
        }),
    ];

    widget::table(columns, asset_list.assets())
        .padding_x(PADDING)
        .padding_y(PADDING)
        .separator_x(SEPARATOR_X)
        .separator_y(SEPARATOR_Y)
}

const HEADER: u32 = 12; // header font size
const FONT: u32 = 11; // font size
const PADDING: u32 = 2; // font size
const SEPARATOR_X: u32 = 0;
const SEPARATOR_Y: u32 = 0;
