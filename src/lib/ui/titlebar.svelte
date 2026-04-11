<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import Add from "$lib/icons/add.svelte";
  import Close from "$lib/icons/close.svelte";

  export let tabItems: any[];
  export let currentTab: string;
  export let showCloseBth = true;

  export let selectedTab = "manga";

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

  function tabSelect(item: string) {
    selectedTab = item;
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
    <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="tabs">
      {#each tabItems as item}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
              class="items flex flex-justify-between flex-align-center {currentTab ==
              item.key
                  ? 'enable-border'
                  : ''}"
              on:click={(_) => setItem(item.key)}
          >
              <div>
                  {item.name.split("/").at(-1)}
              </div>
              {#if showCloseBth}
                  <button
                      class="close-btn"
                      on:click={(e) => closeProject(e, item.key)}
                  >
                      <Close />
                  </button>
              {/if}
          </div>
      {/each}
      {#if showCloseBth}
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

  .window-controls {
    display: flex;
  }

  .tabs {
    display: flex;
    color: var(--primary-color);
    background: var(--accent);

    height: 37px;
    padding: 1px;
  }

  .title-bar {
    display: flex;
    justify-content: space-between;
    height: 100%;
  }

  .hvrBgDark {
    width: 42px;
    border: none;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--primary-color);
  }

  .hvrBgDark:hover {
    background: #8585853b;
  }

  .btn-close {
    width: 45px;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    fill: var(--primary-color);
  }

  .btn-close:hover {
    background: #bf616a;
  }

  .enable-border {
      border-bottom: 2px solid var(--primary-color);
  }

  .items {
      padding: 10px 15px 10px 15px;
      width: fit-content;
  }

  .add-item {
      padding: 10px 15px 10px 15px;
  }

  .add-item:hover {
      background: var(--hover-color);
  }

  .items:hover {
      background: var(--hover-color);
  }

  .close-btn {
      border: none;
      background: transparent;
      color: var(--text-color);
  }

  .close-btn:hover {
      cursor: pointer;
  }
</style>
