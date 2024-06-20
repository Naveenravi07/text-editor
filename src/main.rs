use iced::{
    executor,
    highlighter::{self, Highlighter},
    theme,
    widget::{button, column, container, horizontal_space, row, text, text_editor, tooltip},
    Application, Command, Element, Font, Theme,
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
    Save,
    FileSaved(Result<PathBuf, Error>),
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

            Messages::Save => Command::perform(
                save_file(self.path.clone(), self.content.text()),
                Messages::FileSaved,
            ),

            Messages::FileSaved(Ok(path)) => {
                self.path = Some(path);
                Command::none()
            }

            Messages::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let controls = row![
            action(new_icon(), "New file", Messages::New),
            action(open_icon(), "Open file", Messages::Open),
            action(save_icon(), "Save file", Messages::Save)
        ]
        .spacing(10);

        let editor = text_editor(&self.content)
            .on_action(Messages::Edit)
            .highlight::<Highlighter>(
                highlighter::Settings {
                    theme: highlighter::Theme::Base16Eighties,
                    extension: self
                        .path
                        .as_ref()
                        .and_then(|path| path.extension()?.to_str())
                        .unwrap_or("rs")
                        .to_string(),
                },
                |highlight, _theme| highlight.to_format(),
            )
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
                println!("{formatted}");
                text(formatted)
            };

            row![status, horizontal_space(), pos]
        };
        container(column![controls, editor, status_bar]).into()
    }
}

fn action<'a>(
    el: Element<'a, Messages>,
    label: &'a str,
    onpress: Messages,
) -> Element<'a, Messages> {
    tooltip(
        button(container(el).width(30).center_x().center_y())
            .on_press(onpress)
            .padding([5, 10]),
        label,
        tooltip::Position::Bottom,
    )
    .style(theme::Container::Box)
    .into()
}
fn icon<'a>(codepoint: char) -> Element<'a, Messages> {
    const ICON_FONT: Font = Font::with_name("editor-icons");
    text(codepoint).font(ICON_FONT).into()
}

fn new_icon<'a>() -> Element<'a, Messages> {
    icon('\u{E800}')
}

fn save_icon<'a>() -> Element<'a, Messages> {
    icon('\u{E801}')
}
fn open_icon<'a>() -> Element<'a, Messages> {
    icon('\u{E802}')
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

async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        AsyncFileDialog::new()
            .set_title("Choose a file name ...")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.path().to_owned())?
    };

    let _ = tokio::fs::write(&path, text.as_bytes())
        .await
        .map_err(|err| Error::IO(err.kind()));

    Ok(path)
}

fn main() -> iced::Result {
    Texteditor::run(iced::Settings {
        default_font: Font::DEFAULT,
        fonts: vec![include_bytes!("../fonts/editor-icons.ttf")
            .as_slice()
            .into()],
        ..iced::Settings::default()
    })
}
