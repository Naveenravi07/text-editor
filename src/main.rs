use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, text, text_editor},
    Application, Command, Settings, Theme,
};
use rfd::AsyncFileDialog;
use std::{env, io, path::Path, sync::Arc};

#[derive(Clone, Debug)]
enum Error {
    IO(io::ErrorKind),
    DialogClosed,
}

#[derive(Debug, Clone)]
enum Messages {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, Error>),
    Open,
}

struct Texteditor {
    content: text_editor::Content,
    error: Option<Error>,
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
                error: None,
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

    fn update(&mut self, message: Self::Message) -> Command<Messages> {
        match message {
            Messages::Edit(action) => {
                self.content.perform(action);
                Command::none()
            }

            Messages::FileOpened(result) => {
                match result {
                    Ok(content) => {
                        self.content = text_editor::Content::with_text(&content);
                    }
                    Err(kind) => {
                        self.error = Some(kind);
                    }
                };
                Command::none()
            }

            Messages::Open => Command::perform(pick_file(), Messages::FileOpened),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let controls = row![button("Open").on_press(Messages::Open)];
        let editor = text_editor(&self.content)
            .on_action(Messages::Edit)
            .height(800);

        let pos = {
            let (line, col) = self.content.cursor_position();
            let formatted = format!("{}:{}", line, col);
            text(formatted)
        };

        let status_bar = row![horizontal_space(), pos];
        container(column![controls, editor, status_bar])
            .padding(10)
            .into()
    }
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = AsyncFileDialog::new()
        .set_title("Choose a file to open")
        .set_directory("/")
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path()).await
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(&path)
        .await
        .map(|string| Arc::new(string))
        .map_err(|err| err.kind())
        .map_err(Error::IO)
}

fn main() -> iced::Result {
    Texteditor::run(Settings::default())
}
