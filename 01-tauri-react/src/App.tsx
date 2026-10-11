import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Todo = { id: number; title: string; done: boolean };

export default function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [input, setInput] = useState("");

  // Rust 側のコマンドを呼び、返ってきた全件で state を置き換える
  useEffect(() => {
    invoke<Todo[]>("list").then(setTodos);
  }, []);

  const add = async (e: FormEvent) => {
    e.preventDefault();
    setTodos(await invoke<Todo[]>("add", { title: input }));
    setInput("");
  };
  const toggle = async (id: number) => setTodos(await invoke<Todo[]>("toggle", { id }));
  const remove = async (id: number) => setTodos(await invoke<Todo[]>("remove", { id }));

  return (
    <main>
      <h2>TODO (Tauri + React)</h2>
      <form onSubmit={add}>
        <input value={input} onChange={(e) => setInput(e.target.value)} placeholder="新しいタスク" autoFocus />{" "}
        <button>追加</button>
      </form>
      <ul>
        {todos.map((t) => (
          <li key={t.id} className={t.done ? "done" : ""}>
            <input type="checkbox" checked={t.done} onChange={() => toggle(t.id)} />
            <span> {t.title} </span>
            <button type="button" onClick={() => remove(t.id)}>削除</button>
          </li>
        ))}
      </ul>
      <p>残り {todos.filter((t) => !t.done).length} 件</p>
    </main>
  );
}
