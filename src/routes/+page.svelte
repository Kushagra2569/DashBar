<script>
  import "../app.css";
  import { onMount, tick } from "svelte";
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import { openPath } from "@tauri-apps/plugin-opener";

  /** @type {HTMLInputElement | null} */
  let input = null;
  let query = $state("");
  let results = $state([]);
  let selectedIndex = $state(0);

  // Constants
  const ITEM_HEIGHT = 56; // approximate height of results item
  const BASE_HEIGHT = 60; // search bar height
  const ITEM_MARGIN = 8; // margin between search bar and results

  async function updateWindowSize(resultCount) {
    let newHeight = BASE_HEIGHT;
    if (resultCount > 0) {
      newHeight += ITEM_MARGIN + resultCount * ITEM_HEIGHT;
    }
    // Add a bit of padding for container borders/shadows
    if (resultCount > 0) newHeight += 20;

    await getCurrentWindow().setSize(new LogicalSize(800, newHeight));
  }

  /** @param {KeyboardEvent} event */
  async function handleKeydown(event) {
    if (event.key === "Escape") {
      await getCurrentWindow().hide();
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      selectedIndex = (selectedIndex + 1) % results.length;
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      selectedIndex = (selectedIndex - 1 + results.length) % results.length;
    } else if (event.key === "Enter") {
      if (results[selectedIndex]) {
        await openPath(results[selectedIndex].path);
        await getCurrentWindow().hide();
      }
    }
  }

  async function handleInput() {
    if (query.trim() === "") {
      results = [];
      await updateWindowSize(0);
      return;
    }
    try {
      results = await invoke("search_apps", { query });
      selectedIndex = 0;
      await updateWindowSize(results.length);
    } catch (e) {
      console.error(e);
      results = [];
      await updateWindowSize(0);
    }
  }

  onMount(() => {
    // Focus input when window gains focus
    window.addEventListener("focus", () => {
      input?.focus();
      query = "";
      results = [];
      updateWindowSize(0);
    });

    // Also focus explicitly if we mount (though usually hidden at start)
    input?.focus();
    updateWindowSize(0);
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<main
  class="w-full flex flex-col items-center justify-center bg-transparent overflow-hidden"
>
  <div
    class="w-full h-[60px] bg-[#1e1e1e]/90 backdrop-blur-xl rounded-lg border border-[#3e3e3e] shadow-2xl flex items-center px-4 overflow-hidden shrink-0"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="text-gray-400 mr-4"
      ><circle cx="11" cy="11" r="8"></circle><line
        x1="21"
        y1="21"
        x2="16.65"
        y2="16.65"
      ></line></svg
    >
    <input
      bind:this={input}
      type="text"
      placeholder="Search..."
      class="bg-transparent w-full text-2xl text-white placeholder-gray-500 outline-none h-full font-light"
      autocomplete="off"
      spellcheck="false"
      bind:value={query}
      oninput={handleInput}
    />
  </div>

  {#if results.length > 0}
    <div
      class="w-full mt-2 bg-[#1e1e1e]/90 backdrop-blur-xl rounded-lg border border-[#3e3e3e] shadow-xl overflow-hidden flex-col flex"
    >
      {#each results as result, index}
        <button
          class="w-full text-left px-4 py-3 text-white hover:bg-[#3e3e3e] transition-colors flex items-center {index ===
          selectedIndex
            ? 'bg-[#3e3e3e]'
            : ''}"
          onclick={async () => {
            await openPath(result.path);
            await getCurrentWindow().hide();
          }}
        >
          <div
            class="w-8 h-8 mr-3 bg-gray-700 rounded flex items-center justify-center text-xs"
          >
            APP
          </div>
          <span class="text-lg">{result.name}</span>
        </button>
      {/each}
    </div>
  {/if}
</main>

<style>
  @reference "tailwindcss";
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: transparent;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: transparent;
    }
  }
</style>
