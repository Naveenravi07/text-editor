use iced::{widget::text, Sandbox, Settings};

#[derive(Debug, Clone, Copy)]
enum Messages {}

struct Texteditor {}

impl Sandbox for Texteditor {
    type Message = Messages;

    fn title(&self) -> String {
        String::from("Hed")
    }
    fn new() -> Self {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        match message {};
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        text("Hello world").into()
    }
}

fn main() -> iced::Result {
    Texteditor::run(Settings::default())
}
