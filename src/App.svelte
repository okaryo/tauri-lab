<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    completeTodo as completeTodoCommand,
    createTodo as createTodoCommand,
    listTodos,
    type Todo,
  } from "./lib/api/todos";
  import {
    createWorkLog as createWorkLogCommand,
    listWorkLogs,
    type WorkLog,
  } from "./lib/api/workLogs";
  import {
    sendAppNotification,
    sendTestNotification as sendTestNotificationCommand,
    type NotificationPermissionStatus,
  } from "./lib/desktop/notifications";
  import {
    registerWorkLogShortcut as registerWorkLogShortcutCommand,
    unregisterWorkLogShortcut as unregisterWorkLogShortcutCommand,
    workLogShortcut,
  } from "./lib/desktop/globalShortcuts";

  const defaultWorkDurationMinutes = 25;
  const defaultBreakDurationMinutes = 5;

  type PomodoroMode = "work" | "break";
  type TimerStatus = "idle" | "running" | "paused";

  let todoTitle = "";
  let todos: Todo[] = [];
  let todoErrorMessage = "";
  let workLogBody = "";
  let workLogs: WorkLog[] = [];
  let workLogErrorMessage = "";
  let notificationStatus: NotificationPermissionStatus | "Not checked" = "Not checked";
  let notificationErrorMessage = "";
  let shortcutRegistered = false;
  let shortcutTriggerCount = 0;
  let shortcutStatus = "Not registered";
  let shortcutErrorMessage = "";
  let workDurationMinutes = defaultWorkDurationMinutes;
  let breakDurationMinutes = defaultBreakDurationMinutes;
  let timerNotificationsEnabled = true;
  let draftWorkDurationMinutes = workDurationMinutes;
  let draftBreakDurationMinutes = breakDurationMinutes;
  let draftTimerNotificationsEnabled = timerNotificationsEnabled;
  let settingsErrorMessage = "";
  let pomodoroMode: PomodoroMode = "work";
  let timerStatus: TimerStatus = "idle";
  let remainingSeconds = durationForMode("work");
  let timerId: ReturnType<typeof setInterval> | null = null;

  onMount(() => {
    void loadTodos();
    void loadWorkLogs();
  });

  onDestroy(() => {
    if (shortcutRegistered) {
      void unregisterWorkLogShortcutCommand();
    }

    stopTimer();
  });

  $: timerProgress =
    1 - remainingSeconds / durationForMode(pomodoroMode);

  async function loadTodos() {
    todoErrorMessage = "";

    try {
      todos = await listTodos();
    } catch (error) {
      todoErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function createTodo() {
    todoErrorMessage = "";

    try {
      const todo = await createTodoCommand(todoTitle);
      todos = [...todos, todo];
      todoTitle = "";
    } catch (error) {
      todoErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function completeTodo(id: number) {
    todoErrorMessage = "";

    try {
      const completedTodo = await completeTodoCommand(id);
      todos = todos.map((todo) => (todo.id === id ? completedTodo : todo));
    } catch (error) {
      todoErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function loadWorkLogs() {
    workLogErrorMessage = "";

    try {
      workLogs = await listWorkLogs();
    } catch (error) {
      workLogErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function createWorkLog() {
    workLogErrorMessage = "";

    try {
      const workLog = await createWorkLogCommand(workLogBody);
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
      notificationStatus = await sendTestNotificationCommand();
    } catch (error) {
      notificationErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function registerWorkLogShortcut() {
    shortcutErrorMessage = "";

    try {
      await registerWorkLogShortcutCommand(() => {
        shortcutTriggerCount += 1;
        shortcutStatus = `Triggered ${shortcutTriggerCount} time${
          shortcutTriggerCount === 1 ? "" : "s"
        }`;
      });
      shortcutRegistered = true;
      shortcutStatus = "Registered";
    } catch (error) {
      shortcutErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function unregisterWorkLogShortcut() {
    shortcutErrorMessage = "";

    try {
      await unregisterWorkLogShortcutCommand();
      shortcutRegistered = false;
      shortcutStatus = "Not registered";
    } catch (error) {
      shortcutErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  function startTimer() {
    if (timerStatus === "running") {
      return;
    }

    timerStatus = "running";
    timerId = setInterval(tickTimer, 1000);
  }

  function pauseTimer() {
    stopTimer();
    timerStatus = "paused";
  }

  function resetTimer() {
    stopTimer();
    timerStatus = "idle";
    pomodoroMode = "work";
    remainingSeconds = durationForMode("work");
  }

  function stopTimer() {
    if (timerId === null) {
      return;
    }

    clearInterval(timerId);
    timerId = null;
  }

  function tickTimer() {
    if (remainingSeconds > 1) {
      remainingSeconds -= 1;
      return;
    }

    const completedMode = pomodoroMode;
    const nextMode: PomodoroMode = pomodoroMode === "work" ? "break" : "work";
    pomodoroMode = nextMode;
    remainingSeconds = durationForMode(nextMode);
    void notifyTimerComplete(completedMode, nextMode);
  }

  async function notifyTimerComplete(completedMode: PomodoroMode, nextMode: PomodoroMode) {
    if (!timerNotificationsEnabled) {
      return;
    }

    notificationErrorMessage = "";

    const title = completedMode === "work" ? "Work session complete" : "Break complete";
    const body =
      nextMode === "break" ? "Time to take a short break." : "Time to start the next work session.";

    try {
      notificationStatus = await sendAppNotification(title, body);
    } catch (error) {
      notificationErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  function applyTimerSettings() {
    settingsErrorMessage = "";

    if (!isValidDurationMinutes(draftWorkDurationMinutes)) {
      settingsErrorMessage = "Work duration must be between 1 and 180 minutes.";
      return;
    }

    if (!isValidDurationMinutes(draftBreakDurationMinutes)) {
      settingsErrorMessage = "Break duration must be between 1 and 180 minutes.";
      return;
    }

    workDurationMinutes = draftWorkDurationMinutes;
    breakDurationMinutes = draftBreakDurationMinutes;
    timerNotificationsEnabled = draftTimerNotificationsEnabled;
    resetTimer();
  }

  function isValidDurationMinutes(value: number) {
    return Number.isInteger(value) && value >= 1 && value <= 180;
  }

  function durationForMode(mode: PomodoroMode) {
    return (mode === "work" ? workDurationMinutes : breakDurationMinutes) * 60;
  }

  function formatDuration(totalSeconds: number) {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;

    return `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;
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

  <section class="pomodoro-section" aria-labelledby="pomodoro-heading">
    <h2 id="pomodoro-heading">Pomodoro</h2>
    <div class="timer-panel">
      <p class="timer-mode">{pomodoroMode === "work" ? "Work" : "Break"}</p>
      <p class="timer-display">{formatDuration(remainingSeconds)}</p>
      <progress
        class="timer-progress"
        value={timerProgress}
        max="1"
        aria-label="Pomodoro progress"
      ></progress>
      <div class="button-row">
        <button type="button" on:click={startTimer} disabled={timerStatus === "running"}>
          Start
        </button>
        <button
          type="button"
          class="secondary"
          on:click={pauseTimer}
          disabled={timerStatus !== "running"}
        >
          Pause
        </button>
        <button type="button" class="secondary" on:click={resetTimer}>Reset</button>
      </div>
      <small>Status: {timerStatus}</small>
    </div>
  </section>

  <section class="settings-section" aria-labelledby="settings-heading">
    <h2 id="settings-heading">Settings</h2>

    <form class="settings-form" on:submit|preventDefault={applyTimerSettings}>
      <div class="settings-grid">
        <label class="settings-field" for="work-duration">
          Work minutes
          <input
            id="work-duration"
            type="number"
            min="1"
            max="180"
            step="1"
            bind:value={draftWorkDurationMinutes}
          />
        </label>

        <label class="settings-field" for="break-duration">
          Break minutes
          <input
            id="break-duration"
            type="number"
            min="1"
            max="180"
            step="1"
            bind:value={draftBreakDurationMinutes}
          />
        </label>
      </div>

      <label class="checkbox-row" for="timer-notifications">
        <input
          id="timer-notifications"
          type="checkbox"
          bind:checked={draftTimerNotificationsEnabled}
        />
        Timer completion notifications
      </label>

      <button type="submit">Apply settings</button>

      {#if settingsErrorMessage}
        <p class="error">{settingsErrorMessage}</p>
      {/if}
    </form>
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

  <section class="shortcut-section" aria-labelledby="shortcut-heading">
    <h2 id="shortcut-heading">Global Shortcut</h2>
    <p>Shortcut: {workLogShortcut}</p>
    <p>Status: {shortcutStatus}</p>
    <div class="button-row">
      <button type="button" on:click={registerWorkLogShortcut} disabled={shortcutRegistered}>
        Register
      </button>
      <button
        type="button"
        class="secondary"
        on:click={unregisterWorkLogShortcut}
        disabled={!shortcutRegistered}
      >
        Unregister
      </button>
    </div>

    {#if shortcutErrorMessage}
      <p class="error">{shortcutErrorMessage}</p>
    {/if}
  </section>
</main>
