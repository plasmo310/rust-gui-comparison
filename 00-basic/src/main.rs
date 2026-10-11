//! Rust の主要機能を TODO ドメインで一通り見せるサンプル。
//! 実行: `cargo run` / テスト: `cargo test`

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;

// ---------- 1. 構造体・列挙型・derive ----------
#[derive(Debug, Clone, PartialEq)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
    priority: Priority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Priority {
    Low,
    Normal,
    High,
}

// ---------- 2. トレイト (Display の実装) ----------
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mark = if self.done { "x" } else { " " };
        write!(f, "[{mark}] #{} {} ({:?})", self.id, self.title, self.priority)
    }
}

// ---------- 3. エラー処理 (Result / ? / From) ----------
#[derive(Debug)]
enum TodoError {
    EmptyTitle,
    NotFound(u32),
    BadId(std::num::ParseIntError),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::EmptyTitle => write!(f, "title is empty"),
            TodoError::NotFound(id) => write!(f, "todo #{id} not found"),
            TodoError::BadId(e) => write!(f, "invalid id: {e}"),
        }
    }
}

impl std::error::Error for TodoError {}

impl From<std::num::ParseIntError> for TodoError {
    fn from(e: std::num::ParseIntError) -> Self {
        TodoError::BadId(e)
    }
}

// ---------- 4. impl ブロックと所有権・借用 ----------
#[derive(Default)]
struct TodoList {
    items: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    fn add(&mut self, title: &str, priority: Priority) -> Result<u32, TodoError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(TodoError::EmptyTitle);
        }
        self.next_id += 1;
        self.items.push(Todo { id: self.next_id, title: title.to_string(), done: false, priority });
        Ok(self.next_id)
    }

    fn find_mut(&mut self, id: u32) -> Result<&mut Todo, TodoError> {
        self.items.iter_mut().find(|t| t.id == id).ok_or(TodoError::NotFound(id))
    }

    /// 文字列 ID を受け取り、`?` でエラーを伝播させる。
    fn toggle_str(&mut self, id: &str) -> Result<(), TodoError> {
        let id: u32 = id.parse()?; // ParseIntError -> TodoError (From)
        let t = self.find_mut(id)?;
        t.done = !t.done;
        Ok(())
    }

    fn remove(&mut self, id: u32) -> Result<Todo, TodoError> {
        let pos = self.items.iter().position(|t| t.id == id).ok_or(TodoError::NotFound(id))?;
        Ok(self.items.remove(pos))
    }

    /// 戻り値のライフタイムは &self に紐づく (省略規則で `'_` 相当)。
    fn pending(&self) -> impl Iterator<Item = &Todo> {
        self.items.iter().filter(|t| !t.done)
    }
}

// ---------- 5. ジェネリクスとトレイト境界 ----------
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for TodoList {
    fn summarize(&self) -> String {
        format!("{} items, {} pending", self.items.len(), self.pending().count())
    }
}

fn print_summary<T: Summary>(x: &T) {
    println!("summary: {}", x.summarize());
}

fn largest<T: PartialOrd + Copy>(xs: &[T]) -> Option<T> {
    let mut it = xs.iter().copied();
    let first = it.next()?;
    Some(it.fold(first, |m, x| if x > m { x } else { m }))
}

// ---------- 6. 明示的ライフタイム ----------
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// ---------- 7. パターンマッチ ----------
fn describe(t: &Todo) -> &'static str {
    match (t.done, t.priority) {
        (true, _) => "finished",
        (false, Priority::High) => "urgent!",
        (false, Priority::Normal | Priority::Low) => "later",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 所有権: add は &mut self を借用するだけ。list は main が所有し続ける。
    let mut list = TodoList::default();
    list.add("Rust を学ぶ", Priority::High)?;
    list.add("牛乳を買う", Priority::Low)?;
    list.add("  README を書く  ", Priority::Normal)?;

    // エラーは値として扱う
    if let Err(e) = list.add("   ", Priority::Low) {
        println!("expected error: {e}");
    }
    if let Err(e) = list.toggle_str("abc") {
        println!("expected error: {e}");
    }
    list.toggle_str("2")?;
    println!("removed: {}", list.remove(3)?);

    for t in &list.items {
        println!("{t} -> {}", describe(t));
    }
    print_summary(&list);

    // ---------- 8. イテレータ・クロージャ・コレクション ----------
    let titles: Vec<String> = list.items.iter().map(|t| t.title.to_uppercase()).collect();
    println!("titles: {titles:?}");

    let mut by_priority: HashMap<Priority, Vec<&Todo>> = HashMap::new();
    for t in &list.items {
        by_priority.entry(t.priority).or_default().push(t);
    }
    let mut keys: Vec<_> = by_priority.keys().copied().collect();
    keys.sort(); // Ord を derive しているので比較可能
    println!("priorities: {keys:?}");

    let threshold = 1; // クロージャは環境をキャプチャ
    let long_titles = list.items.iter().filter(|t| t.title.chars().count() > threshold).count();
    println!("long titles: {long_titles}");

    println!("largest: {:?}, longer: {}", largest(&[3, 9, 4]), longer("abc", "de"));

    // ---------- 9. Option / 文字列スライス ----------
    let first_word = list.items.first().and_then(|t| t.title.split_whitespace().next());
    println!("first word: {first_word:?}");

    // ---------- 10. 並行処理: Arc<Mutex<_>> + スレッド ----------
    let counter = Arc::new(Mutex::new(0u32));
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                *c.lock().unwrap() += i;
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("thread sum: {}", *counter.lock().unwrap());

    Ok(())
}

// ---------- 11. テスト ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_title() {
        let mut l = TodoList::default();
        assert!(matches!(l.add("  ", Priority::Low), Err(TodoError::EmptyTitle)));
    }

    #[test]
    fn toggle_and_remove() {
        let mut l = TodoList::default();
        let id = l.add("a", Priority::Normal).unwrap();
        l.toggle_str(&id.to_string()).unwrap();
        assert!(l.items[0].done);
        assert_eq!(l.remove(id).unwrap().title, "a");
        assert!(matches!(l.remove(id), Err(TodoError::NotFound(_))));
    }

    #[test]
    fn generic_largest() {
        assert_eq!(largest(&[1.5, 0.2]), Some(1.5));
        assert_eq!(largest::<i32>(&[]), None);
    }
}
