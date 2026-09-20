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

      await addTodo(input.value);

      await onChanged();
    });
}

export function setupCompleteTodos(onChanged: () => Promise<void>) {
    document.querySelectorAll<HTMLButtonElement>(".complete-button")
        .forEach(button => {
          button.addEventListener("click", async () => {
            const id = Number(button.dataset.id);
  
            await completeTodo(id);
  
            await onChanged();
          });
        });
}

export function setupEditTodos(onChanged: () => Promise<void>) {
    document.querySelectorAll<HTMLButtonElement>(".edit-button")
        .forEach(button => {
          button.addEventListener("click", async () => {
            const id = Number(button.dataset.id);
            const currentTitle = button.dataset.title ?? "";
    
            const title = prompt("Neuer Titel:", currentTitle);
    
            if (!title?.trim()) {
              return;
            }
    
            await editTodo(id, title.trim());
    
            await onChanged();
          });
        });
}

export function setupDeleteTodos(onChanged: () => Promise<void>) {
    document.querySelectorAll<HTMLButtonElement>(".delete-button")
    .forEach(button => {
      button.addEventListener("click", async () => {
        const id = Number(button.dataset.id);

        await deleteTodo(id);

        await onChanged();
      });
    });
}