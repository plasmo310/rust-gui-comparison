use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box as GBox, Button, CheckButton, Entry, Label, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

struct Todo {
    title: String,
    done: bool,
}

struct State {
    todos: Vec<Todo>,
    list: GBox,
    remaining: Label,
}

// 状態から GTK ウィジェットを再構築する（簡易実装）
fn render(state: &Rc<RefCell<State>>) {
    let s = state.borrow();
    while let Some(child) = s.list.first_child() {
        s.list.remove(&child);
    }
    for (i, t) in s.todos.iter().enumerate() {
        let row = GBox::new(Orientation::Horizontal, 8);
        let check = CheckButton::with_label(&t.title);
        check.set_active(t.done);
        check.set_hexpand(true);
        let st = state.clone();
        check.connect_toggled(move |c| {
            st.borrow_mut().todos[i].done = c.is_active();
            update_remaining(&st);
        });
        let del = Button::with_label("削除");
        let st = state.clone();
        del.connect_clicked(move |_| {
            st.borrow_mut().todos.remove(i);
            // borrow 解放後に再描画
            render(&st);
        });
        row.append(&check);
        row.append(&del);
        s.list.append(&row);
    }
    drop(s);
    update_remaining(state);
}

fn update_remaining(state: &Rc<RefCell<State>>) {
    let s = state.borrow();
    let n = s.todos.iter().filter(|t| !t.done).count();
    s.remaining.set_text(&format!("残り {n} 件"));
}

fn build_ui(app: &Application) {
    let list = GBox::new(Orientation::Vertical, 6);
    let remaining = Label::new(None);
    let state = Rc::new(RefCell::new(State { todos: vec![], list: list.clone(), remaining: remaining.clone() }));

    let entry = Entry::builder().placeholder_text("新しいタスク").hexpand(true).build();
    let add_btn = Button::with_label("追加");
    let add = {
        let state = state.clone();
        let entry = entry.clone();
        move || {
            let t = entry.text().trim().to_string();
            if !t.is_empty() {
                state.borrow_mut().todos.push(Todo { title: t, done: false });
                entry.set_text("");
                render(&state);
            }
        }
    };
    entry.connect_activate({
        let add = add.clone();
        move |_| add()
    });
    add_btn.connect_clicked(move |_| add());

    let input_row = GBox::new(Orientation::Horizontal, 8);
    input_row.append(&entry);
    input_row.append(&add_btn);

    let scroll = ScrolledWindow::builder().vexpand(true).child(&list).build();
    let root = GBox::new(Orientation::Vertical, 12);
    root.set_margin_top(16);
    root.set_margin_bottom(16);
    root.set_margin_start(16);
    root.set_margin_end(16);
    root.append(&Label::new(Some("TODO (GTK4)")));
    root.append(&input_row);
    root.append(&scroll);
    root.append(&remaining);

    render(&state);
    ApplicationWindow::builder().application(app).title("TODO (GTK4)").default_width(420).default_height(480).child(&root).build().present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id("dev.example.todo-gtk4").build();
    app.connect_activate(build_ui);
    app.run()
}
