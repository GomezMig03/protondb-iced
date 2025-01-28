use iced::{Element}
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
    Search,
}

impl ProtonDB {
    fn view(&self) -> Element<Message> {
        let content: Element<_> = match self {
            ProtonDB::Search => column![
            ]

    }

    fn update (&mut self, message: Message) {

    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

struct Search {
        content: String,
}

enum MessageSearch {
    ContentChanged(String)
}

impl Search {
        fn view(&self) {
            column![
                text("Put steam id to search game.").size(40),
                text_input("", &self).on_input(MessageSearch::ContentChanged).into(),
                button("Search").on_press(Message::Search)
            ]
        }

        fn update(&mut self, message: MessageSearch) {
            match message {
                MessageSearch::ContentChanged(content) => {
                    Search.content = content;
                }
            }
        }
    }

