<script lang="ts">
  export let historyItem;

  let expanded = false;

  function toogleExpanded() {
    expanded = !expanded;
  }
</script>

{#if historyItem["role"] === "user"}
  <pre
    class="text-wrap-wrap chat-item chat-item-user">{historyItem["content"]}</pre>
{:else if historyItem["role"] === "tool"}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="chat-item chat-item-tool" on:click={(_) => toogleExpanded()}>
    <div class="tool-head">
      <span class="tag">Tool</span>
      <pre class="tool-name">{historyItem["name"]}</pre>
      <span class="tool-toggle">{expanded ? "−" : "+"}</span>
    </div>
    {#if expanded}
      <pre class="tool-body">{historyItem["content"]}</pre>
    {/if}
  </div>
{:else if historyItem["role"] === "system"}
  <pre class="text-wrap-wrap chat-item chat-item-system">{historyItem[
      "content"
    ]}</pre>
{:else}
  <pre class="text-wrap-wrap chat-item chat-item-assistant"
    >{historyItem["content"]}</pre
  >
{/if}

<style>
  .chat-item {
    padding: 12px 14px;
    border-radius: 0px;
    border: 1px solid var(--border-color);
    background: var(--surface-color);
    margin: 0;
    font-family: inherit;
    font-size: 12.5px;
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .chat-item-user {
    margin-left: auto;
    max-width: 90%;
    border-color: var(--primary-color);
    background: var(--hover-color);
  }

  .chat-item-assistant {
    border-left: 2px solid var(--border-strong);
  }

  .chat-item-system {
    border-left: 2px solid var(--muted-color);
    color: var(--muted-color);
    font-size: 12px;
  }

  .chat-item-tool {
    cursor: pointer;
    border-left: 2px solid var(--primary-color);
  }

  .tool-head {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .tool-name {
    margin: 0;
    font-family: inherit;
    font-size: 12px;
    color: var(--text-color);
  }

  .tool-toggle {
    margin-left: auto;
    color: var(--muted-color);
  }

  .tool-body {
    margin: 10px 0 0 0;
    padding-top: 10px;
    border-top: 1px solid var(--border-color);
    font-family: inherit;
    font-size: 12px;
    color: var(--muted-color);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .tag {
    padding: 2px 6px;
    border: 1px solid var(--border-color);
    color: var(--primary-color);
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }
</style>
