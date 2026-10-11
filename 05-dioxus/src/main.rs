use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, PartialEq)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[component]
fn App() -> Element {
    let mut todos = use_signal(Vec::<Todo>::new);
    let mut input = use_signal(String::new);
    let mut next_id = use_signal(|| 0u32);

    let mut add = move || {
        let t = input().trim().to_string();
        if !t.is_empty() {
            let id = next_id();
            next_id += 1;
            todos.write().push(Todo { id, title: t, done: false });
            input.set(String::new());
        }
    };
    let remaining = todos.read().iter().filter(|t| !t.done).count();

    rsx! {
        div { style: "font-family: sans-serif; padding: 16px;",
            h2 { "TODO (Dioxus)" }
            input {
                placeholder: "新しいタスク",
                value: "{input}",
                oninput: move |e| input.set(e.value()),
                onkeydown: move |e| if e.key() == Key::Enter { add() },
            }
            button { onclick: move |_| add(), "追加" }
            ul {
                for t in todos.read().iter().cloned() {
                    li { key: "{t.id}",
                        input {
                            r#type: "checkbox",
                            checked: t.done,
                            onchange: move |_| {
                                if let Some(x) = todos.write().iter_mut().find(|x| x.id == t.id) {
                                    x.done = !x.done;
                                }
                            },
                        }
                        span { "{t.title}" }
                        button { onclick: move |_| todos.write().retain(|x| x.id != t.id), "削除" }
                    }
                }
            }
            p { "残り {remaining} 件" }
        }
    }
}
