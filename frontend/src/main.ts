import { invoke } from "@tauri-apps/api/core";

type Todo = {
  id: number;
  title: string;
  completed: boolean;
};

async function loadTodos() {
  const todos = await invoke<Todo[]>("get_todos");

  document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
    <h1>Todos</h1>

    <form id="add-form">
      <input id="title" type="text" placeholder="New todo..." />
      <button type="submit">Add</button>
    </form>

    <ul>
      ${todos
        .map(
          todo => `
            <li>
              ${todo.id}: ${todo.title}
              ${todo.completed ? "✓" : ""}
            </li>
          `
        )
        .join("")}
    </ul>
  `;

  document.querySelector<HTMLFormElement>("#add-form")!
    .addEventListener("submit", async event => {
      event.preventDefault();

      const input = document.querySelector<HTMLInputElement>("#title")!;

      if (!input.value.trim()) {
        return;
      }

      await invoke<number>("add_todo", {
        title: input.value,
      });

      await loadTodos();
    });
}

await loadTodos();