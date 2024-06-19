use iced::{
    executor,
    widget::{column, container, horizontal_space, row, text, text_editor},
    Application, Command, Settings, Theme,
};

#[derive(Debug, Clone)]
enum Messages {
    Edit(text_editor::Action),
}

struct Texteditor {
    content: text_editor::Content,
}

impl Application for Texteditor {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Texteditor, Command<Self::Message>) {
        (
            Texteditor {
                content: text_editor::Content::with_text(include_str!("main.rs")),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Hed")
    }

    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::Edit(action) => self.content.perform(action),
        };
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let editor = text_editor(&self.content)
            .on_action(Messages::Edit)
            .height(800);

        let pos = {
            let (line, col) = self.content.cursor_position();
            let formatted = format!("{}:{}", line, col);
            text(formatted)
        };

        let status_bar = row![horizontal_space(), pos];
        container(column![editor, status_bar]).padding(10).into()
    }
}

fn main() -> iced::Result {
    Texteditor::run(Settings::default())
}
