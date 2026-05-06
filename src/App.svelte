<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = "";
  let greeting = "";
  let errorMessage = "";

  async function greet() {
    greeting = "";
    errorMessage = "";

    try {
      greeting = await invoke<string>("greet", { name });
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<main class="container">
  <section class="intro">
    <h1>Tauri + Svelte</h1>
    <p>Frontend は Svelte、デスクトップ側の処理は Rust で動く最小構成です。</p>
  </section>

  <form class="greet-form" on:submit|preventDefault={greet}>
    <label for="name">Name</label>
    <div class="row">
      <input id="name" bind:value={name} placeholder="Enter a name" />
      <button type="submit">Greet</button>
    </div>
  </form>

  {#if greeting}
    <p class="greeting">{greeting}</p>
  {/if}

  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}
</main>
