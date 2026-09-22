import type { Todo } from "./api";
import { createIcons, Plus, Check, Pencil, X } from "lucide";

export function renderTodos(todos: Todo[]) {
  document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
    <div class="app-shell">
      <main class="todo-app">

        <header class="app-header">
          <div>
            <span class="eyebrow">TASKS</span>
            <h1>My Todos</h1>
          </div>

          <div class="todo-count">
            ${todos.length}
          </div>
        </header>

        <form id="add-form" class="add-form">
          <div class="input-wrapper">
            <input
              id="title"
              type="text"
              placeholder="What needs to be done?"
              autocomplete="off"
            />
          </div>

          <button class="add-button" type="submit">
            <i data-lucide="plus"></i>
            <span>Add</span>
          </button>
        </form>

        <section class="todo-list">

          ${
            todos.length === 0
              ? `
                <div class="empty-state">
                  <div class="empty-icon">
                    <i data-lucide="check"></i>
                  </div>
                  <h2>All clear</h2>
                  <p>You don't have any tasks yet.</p>
                </div>
              `
              : todos
                  .map(
                    todo => `
                      <article
                        class="todo-item ${todo.completed ? "completed" : ""}"
                      >
                        <div class="todo-main">

                          <button
                            class="complete-button todo-checkbox"
                            data-id="${todo.id}"
                            aria-label="${
                              todo.completed
                                ? "Mark as incomplete"
                                : "Complete todo"
                            }"
                          >
                            ${todo.completed ? `<i data-lucide="check"></i>` : ""}
                          </button>

                          <div class="todo-content">
                            <span class="todo-title">
                              ${todo.title}
                            </span>

                            ${
                              todo.completed
                                ? `<span class="todo-status">Completed</span>`
                                : ""
                            }
                          </div>

                        </div>

                        <div class="todo-actions">

                          <button
                            class="edit-button icon-button"
                            data-id="${todo.id}"
                            data-title="${todo.title}"
                            aria-label="Edit todo"
                          >
                            <i data-lucide="pencil"></i>
                          </button>

                          <button
                            class="delete-button icon-button"
                            data-id="${todo.id}"
                            aria-label="Delete todo"
                          >
                            <i data-lucide="x"></i>
                          </button>

                        </div>
                      </article>
                    `
                  )
                  .join("")
          }

        </section>

      </main>
    </div>
  `;

  createIcons({
    icons: {
      Plus,
      Check,
      Pencil,
      X,
    },
  });
}