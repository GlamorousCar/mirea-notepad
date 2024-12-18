

use iced::application;
use iced::highlighter::{self, Highlighter};
use iced::widget::{
    button, column,Column, container, horizontal_space, pick_list, row, text, text_editor, tooltip,
};
use iced::{
    executor, keyboard, theme, Application, Sandbox,Command, Element, Font, Length, Settings, Subscription,
    Theme,
};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use iced::time::{self, Duration};
use iced::keyboard::key;
use iced::widget::shader::wgpu::hal::ShaderInput;
// use crate::FontFamily::SansSerif;

pub fn main() -> iced::Result {


    Editor::run(Settings {
        default_font: Font::MONOSPACE,
        // default_font: SansSerif


        fonts: vec![include_bytes!("../fonts/editor-icons.ttf")
            .as_slice()
            .into()],
        ..Settings::default()
    })
}


pub struct Editor {
    theme: highlighter::Theme,
    editor_theme: theme::Theme,
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
    is_loading: bool,
    is_dirty: bool,
    undo_stack: Vec<String>, // Для хранения истории изменений
    redo_stack: Vec<String>, // Для хранения состояний для повторов
    font_family: FontFamily,

    auto_save_interval: Duration,
    last_save_time: Option<std::time::Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FontFamily {
    Monospace,
    Default,
    // SansSerif,
}

impl FontFamily {
    fn all() -> &'static [FontFamily] {
        &[FontFamily::Monospace, FontFamily::Default,]
    }
}
impl ToString for FontFamily {
    fn to_string(&self) -> String {
        match self {
            FontFamily::Monospace => "Monospace",
            FontFamily::Default => "Default",
            // FontFamily::SansSerif => "Sans-Serif",
        }
            .to_string()
    }
}


#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    New,
    Open,
    Save,

    FileSave(Result<PathBuf, Error>),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    ThemeSelected(highlighter::Theme),
    EThemeSelected(theme::Theme),


    AutoSaveTriggered,

    FontSelected(FontFamily),
    FileSaved(Result<PathBuf, Error>),

    ActionPerformed(text_editor::Action),
    Undo, // Новое сообщение для Undo
    Redo, // Новое сообщение для Redo
}


#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}


impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                path: None,
                content: text_editor::Content::new(),
                error: None,
                theme: highlighter::Theme::SolarizedDark,
                font_family: FontFamily::Monospace,

                editor_theme: theme::Theme::Dark,

                is_loading: false,
                is_dirty: false,
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),

                auto_save_interval: Duration::from_secs(60), // Автосохранение каждые 60 секунд
                last_save_time: None,

            },

            Command::perform(load_file(default_file()), Message::FileOpened),

        )
    }

    fn title(&self) -> String  {
        String::from("Mirea notepad by John Drof 1.0")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content.perform(action);
                self.error = None;
                Command::none()
            }
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
            Message::New => {
                self.path = None;
                self.content = text_editor::Content::new();
                self.is_dirty = true;
                Command::none()
            },
            Message::Save => {
                let text = self.content.text();
                Command::perform(save_file(self.path.clone(), text), Message::FileSave)
            }
            Message::FileSave(Ok(path)) => {
                self.path = Some(path);
                self.is_dirty = false;
                Command::none()
            }
            Message::FileSave(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::FileOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.is_dirty = false;
                self.content = text_editor::Content::with_text(&content);

                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);

                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Command::none()
            }
            Message::EThemeSelected(theme) => {
                self.editor_theme = theme;
                Command::none()
            }
            Message::FontSelected(font) => {
                self.font_family = font;
                Command::none()
            }
            Message::AutoSaveTriggered => {
                if self.is_dirty && !self.is_loading {
                    self.is_loading = true;
                    let text = self.content.text();
                    return Command::perform(save_file(self.path.clone(), text), Message::FileSave);
                }
                Command::none()
            }
            Message::FileSaved(result) => {
                self.is_loading = false;
                if let Ok(path) = result {
                    self.path = Some(path);
                    self.is_dirty = false;
                    self.last_save_time = Some(std::time::Instant::now());
                }
                Command::none()
            }


            Message::ActionPerformed(action) => {
                if action.is_edit() {
                    // Сохранить текущее состояние для Undo
                    self.undo_stack.push(self.content.text().to_string());
                    self.redo_stack.clear(); // Очистить стек Redo после нового изменения
                }
                self.is_dirty = true;
                self.content.perform(action);
                Command::none()
            }
            Message::Undo => {
                if let Some(previous_state) = self.undo_stack.pop() {
                    // Сохранить текущее состояние для Redo
                    self.redo_stack.push(self.content.text().to_string());
                    // Восстановить предыдущее состояние
                    self.content = text_editor::Content::with_text(&previous_state);
                }
                Command::none()
            }
            Message::Redo => {
                if let Some(next_state) = self.redo_stack.pop() {

                    // Сохранить текущее состояние для Undo
                    self.undo_stack.push(self.content.text().to_string());
                    // Восстановить следующее состояние
                    self.content = text_editor::Content::with_text(&next_state);
                }
                Command::none()
            }

        }
    }
    fn subscription(&self) -> Subscription<Message> {
        let auto_save = time::every(self.auto_save_interval).map(|_| Message::AutoSaveTriggered);

        Subscription::batch(vec![
            auto_save,
            keyboard::on_key_press(|key, modifiers| match key.as_ref() {
                keyboard::Key::Character("z") if modifiers.command() => Some(Message::Undo),
                keyboard::Key::Character("y") if modifiers.command() => Some(Message::Redo),
                keyboard::Key::Character("s") if modifiers.command() => Some(Message::Save),
                keyboard::Key::Character("n") if modifiers.command() => Some(Message::New),
                keyboard::Key::Character("o") if modifiers.command() => Some(Message::Open),
                _ => None,
            })
            // Другие подписки...
        ])

    }
    fn view(&self) -> Element<Message> {
        let toolbar = row![
        action(new_icon(), "New File", Some(Message::New)),
        action(open_icon(), "Open File", Some(Message::Open)),
        action(save_icon(),"Save File", Some(Message::Save)),
        action(text("Undo").font(Font::DEFAULT).into(), "Undo", (!self.undo_stack.is_empty()).then_some(Message::Undo)),
        // action(undo_icon(), "Undo", (!self.undo_stack.is_empty()).then_some(Message::Undo)),
        action(text("Redo").font(Font::DEFAULT).into(), "Redo", (!self.redo_stack.is_empty()).then_some(Message::Redo)),

        horizontal_space().width(Length::Fill),
        pick_list(
            highlighter::Theme::ALL,
            Some(self.theme),
            Message::ThemeSelected
        ),
        pick_list(
            theme::Theme::ALL,
            Some(self.editor_theme.clone()),
            Message::EThemeSelected
        ),
        pick_list(FontFamily::all(), Some(self.font_family), Message::FontSelected)
            .text_size(14)
            .padding([5, 10])

        ]
            .spacing(5)
            .padding(10)
            .align_items(iced::Alignment::Center);;

        let editor = text_editor(&self.content)
            .on_action(Message::ActionPerformed)
            .highlight::<Highlighter>(
                highlighter::Settings {
                    theme: self.theme,
                    extension: self
                        .path
                        .as_ref()
                        .and_then(|path| path.extension()?.to_str())
                        .unwrap_or("rs")
                        .to_string(),
                },
                |highlight, _theme| highlight.to_format() ,
            ).font(match self.font_family {
            FontFamily::Monospace => Font::MONOSPACE,
            FontFamily::Default => Font::DEFAULT,
        });

        Column::new()
            .push(toolbar)
            .push(
                container(editor)
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .into()
    }
    fn theme(&self) -> Theme {
        self.editor_theme.clone()
    }
}


fn action<'a>(
    content: Element<'a, Message>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let is_disabled = on_press.is_none();
    tooltip(
        button(container(content).width(42).center_x().center_y())
            .on_press_maybe(on_press)
            .padding([5, 10])
            .style(if is_disabled {
                theme::Button::Secondary
            } else {
                theme::Button::Primary
            }),
        label,
        tooltip::Position::FollowCursor,
    )
        .style(theme::Container::Box)
        .into()
}
fn icon<'a>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");
    text(codepoint).font(ICON_FONT).into()
}



fn new_icon<'a>() -> Element<'a, Message> {
    icon('\u{F1C9}')
}

fn save_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

fn open_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}



async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose A Text File...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    // let contents = tokio::fs::read_to_string(&path)
    let contents = fs::read_to_string(&path)
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;

    Ok((path, contents))
}

async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::FileDialog::new()
            .set_title("Choose A File Name")
            .save_file()
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.to_path_buf())?
    };
    fs::write(&path, text).map_err(|error| Error::IO(error.kind()))?;
    Ok(path)
}
fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/README.md", env!("CARGO_MANIFEST_DIR")))
}