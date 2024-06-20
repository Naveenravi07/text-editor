use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, text, text_editor},
    Application, Command, Settings, Theme,
};
use rfd::AsyncFileDialog;
use std::{
    env, io,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone, Debug)]
enum Error {
    IO(io::ErrorKind),
    DialogClosed,
}

#[derive(Debug, Clone)]
enum Messages {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    Open,
    New,
}

struct Texteditor {
    path: Option<PathBuf>,
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
                path: None,
            },
            Command::perform(load_file(default_file()), Messages::FileOpened),
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
                    Ok((path, content)) => {
                        println!("{}", &path.to_string_lossy());
                        self.path = Some(path);
                        self.content = text_editor::Content::with_text(&content);
                    }
                    Err(kind) => {
                        self.error = Some(kind);
                    }
                };
                Command::none()
            }

            Messages::Open => Command::perform(pick_file(), Messages::FileOpened),
            Messages::New => {
                self.path = None;
                self.content = text_editor::Content::new();
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let controls = row![
            button("New").on_press(Messages::New),
            button("Open").on_press(Messages::Open)
        ]
        .spacing(10);

        let editor = text_editor(&self.content)
            .on_action(Messages::Edit)
            .height(800);

        let status_bar = {
            let status = if let Some(Error::IO(error)) = self.error {
                text(error.to_string())
            } else {
                match self.path.as_deref().and_then(Path::to_str) {
                    Some(path) => text(path),
                    None => text("New file"),
                }
            };

            let pos = {
                let (line, col) = self.content.cursor_position();
                let formatted = format!("{}:{}", line, col);
                text(formatted)
            };

            row![status, horizontal_space(), pos]
        };
        container(column![controls, editor, status_bar]).into()
    }
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = AsyncFileDialog::new()
        .set_title("Choose a file to open")
        .set_directory("/")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let file_content = tokio::fs::read_to_string(&path)
        .await
        .map(|string| Arc::new(string))
        .map_err(|err| err.kind())
        .map_err(Error::IO)?;

    Ok((path, file_content))
}

fn main() -> iced::Result {
    Texteditor::run(Settings::default())
}
