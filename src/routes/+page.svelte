<script>
  import "../app.css";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  /** @type {HTMLInputElement | null} */
  let input = null;

  async function handleKeydown(event) {
    if (event.key === "Escape") {
      await getCurrentWindow().hide();
    }
  }

  onMount(() => {
    // Focus input when window gains focus
    window.addEventListener("focus", () => {
      input?.focus();
    });

    // Also focus explicitly if we mount (though usually hidden at start)
    input?.focus();
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<main class="h-screen w-screen flex items-center justify-center bg-transparent">
  <div
    class="w-full h-full bg-[#1e1e1e]/90 backdrop-blur-xl rounded-lg border border-[#3e3e3e] shadow-2xl flex items-center px-4 overflow-hidden"
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
    />
  </div>
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
