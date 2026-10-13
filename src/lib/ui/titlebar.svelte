<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import Add from "$lib/icons/add.svelte";
  import Close from "$lib/icons/close.svelte";

  export let tabItems: any[];
  export let currentTab: string;
  export let showCloseBtn = true;

  const appWindow = getCurrentWindow();

  function onClickReduce() {
    appWindow.minimize();
  }

  function onClickMaximize() {
    appWindow.toggleMaximize();
  }

  function onClickClose() {
    appWindow.close();
  }

  function setItem(key: string) {
      currentTab = key;
  }

  function closeProject(e: any, key: string) {
      e.preventDefault();
      tabItems = tabItems.filter((value, index) => value.key !== key);
      setTimeout(() => {
          if (tabItems.length > 0) {
              setItem(tabItems[tabItems.length - 1].key);
          } else {
              setItem("add");
          }
      }, 0);
  }
</script>

<div data-tauri-drag-region class="title-bar">
  <div class="brand">
    <span class="brand-mark" aria-hidden="true">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        height="20"
        width="20"
        fill="none"
        stroke="currentColor"
        stroke-width="1.6"
        stroke-linejoin="round"
      >
        <path d="M12 3 L21.5 20 L3 20 Z" />
        <path d="M12 9 L17 20 L7 20 Z" />
      </svg>
    </span>
    <span class="brand-name">PROJECT<span class="accent">·</span>MAN</span>
  </div>

    <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div data-tauri-drag-region class="tabs">
      {#each tabItems as item}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
              class="items flex flex-justify-between flex-align-center {currentTab ==
              item.key
                  ? 'enable-border'
                  : ''}"
              on:click={(_) => setItem(item.key)}
          >
              <div class="tab-label">
                  {item.name.split("/").at(-1)}
              </div>
              {#if showCloseBtn}
                  <button
                      class="close-btn"
                      on:click={(e) => closeProject(e, item.key)}
                  >
                      <Close />
                  </button>
              {/if}
          </div>
      {/each}
      {#if showCloseBtn}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
              on:click={(_) => setItem("add")}
              class="add-item flex flex-justify-center flex-align-center"
          >
              <div><Add /></div>
          </div>
      {/if}
  </div>
  <div class="window-controls">
    <button
      aria-label="Reduce button"
      class="hvrBgDark"
      on:click={onClickReduce}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 16.933333 16.933334"
        height="13"
        width="13"
      >
        <g>
          <path
            d="M 1.8515625,7.8046875 A 0.66145998,0.66145998 0 0 0 1.1914062,8.4667969 0.66145998,0.66145998 0 0 0 1.8515625,9.1289063 H 15.082031 A 0.66145998,0.66145998 0 0 0 15.742188,8.4667969 0.66145998,0.66145998 0 0 0 15.082031,7.8046875 Z"
            id="path888"
          ></path>
        </g>
      </svg>
    </button>

    <button
      aria-label="maximize button"
      class="hvrBgDark"
      on:click={onClickMaximize}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 16.933333 16.933334"
        height="13"
        width="13"
      >
        <g>
          <path
            id="path841"
            d="M 3.1757812 1.1914062 C 2.087675 1.1914062 1.1914062 2.087675 1.1914062 3.1757812 L 1.1914062 13.757812 C 1.1914068 14.845918 2.0876754 15.742188 3.1757812 15.742188 L 13.757812 15.742188 C 14.845918 15.742187 15.742187 14.845918 15.742188 13.757812 L 15.742188 3.1757812 C 15.742188 2.0876754 14.845918 1.1914068 13.757812 1.1914062 L 3.1757812 1.1914062 z M 3.1757812 2.5136719 L 13.757812 2.5136719 C 14.13096 2.5136721 14.419922 2.8026342 14.419922 3.1757812 L 14.419922 13.757812 C 14.419922 14.130959 14.130959 14.419922 13.757812 14.419922 L 3.1757812 14.419922 C 2.8026342 14.419922 2.5136721 14.13096 2.5136719 13.757812 L 2.5136719 3.1757812 C 2.5136719 2.802634 2.802634 2.5136719 3.1757812 2.5136719 z "
          ></path>
        </g>
      </svg>
    </button>

    <button
      aria-label="close button"
      class="btn-close"
      on:click={onClickClose}
      on:keypress={onClickClose}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 16.933333 16.933334"
        height="13"
        width="13"
      >
        <g>
          <path
            id="path839"
            d="M 2.4863281 1.8535156 A 0.66145998 0.66145998 0 0 0 2.0449219 2.0449219 A 0.66145998 0.66145998 0 0 0 2.0449219 2.9804688 L 7.53125 8.4648438 L 2.0449219 13.951172 A 0.66145998 0.66145998 0 0 0 2.0449219 14.886719 A 0.66145998 0.66145998 0 0 0 2.9804688 14.886719 L 8.4648438 9.4023438 L 13.951172 14.886719 A 0.66145998 0.66145998 0 0 0 14.886719 14.886719 A 0.66145998 0.66145998 0 0 0 14.886719 13.951172 L 9.4023438 8.4648438 L 14.886719 2.9804688 A 0.66145998 0.66145998 0 0 0 14.886719 2.0449219 A 0.66145998 0.66145998 0 0 0 13.951172 2.0449219 L 8.4648438 7.53125 L 2.9804688 2.0449219 A 0.66145998 0.66145998 0 0 0 2.4863281 1.8535156 z "
          ></path>
        </g>
      </svg>
    </button>
  </div>
</div>

<style>
  path {
    fill: var(--text-color);
  }

  .title-bar {
    display: flex;
    align-items: stretch;
    justify-content: space-between;
    height: 100%;
    background: var(--background-color);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 18px;
    border-right: 1px solid var(--border-color);
    color: var(--text-color);
    flex-shrink: 0;
  }

  .brand-mark {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border: 1px solid var(--border-strong);
    color: var(--primary-color);
    background: var(--surface-color);
  }

  .brand-name {
    font-size: 12px;
    letter-spacing: 0.18em;
    font-weight: 600;
  }

  .window-controls {
    display: flex;
    align-items: stretch;
    border-left: 1px solid var(--border-color);
  }

  .tabs {
    display: flex;
    align-items: center;
    color: var(--muted-color);
    gap: 6px;
    flex: 1;
    min-width: 0;
    overflow-x: auto;
    scrollbar-width: none;
    padding: 0 12px;
  }

  .tabs::-webkit-scrollbar {
    display: none;
  }

  .hvrBgDark {
    width: 46px;
    border: none;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--muted-color);
    transition: background 0.15s ease, fill 0.15s ease;
  }

  .hvrBgDark:hover {
    background: var(--hover-color);
    fill: var(--text-color);
  }

  .btn-close {
    width: 50px;
    border: none;
    border-left: 1px solid var(--border-color);
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--muted-color);
    transition: background 0.15s ease, fill 0.15s ease;
  }

  .btn-close:hover {
    background: #d84a4a;
    fill: #ffffff;
  }

  .enable-border {
    background: var(--hover-color);
    border-color: var(--border-strong);
    color: var(--text-color);
  }

  .items {
    display: flex;
    gap: 8px;
    padding: 6px 10px 6px 14px;
    border: 1px solid transparent;
    cursor: pointer;
    color: var(--muted-color);
    text-transform: uppercase;
    font-size: 11px;
    letter-spacing: 0.1em;
    white-space: nowrap;
    flex-shrink: 0;
    transition: background 0.15s ease, border-color 0.15s ease,
      color 0.15s ease;
  }

  .items:hover {
    color: var(--text-color);
    border-color: var(--border-color);
  }

  .add-item {
    padding: 6px 10px;
    border: 1px solid transparent;
    cursor: pointer;
    color: var(--muted-color);
    transition: background 0.15s ease, border-color 0.15s ease,
      color 0.15s ease;
    flex-shrink: 0;
  }

  .add-item:hover {
    background: var(--hover-color);
    border-color: var(--border-color);
    color: var(--text-color);
  }

  .close-btn {
    border: none;
    background: transparent;
    color: var(--muted-color);
    display: flex;
    align-items: center;
    padding: 0;
  }

  .close-btn :global(svg) {
    height: 14px;
    width: 14px;
    fill: currentColor;
  }

  .close-btn:hover {
    cursor: pointer;
    color: var(--text-color);
  }

  .add-item :global(svg) {
    height: 16px;
    width: 16px;
  }
</style>
