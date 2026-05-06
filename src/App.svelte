<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type Todo = {
    id: number;
    title: string;
    completed: boolean;
  };

  let todoTitle = "";
  let todos: Todo[] = [];
  let todoErrorMessage = "";

  onMount(() => {
    void loadTodos();
  });

  async function loadTodos() {
    todoErrorMessage = "";

    try {
      todos = await invoke<Todo[]>("list_todos");
    } catch (error) {
      todoErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function createTodo() {
    todoErrorMessage = "";

    try {
      const todo = await invoke<Todo>("create_todo", { title: todoTitle });
      todos = [...todos, todo];
      todoTitle = "";
    } catch (error) {
      todoErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function completeTodo(id: number) {
    todoErrorMessage = "";

    try {
      const completedTodo = await invoke<Todo>("complete_todo", { id });
      todos = todos.map((todo) => (todo.id === id ? completedTodo : todo));
    } catch (error) {
      todoErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<main class="container">
  <section class="intro">
    <h1>Tauri + Svelte</h1>
    <p>Todo の操作を通して、Svelte から Rust command を呼ぶ流れを練習します。</p>
  </section>

  <section class="todo-section" aria-labelledby="todo-heading">
    <h2 id="todo-heading">Todo</h2>

    <form class="todo-form" on:submit|preventDefault={createTodo}>
      <label for="todo-title">Title</label>
      <div class="row">
        <input id="todo-title" bind:value={todoTitle} placeholder="Add a todo" />
        <button type="submit">Add</button>
      </div>
    </form>

    {#if todoErrorMessage}
      <p class="error">{todoErrorMessage}</p>
    {/if}

    {#if todos.length > 0}
      <ul class="todo-list">
        {#each todos as todo (todo.id)}
          <li class:completed={todo.completed}>
            <span>{todo.title}</span>
            <div class="todo-actions">
              <small>#{todo.id}</small>
              <button
                type="button"
                class="secondary"
                disabled={todo.completed}
                on:click={() => completeTodo(todo.id)}
              >
                {todo.completed ? "Done" : "Complete"}
              </button>
            </div>
          </li>
        {/each}
      </ul>
    {:else}
      <p class="empty">No todos yet.</p>
    {/if}
  </section>
</main>
