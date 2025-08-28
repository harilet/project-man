<script lang="ts">
  export let currentProject: string;
  import { invoke } from "@tauri-apps/api/core";
  import ModelDropDown from "./modelDropDown.svelte";
  import { listen } from "@tauri-apps/api/event";

  let stagedFiles: any[] = [];

  let fileDiff = "";

  let selectedModel = "";

  let llmResponse = "";

  let history :any[]= [];

  $: {
    if (currentProject != "add" && currentProject != "") {
      invoke("get_staged_files", {
        location: currentProject,
      }).then(function (data: any) {
        stagedFiles = data;
      });
    }

    listen("get-history", function (data: any) {
      console.log(data);
      history = data.payload;
    });
  }

  function getChanges(file: string) {
    invoke("get_file_diff", {
      location: currentProject,
      file: file,
    }).then(function (data: any) {
      fileDiff = data;
    });
  }

  function sendMessage() {
    let message = [];
    message.push(
      "Create commit message for the following file/files contains changes" +
        stagedFiles.join(", ")
    );
    message.push("you can use the get_file_diff to get the changes of a file");
    message.push("The current repo you are using is " + currentProject);
    message.push(
      "create seperate commit messages for each file and summarize into one single commit message ideally 1 to 2 lines"
    );
    console.log(currentProject);
    invoke("send_message", {
      model: selectedModel,
      messages: message,
      history: history,
    }).then(function (data: any) {
      llmResponse = data;
    });
  }
</script>

<div class="flex flex-row w-100 h-100">
  <div class="w-50 h-100">
    <div class="h-50 bottom-border main-scrollbar">
      <pre>{fileDiff}</pre>
    </div>
    <div class="h-50 main-scrollbar">
      {#each stagedFiles as file}
        <button on:click={(_) => getChanges(file)} class="commit-file-path">
          {file}
        </button>
      {/each}
    </div>
  </div>
  <div class="w-50 h-100 left-border">
    <div class="chat-header">
      <ModelDropDown bind:selectedModel />
    </div>
    <div class="main-scrollbar chat-response">
      {#if history.length>0}
      {#each history as historyItem}
        <pre class="text-wrap-wrap">{historyItem["content"]}</pre>
      {/each}
        
      {/if}
      
      <pre class="text-wrap-wrap">{llmResponse}</pre>
    </div>
    <div class="chat-footer">
      <button class="btn" on:click={(_) => sendMessage()}>send</button>
    </div>
  </div>
</div>

<style>
  .commit-file-path {
    padding: 6px;
    cursor: pointer;
    border-radius: 4px;
    background: transparent;
    color: var(----on-background-colorolor);
    border: none;
    width: -webkit-fill-available;
    text-align: start;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .commit-file-path:hover {
    background: var(--hover-color);
  }

  .chat-header {
    height: 37px;
  }

  .chat-response {
    height: calc(100% - 74px);
  }

  .chat-footer {
    height: 37px;
  }
</style>
