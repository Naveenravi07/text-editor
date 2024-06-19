use iced::{
    widget::{column, container, horizontal_space, row, text, text_editor},
    Sandbox, Settings, Theme,
};

#[derive(Debug, Clone)]
enum Messages {
    Edit(text_editor::Action),
}

struct Texteditor {
    content: text_editor::Content,
}

impl Sandbox for Texteditor {
    type Message = Messages;

    fn title(&self) -> String {
        String::from("Hed")
    }

    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }

    fn new() -> Self {
        Texteditor {
            content: text_editor::Content::with_text(include_str!("main.rs")),
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Messages::Edit(action) => self.content.perform(action),
        };
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let editor = text_editor(&self.content)
            .on_action(Messages::Edit)
            .height(800);

        let pos = {
            let (line, col) = self.content.cursor_position();
            let formatted = format!("{}:{}",line,col);
            text(formatted)
        };
    
        let status_bar = row![horizontal_space(),pos];
        container(column![editor, status_bar]).padding(10).into()
    }
}

fn main() -> iced::Result {
    Texteditor::run(Settings::default())
}
