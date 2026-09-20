import { invoke } from "@tauri-apps/api/core";

export type Todo = {
  id: number;
  title: string;
  completed: boolean;
};

export async function getTodos(): Promise<Todo[]> {
  return await invoke<Todo[]>("get_todos");
}

export async function addTodo(title: string): Promise<number> {
  return await invoke<number>("add_todo", { title });
}

export async function completeTodo(id: number): Promise<void> {
  await invoke("complete_todo", { id });
}

export async function editTodo(id: number, title: string): Promise<void> {
  await invoke("edit_todo", { id, title });
}

export async function deleteTodo(id: number): Promise<void> {
  await invoke("delete_todo", { id });
}