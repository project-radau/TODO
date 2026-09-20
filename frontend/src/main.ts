import {
  getTodos
} from "./api";

import { renderTodos } from "./ui";
import { 
  setupAddTodo,
  setupCompleteTodos,
  setupEditTodos,
  setupDeleteTodos
} from "./events";

async function loadTodos() {
  const todos = await getTodos();

  renderTodos(todos);
}

await loadTodos();

setupAddTodo(loadTodos);
setupCompleteTodos(loadTodos);
setupEditTodos(loadTodos);
setupDeleteTodos(loadTodos);