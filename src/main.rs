use iced::{Element, color}
use iced::widget::{self, text, column, center, text_input}

pub fn main() -> iced:Result {

}

#[Derive(debug)]
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

struct Search {
        content: String,
}

impl ProtonDB {
    fn view(&self) -> Element<Message> {
        let content: Element<_> = match self {
            ProtonDB::Search => column![
                text("Put steam id to search game.").size(40),
                text_input("", &self).on_input(MessageSearch::ContentChanged).into(),
                button("Search").on_press(Message::Search)
            ]
            .max_width(500)
            .spacing(20)
            .align_x(Center)
            .into(),
            ProtonDB::Loading => {
                text("Searching the game...").size(50).align_x(Center).into()
            }
            ProtonDB::Error => column! [
                text("Could not find game.").size(20).color(color!(0xff0000)).into(),
                text("Put steam id to search game.").size(40),
                text_input("", &self).on_input(MessageSearch::ContentChanged).into(),
                button("Search").on_press(Message::Search)
            ]
            .max_width(500)
            .spacing(20)
            .align_x(Center)
            .into(),
            ProtonDB::Loaded { game } => // TODO:
        }

    }

    fn update (&mut self, message: Message) {

    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}


impl Search {
        fn view(&self) {
        }

        fn update(&mut self, message: MessageSearch) {
            match message {
                MessageSearch::ContentChanged(content) => {
                    Search.content = content;
                }
            }
        }
    }

