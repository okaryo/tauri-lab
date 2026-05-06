<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type Greeting = {
    message: string;
    nameLength: number;
  };

  let name = "";
  let greeting: Greeting | null = null;
  let errorMessage = "";

  async function greet() {
    greeting = null;
    errorMessage = "";

    try {
      greeting = await invoke<Greeting>("greet", { name });
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
    <section class="greeting" aria-label="Greeting from Rust">
      <p>{greeting.message}</p>
      <dl>
        <div>
          <dt>Name length</dt>
          <dd>{greeting.nameLength}</dd>
        </div>
      </dl>
    </section>
  {/if}

  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}
</main>
