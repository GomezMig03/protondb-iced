use iced::{Element, color, Center, Theme, Length, widget};
use iced::widget::{text, column, text_input, container, image, row, button};

pub fn main() -> iced::Result {
    iced::application("ProtonDB-iced", ProtonDB::update, ProtonDB::view)
        .theme(ProtonDB::theme)
        .run()
}

#[derive(Debug)]
enum ProtonDB {
    Search,
    Loading,
    Error,
    Loaded { game: Game },
}

#[derive(Debug, Clone)]
enum Message {
    GameFound(Result<Game, Error>),
    Search, // TODO: Change the name
    ContentChanged(String)
}

struct SearchBar {
   Content: String,
}

impl ProtonDB {
    fn view(&self) -> Element<Message> {
        let content = match self {
            ProtonDB::Search => column![
                text("Put steam id to search game.").size(40),
                text_input("", &SearchBar::Content).on_input(Message::ContentChanged).into(),
                button("Search").on_press(Message::Search)
            ]
            .max_width(500)
            .spacing(20),
            ProtonDB::Loading => {
                column![
                    text("Searching the game...").size(50).align_x(Center)
                ]
            }
            ProtonDB::Error => column! [
                text("Could not find game.").size(20).color(color!(0xff0000)).into(),
                text("Put steam id to search game.").size(40),
                text_input("", &SearchBar::Content).on_input(Message::ContentChanged).into(),
                button("Search").on_press(Message::Search)
            ]
            .max_width(500)
            .spacing(20),
            ProtonDB::Loaded { game } => column![
                game.view(),
                button("Go back to home.").on_press(Message::Search)
            ]
            .spacing(15)
            .align_x(Center),
        };
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn update (&mut self, message: Message) {
        match message {
            Message::GameFound(Ok(game)) => {
                *self = ProtonDB::Loaded { game };
            }
            Message::GameFound(Err(_error)) => {
                *self = ProtonDB::Error;
            }
            Message::Search => match self {
                ProtonDB::Loading => println!("test"),
                _ => {
                    *self = ProtonDB::Loading;

                    //Command::perform(ProtonDB::search(), Message::GameFound)
                }
            },
        }

    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

#[derive(Debug, Clone)]
enum Medal {
    Platinum,
    Gold,
    Silver,
    Bronze,
    Borked,
    Native
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    name: String,
    medal: Medal,
    image: image,
}

impl Game {
    fn view(&self) -> Element<Message> {
        row![
            image::viewer(self.image.clone()),
            column![
                row![
                    text(&self.name).size(25).width(Length::Fill),
                    text(format!("#{}", self.id)).size(15),
                ]
                .align_y(Center)
                .spacing(20),
                self.medal,
            ]
            .spacing(20),
        ]
        .spacing(20)
        .align_y(Center)
        .into()
    }
}

#[derive(Debug, Clone)]
enum Error {
    APIError,
    LanguageError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
