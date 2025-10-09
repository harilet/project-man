<script lang="ts">
  export let historyItem;

  let expanded = false;

  function toogleExpanded() {
    expanded = !expanded;
  }

  function getToolOutput(tool_output: any) {
    let toolOutput: any[] = [];
    tool_output = "[" + tool_output.replaceAll("\n", ",") + "]";
    console.log(tool_output);
    let data = JSON.parse(tool_output);
    console.log(data);
    data.forEach((element: any) => {
      toolOutput = [...toolOutput, element];
    });
    console.log(toolOutput);
    return toolOutput;
  }

  function printFileChangeLine(line: any) {
    if (line["from_no"] == line["to_no"]) {
      return (
        line["change_type"] + ": " + line["from_no"] + ": " + line["content"]
      );
    } else if (line["from_no"] == "") {
      return (
        line["change_type"] + ": " + line["to_no"] + ": " + line["content"]
      );
    } else if (line["to_no"] == "") {
      return (
        line["change_type"] + ": " + line["from_no"] + ": " + line["content"]
      );
    } else {
      return (
        line["change_type"] +
        ": " +
        line["from_no"] +
        "->" +
        line["to_no"] +
        ": " +
        line["content"]
      );
    }
  }

  function getChangeLineColor(line: any) {
    if (line["change_type"] == "+") {
      return "#00be7e4d";
    } else if (line["change_type"] == "-") {
      return "#ff1d5733";
    } else {
      return "";
    }
  }
</script>

{#if historyItem["role"] === "user"}
  <pre
    class="text-wrap-wrap hover w-90 full-border chat-item"
    style="margin-left: auto;">{historyItem["content"]}</pre>
{:else if historyItem["role"] === "tool"}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    style="width: calc(100% - 10px);"
    class="text-wrap-wrap hover full-border chat-item"
    on:click={(_) => toogleExpanded()}
  >
    <pre>Tool {historyItem["content"]["tool_name"]} called 
{historyItem["content"]["tool_input"]}</pre>
    {#if expanded}
      {#each getToolOutput(historyItem["content"]["tool_output"]) as line}
        <pre
          style="background-color: {getChangeLineColor(line)};"
          class="m-0">{printFileChangeLine(line)}</pre>
      {/each}
    {/if}
  </div>
{:else if historyItem["role"] === "system"}
  <pre
    class="text-wrap-wrap hover full-border chat-item"
    style="width: calc(100% - 10px);">{historyItem["content"]}</pre>
{:else}
  <pre class="text-wrap-wrap hover w-90 full-border chat-item">{historyItem[
      "content"
    ]}</pre>
{/if}

<style>
  .chat-item {
    padding: 4px;
    border-radius: 2px;
  }
</style>
