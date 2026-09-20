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

              ${
                !todo.completed
                  ? `<button class="complete-button" data-id="${todo.id}">
                      Complete
                    </button>`
                  : ""
              }

              <button class="edit-button" data-id="${todo.id}" data-title="${todo.title}">
                Edit
              </button>
              <button class="delete-button" data-id="${todo.id}">
                Delete
              </button>
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

  document.querySelectorAll<HTMLButtonElement>(".complete-button")
    .forEach(button => {
      button.addEventListener("click", async () => {
        const id = Number(button.dataset.id);

        await invoke("complete_todo", { id });

        await loadTodos();
      });
    });

  document.querySelectorAll<HTMLButtonElement>(".edit-button")
    .forEach(button => {
      button.addEventListener("click", async () => {
        const id = Number(button.dataset.id);
        const currentTitle = button.dataset.title ?? "";

        const title = prompt("Neuer Titel:", currentTitle);

        if (!title?.trim()) {
          return;
        }

        await invoke("edit_todo", {
          id,
          title: title.trim(),
        });

        await loadTodos();
      });
    });

  document.querySelectorAll<HTMLButtonElement>(".delete-button")
    .forEach(button => {
      button.addEventListener("click", async () => {
        const id = Number(button.dataset.id);

        await invoke("delete_todo", {
          id
        });

        await loadTodos();
      });
    });
}

await loadTodos();