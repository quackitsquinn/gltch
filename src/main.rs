use iced::widget::{Column, button, column, text};

fn main() -> iced::Result {
    iced::run("the glorious counter", Counter::update, Counter::view)
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Incr,
    Decr,
}

#[derive(Default)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: 0 }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Incr => self.value += 1,
            Message::Decr => self.value -= 1,
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        column![
            button("+").on_press(Message::Incr),
            text(self.value.to_string()),
            button("-").on_press(Message::Decr),
        ]
    }

    pub fn get(&self) -> i32 {
        self.value
    }
}
