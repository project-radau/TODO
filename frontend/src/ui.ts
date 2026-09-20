import type { Todo } from "./api";

export function renderTodos(todos: Todo[]) {
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

              <button
                class="edit-button"
                data-id="${todo.id}"
                data-title="${todo.title}"
              >
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
}