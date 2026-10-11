use iced::widget::{button, checkbox, column, row, text, text_input};
use iced::{Element, Fill};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

struct Todo {
    title: String,
    done: bool,
}

#[derive(Default)]
struct App {
    todos: Vec<Todo>,
    input: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Add,
    Toggle(usize, bool),
    Delete(usize),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(s) => self.input = s,
            Message::Add => {
                let t = self.input.trim();
                if !t.is_empty() {
                    self.todos.push(Todo { title: t.to_string(), done: false });
                    self.input.clear();
                }
            }
            Message::Toggle(i, v) => self.todos[i].done = v,
            Message::Delete(i) => {
                self.todos.remove(i);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let list = self.todos.iter().enumerate().fold(column![].spacing(6), |col, (i, t)| {
            col.push(
                row![
                    checkbox(t.done).label(t.title.as_str()).on_toggle(move |v| Message::Toggle(i, v)).width(Fill),
                    button("削除").on_press(Message::Delete(i)),
                ]
                .spacing(8),
            )
        });
        let remaining = self.todos.iter().filter(|t| !t.done).count();
        column![
            text("TODO (iced)").size(28),
            row![
                text_input("新しいタスク", &self.input).on_input(Message::InputChanged).on_submit(Message::Add),
                button("追加").on_press(Message::Add),
            ]
            .spacing(8),
            list,
            text(format!("残り {remaining} 件")),
        ]
        .spacing(12)
        .padding(16)
        .into()
    }
}
