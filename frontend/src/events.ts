import {
  addTodo,
  completeTodo,
  editTodo,
  deleteTodo
} from "./api";

export function setupAddTodo(onChanged: () => Promise<void>) {
  document
    .querySelector<HTMLFormElement>("#add-form")!
    .addEventListener("submit", async event => {
        event.preventDefault();

        const input = document.querySelector<HTMLInputElement>("#title")!;

        if (!input.value.trim()) {
            return;
        }

        console.log("TITLE:", input.value);
        await addTodo(input.value);

        await onChanged();
    });
}

export function setupTodoEvents(onChanged: () => Promise<void>) {
  document
    .querySelector<HTMLDivElement>("#app")!
    .addEventListener("click", async event => {
        const target = event.target as HTMLElement;
        const button = target.closest("button");

        if (!button) {
            return;
        }

        const id = Number(button.dataset.id);

        if (button.classList.contains("complete-button")) {
            await completeTodo(id);
        }

        if (button.classList.contains("edit-button")) {
            const currentTitle = button.dataset.title ?? "";
            const title = prompt("Neuer Titel:", currentTitle);

            if (!title?.trim()) {
            return;
            }

            await editTodo(id, title.trim());
        }

        if (button.classList.contains("delete-button")) {
            await deleteTodo(id);
        }

        await onChanged();
    });
}