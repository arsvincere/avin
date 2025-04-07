use iced::widget::{
    Column, button, column, container, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Length, Padding, Theme};

fn main() -> iced::Result {
    iced::application("AVIN - Terminal", Terminal::update, Terminal::view)
        .theme(Terminal::theme)
        .centered()
        .run()
}

struct Terminal {
    // asset: impl Asset,
    grocery_items: Vec<String>,
    input_value: String,
}

impl Default for Terminal {
    fn default() -> Self {
        todo!();
        // let asset = Asset::from_str("moex_share_sber").unwrap();
        // Self {
        //     // asset,
        //     grocery_items: vec![
        //         "Eggs".to_owned(),
        //         "Milk".to_owned(),
        //         "Flour".to_owned(),
        //     ],
        //     input_value: String::default(),
        // }
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputValue(String),
    Submitted,
    DeleteItem(usize),
}

impl Terminal {
    fn view(&self) -> Element<Message> {
        container(
            column!(
                Self::items_list_view(&self.grocery_items),
                row!(
                    text_input("Input grocery item", &self.input_value)
                        .on_input(Message::InputValue)
                        .on_submit(Message::Submitted),
                    button("Submit").on_press(Message::Submitted),
                )
                .spacing(30)
                .padding(Padding::from(30)),
            )
            .align_x(Alignment::Center),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputValue(value) => self.input_value = value,
            Message::Submitted => {
                self.grocery_items.push(self.input_value.clone());
                self.input_value = String::default();
            }
            Message::DeleteItem(item) => {
                self.grocery_items.remove(item);
            }
        }
    }

    fn theme(&self) -> Theme {
        Theme::KanagawaDragon
    }

    fn items_list_view(items: &[String]) -> Element<Message> {
        let mut column = Column::new()
            .spacing(20)
            .align_x(Alignment::Center)
            .width(Length::Fill);

        for (index, item) in items.iter().enumerate() {
            column = column.push(Self::grocery_item(index, item));
        }

        scrollable(container(column))
            .width(250.0)
            .height(300)
            .into()
    }

    fn grocery_item(index: usize, value: &str) -> Element<Message> {
        row![
            text(value),
            button("Delete").on_press(Message::DeleteItem(index))
        ]
        .align_y(Alignment::Center)
        .spacing(30)
        .into()
    }
}
