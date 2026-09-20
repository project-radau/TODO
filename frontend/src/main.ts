import {
  getTodos
} from "./api";

import { renderTodos } from "./ui";
import { 
  setupAddTodo,
  setupTodoEvents
} from "./events";

async function loadTodos() {
  const todos = await getTodos();

  renderTodos(todos);
}

await loadTodos();

setupAddTodo(loadTodos);
setupTodoEvents(loadTodos);