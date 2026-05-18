<script lang="ts">
  export let historyItem;

  let expanded = false;

  function toogleExpanded() {
    expanded = !expanded;
  }
</script>

{#if historyItem["role"] === "user"}
  <pre
    class="text-wrap-wrap hover w-90 full-border chat-item"
    style="margin-left: auto; padding: 5px;">{historyItem["content"]}</pre>
{:else if historyItem["role"] === "tool"}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    style="width: calc(100% - 12px); padding: 5px;"
    class="text-wrap-wrap hover full-border chat-item"
    on:click={(_) => toogleExpanded()}
  >
    <pre>{historyItem["name"]}</pre>
    {#if expanded}
      <pre>{historyItem["content"]}</pre>
    {/if}
  </div>
{:else if historyItem["role"] === "system"}
  <pre
    class="text-wrap-wrap hover full-border chat-item"
    style="width: calc(100% - 12px); padding: 5px;">{historyItem[
      "content"
    ]}</pre>
{:else}
  <pre
    class="text-wrap-wrap hover w-90 full-border chat-item"
    style="padding: 5px;">{historyItem["content"]}</pre>
{/if}

<style>
  .chat-item {
    padding: 4px;
    border-radius: 2px;
  }
</style>
