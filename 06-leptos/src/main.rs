use leptos::prelude::*;

#[derive(Clone)]
struct Todo {
    id: u32,
    title: String,
    done: RwSignal<bool>,
}

#[component]
fn App() -> impl IntoView {
    let (todos, set_todos) = signal(Vec::<Todo>::new());
    let (input, set_input) = signal(String::new());
    let next_id = StoredValue::new(0u32);

    let add = move || {
        let t = input.get().trim().to_string();
        if !t.is_empty() {
            let id = next_id.get_value();
            next_id.set_value(id + 1);
            set_todos.update(|v| v.push(Todo { id, title: t, done: RwSignal::new(false) }));
            set_input.set(String::new());
        }
    };
    let remaining = move || todos.get().iter().filter(|t| !t.done.get()).count();

    view! {
        <h2>"TODO (Leptos)"</h2>
        <input
            placeholder="新しいタスク"
            prop:value=input
            on:input:target=move |ev| set_input.set(ev.target().value())
            on:keydown=move |ev| if ev.key() == "Enter" { add() }
        />
        <button on:click=move |_| add()>"追加"</button>
        <ul>
            <For each=move || todos.get() key=|t| t.id let:t>
                <li>
                    <input type="checkbox" prop:checked=t.done on:change=move |_| t.done.update(|d| *d = !*d) />
                    <span>{t.title.clone()}</span>
                    <button on:click=move |_| set_todos.update(|v| v.retain(|x| x.id != t.id))>"削除"</button>
                </li>
            </For>
        </ul>
        <p>"残り " {remaining} " 件"</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
