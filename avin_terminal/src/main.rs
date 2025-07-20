// #![allow(unused)]

use iced::widget::{
    button, checkbox, column, container, responsive, scrollable, text,
};
use iced::{Element, Length, Renderer, Task, Theme};
use iced_table::table;

use avin_core::AssetList;
use avin_utils::CFG;

fn main() {
    iced::application(Terminal::title, Terminal::update, Terminal::view)
        .theme(Terminal::theme)
        .run()
        .unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    Active(usize, bool),

    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
}

struct Terminal {
    asset_list: AssetList,

    columns: Vec<Column>,
    rows: Vec<Row>,
    header: scrollable::Id,
    body: scrollable::Id,
    resize_columns_enabled: bool,
    max_width_enabled: bool,
    theme: Theme,
}
impl Default for Terminal {
    fn default() -> Self {
        // load asset list
        let asset_list =
            AssetList::load_name(&CFG.core.default_asset_list).unwrap();

        // read asset list
        let mut rows = Vec::new();
        for (n, asset) in asset_list.assets().iter().enumerate() {
            let row = Row::new(n, asset.ticker());
            rows.push(row);
        }

        Self {
            asset_list,

            columns: vec![
                Column::new(ColumnKind::Ticker),
                Column::new(ColumnKind::Active),
                Column::new(ColumnKind::Day),
            ],
            rows,
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            resize_columns_enabled: true,
            max_width_enabled: false,
            theme: Theme::KanagawaDragon,
        }
    }
}
impl Terminal {
    fn title(&self) -> String {
        "AVIN - Terminal".into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Active(index, is_enabled) => {
                if let Some(row) = self.rows.get_mut(index) {
                    row.is_enabled = is_enabled;
                }
            }

            Message::SyncHeader(offset) => {
                return Task::batch(vec![scrollable::scroll_to(
                    self.header.clone(),
                    offset,
                )]);
            }
            Message::Resizing(index, offset) => {
                if let Some(column) = self.columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
            }
            Message::Resized => self.columns.iter_mut().for_each(|column| {
                if let Some(offset) = column.resize_offset.take() {
                    column.width += offset;
                }
            }),
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let load_btn = button(text("Load"));

        let table = responsive(|size| {
            let mut table = table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                Message::SyncHeader,
            );

            if self.resize_columns_enabled {
                table = table
                    .on_column_resize(Message::Resizing, Message::Resized);
            }
            if self.max_width_enabled {
                table = table.min_width(size.width);
            }

            table.into()
        });

        let content = column![load_btn, table].spacing(4);
        container(container(content).width(Length::Fill).height(Length::Fill))
            .padding(4)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

enum ColumnKind {
    Ticker,
    Active,
    Day,
}
struct Column {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}
impl Column {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Ticker => 60.0,
            ColumnKind::Active => 20.0,
            ColumnKind::Day => 60.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

struct Row {
    _index: usize,
    ticker: String,
    is_enabled: bool,
}
impl Row {
    fn new(_index: usize, ticker: &str) -> Self {
        Self {
            _index,
            ticker: ticker.to_string(),
            is_enabled: false,
        }
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for Column {
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Ticker => "Ticker",
            ColumnKind::Active => "A",
            ColumnKind::Day => "D",
        };

        container(text(content)).center_y(24).into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a Row,
    ) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::Ticker => text(&row.ticker).into(),
            ColumnKind::Active => checkbox("", row.is_enabled)
                .on_toggle(move |active| Message::Active(row_index, active))
                .into(),
            ColumnKind::Day => text("10.22").into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
