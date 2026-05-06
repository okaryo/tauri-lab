import { invoke } from "@tauri-apps/api/core";

export type Todo = {
  id: number;
  title: string;
  completed: boolean;
};

export function listTodos() {
  return invoke<Todo[]>("list_todos");
}

export function createTodo(title: string) {
  return invoke<Todo>("create_todo", { title });
}

export function completeTodo(id: number) {
  return invoke<Todo>("complete_todo", { id });
}
