use slint::{Model, ModelRc, VecModel};
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let todos = Rc::new(VecModel::<TodoItem>::default());
    ui.set_todos(ModelRc::from(todos.clone()));

    let update_remaining = {
        let ui = ui.as_weak();
        let todos = todos.clone();
        move || {
            let n = todos.iter().filter(|t| !t.done).count() as i32;
            ui.unwrap().set_remaining(n);
        }
    };

    ui.on_add({
        let todos = todos.clone();
        let ui = ui.as_weak();
        let update = update_remaining.clone();
        move |title| {
            let title = title.trim();
            if !title.is_empty() {
                todos.push(TodoItem { title: title.into(), done: false });
                ui.unwrap().set_input("".into());
                update();
            }
        }
    });
    ui.on_toggle({
        let todos = todos.clone();
        let update = update_remaining.clone();
        move |i, done| {
            if let Some(mut t) = todos.row_data(i as usize) {
                t.done = done;
                todos.set_row_data(i as usize, t);
                update();
            }
        }
    });
    ui.on_remove({
        let todos = todos.clone();
        let update = update_remaining.clone();
        move |i| {
            todos.remove(i as usize);
            update();
        }
    });

    ui.run()
}
