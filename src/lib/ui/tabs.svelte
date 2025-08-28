<script lang="ts">
  import Add from "$lib/icons/add.svelte";
  import Close from "$lib/icons/close.svelte";

  export let tabItems: any[];
  export let currentTab: string;
  export let showCloseBth = true;

  function setItem(key: string) {
    currentTab = key;
  }

  function closeProject(e: any, key: string) {
    e.preventDefault();
    tabItems = tabItems.filter((value, index) => value.key !== key);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="flex tabs">
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
        {item.name}
      </div>
      {#if showCloseBth}
        <button class="close-btn" on:click={(e) => closeProject(e, item.key)}>
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

<style>
  .enable-border {
    border-bottom: 2px solid var(--primary-color);
  }
  .tabs {
    height: 37px;
    padding: 1px;
    border-bottom: 1px solid var(--border-color);
  }

  .items {
    padding: 10px 15px 10px 15px;
    width: 100%;
    max-width: 20vw;
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
