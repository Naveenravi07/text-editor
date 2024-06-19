use iced::{
    executor,
    widget::{column, container, horizontal_space, row, text, text_editor},
    Application, Command, Settings, Theme,
};
use std::{env, io, path::Path, sync::Arc};

#[derive(Debug, Clone)]
enum Messages {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, io::ErrorKind>),
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
                content: text_editor::Content::new(),
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Messages::FileOpened,
            ),
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
            Messages::FileOpened(result) => match result {
                Ok(content) => {
                    self.content = text_editor::Content::with_text(&content);
                }
                Err(kind) => {
                    println!("ERR : {}", kind);
                }
            },
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

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(&path)
        .await
        .map(|string| Arc::new(string))
        .map_err(|err| err.kind())
}

fn main() -> iced::Result {
    Texteditor::run(Settings::default())
}
