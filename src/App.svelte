<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
  } from "@tauri-apps/plugin-notification";
  import { onMount } from "svelte";

  type Todo = {
    id: number;
    title: string;
    completed: boolean;
  };

  type WorkLog = {
    id: number;
    body: string;
    createdAtMs: number;
  };

  let todoTitle = "";
  let todos: Todo[] = [];
  let todoErrorMessage = "";
  let workLogBody = "";
  let workLogs: WorkLog[] = [];
  let workLogErrorMessage = "";
  let notificationStatus = "Not checked";
  let notificationErrorMessage = "";

  onMount(() => {
    void loadTodos();
    void loadWorkLogs();
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

  async function loadWorkLogs() {
    workLogErrorMessage = "";

    try {
      workLogs = await invoke<WorkLog[]>("list_work_logs");
    } catch (error) {
      workLogErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function createWorkLog() {
    workLogErrorMessage = "";

    try {
      const workLog = await invoke<WorkLog>("create_work_log", { body: workLogBody });
      workLogs = [workLog, ...workLogs];
      workLogBody = "";
    } catch (error) {
      workLogErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  function formatTimestamp(timestampMs: number) {
    return new Intl.DateTimeFormat("ja-JP", {
      dateStyle: "short",
      timeStyle: "short",
    }).format(new Date(timestampMs));
  }

  async function sendTestNotification() {
    notificationErrorMessage = "";

    try {
      let permissionGranted = await isPermissionGranted();

      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === "granted";
      }

      notificationStatus = permissionGranted ? "Granted" : "Denied";

      if (!permissionGranted) {
        return;
      }

      sendNotification({
        title: "tauri-lab",
        body: "Notification plugin is ready.",
      });
    } catch (error) {
      notificationErrorMessage = error instanceof Error ? error.message : String(error);
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

  <section class="work-log-section" aria-labelledby="work-log-heading">
    <h2 id="work-log-heading">Work Log</h2>

    <form class="work-log-form" on:submit|preventDefault={createWorkLog}>
      <label for="work-log-body">Body</label>
      <textarea
        id="work-log-body"
        bind:value={workLogBody}
        rows="4"
        placeholder="Write what you worked on"
      ></textarea>
      <button type="submit">Add log</button>
    </form>

    {#if workLogErrorMessage}
      <p class="error">{workLogErrorMessage}</p>
    {/if}

    {#if workLogs.length > 0}
      <ul class="work-log-list">
        {#each workLogs as workLog (workLog.id)}
          <li>
            <p>{workLog.body}</p>
            <small>{formatTimestamp(workLog.createdAtMs)} / #{workLog.id}</small>
          </li>
        {/each}
      </ul>
    {:else}
      <p class="empty">No work logs yet.</p>
    {/if}
  </section>

  <section class="notification-section" aria-labelledby="notification-heading">
    <h2 id="notification-heading">Notification</h2>
    <p>Permission: {notificationStatus}</p>
    <button type="button" on:click={sendTestNotification}>Send test notification</button>

    {#if notificationErrorMessage}
      <p class="error">{notificationErrorMessage}</p>
    {/if}
  </section>
</main>
