<script lang="ts">
  export let currentProject: string;
  import { invoke } from "@tauri-apps/api/core";
  import ModelDropDown from "./modelDropDown.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let stagedFiles: any[] = [];

  let fileDiff = "";

  let selectedModel = "";

  let llmResponse = "";

  let history: any[] = [];

  let userInput = "";

  let branchName = "";

  $: {
    if (currentProject != "add" && currentProject != "") {
      invoke("get_staged_files", {
        location: currentProject,
      }).then(function (data: any) {
        stagedFiles = data;
      });
    }

    listen("get-history", function (data: any) {
      history = data.payload;
    });
  }

  onMount(() => {
    invoke("get_current_branch_name", {
      location: currentProject,
    }).then(function (data: any) {
      branchName = data;
    });
  });

  function getChanges(file: string) {
    invoke("get_file_diff", {
      location: currentProject,
      file: file,
    }).then(function (data: any) {
      fileDiff = data;
    });
  }

  function sendStartMessage() {
    let message = [];
    message.push(
      "Create commit message for the following file/files which contains the changes " +
        stagedFiles.join(", ")
    );
    message.push("you can use the get_file_diff to get the changes of a file");
    message.push("The current repo you are using is " + currentProject);
    message.push(
      "create seperate commit messages for each file and summarize into one single commit message ideally 1 to 2 lines"
    );

    message.push("The current branch name is " + branchName);

    console.log("userInput: " + userInput);

    if (userInput != "") {
      message.push(userInput);
      userInput = "";
    }
    console.log("message: " + message);

    invoke("send_message", {
      model: selectedModel,
      messages: message,
      history: history,
    }).then(function (data: any) {
      llmResponse = data;
    });
  }

  function sendMessage() {
    if (userInput != "") {
      let message = [];
      message.push(userInput);
      userInput = "";
      invoke("send_message", {
        model: selectedModel,
        messages: message,
        history: history,
      }).then(function (data: any) {
        llmResponse = data;
      });
    }
  }

  function printFileChangeLine(line: string) {
    if (line != "") {
      let obj = JSON.parse(line);
      if (obj["from_no"] == obj["to_no"]) {
        return (
          obj["change_type"] + ": " + obj["from_no"] + ": " + obj["content"]
        );
      } else if (obj["from_no"] == "") {
        return obj["change_type"] + ": " + obj["to_no"] + ": " + obj["content"];
      } else if (obj["to_no"] == "") {
        return (
          obj["change_type"] + ": " + obj["from_no"] + ": " + obj["content"]
        );
      } else {
        return (
          obj["change_type"] +
          ": " +
          obj["from_no"] +
          "->" +
          obj["to_no"] +
          ": " +
          obj["content"]
        );
      }
    }
  }
</script>

<div class="flex flex-row w-100 h-100">
  <div class="w-50 h-100">
    <div class="chat-header">
      <ModelDropDown bind:selectedModel />
    </div>
    <div class="main-scrollbar chat-response">
      {#if history.length > 0}
        {#each history as historyItem}
          {#if historyItem["role"] === "user"}
            <pre
              class="text-wrap-wrap hover w-90 full-border chat-item"
              style="margin-left: auto;">{historyItem["content"]}</pre>
          {:else}
            <pre
              class="text-wrap-wrap hover w-90 full-border chat-item">{historyItem[
                "content"
              ]}</pre>
          {/if}
        {/each}
      {/if}
    </div>
    <div class="chat-footer">
      <div class="flex endpoint-input top-border h-100">
        {#if history.length > 0}
          <form class="w-100 flex">
            <input class="w-100 input" bind:value={userInput} />
            <button class="btn" on:click={(_) => sendMessage()}>Send</button>
          </form>
        {:else}
          <form class="w-100 flex">
            <input class="w-100 input" bind:value={userInput} />
            <button
              class="btn w-100"
              style="border-top: none;border-right: none;"
              on:click={(_) => sendStartMessage()}>Start</button
            >
          </form>
        {/if}
      </div>
    </div>
  </div>
  <div class="w-50 h-100">
    <div class="h-50 left-border bottom-border main-scrollbar">
      {#each fileDiff.split("\n") as line}
        <pre class="hover margin-0 text-wrap-wrap">{printFileChangeLine(
            line
          )}</pre>
      {/each}
    </div>
    <div class="h-50 main-scrollbar left-border">
      {#each stagedFiles as file}
        <button
          on:click={(_) => getChanges(file)}
          class="commit-file-path hover"
        >
          {file}
        </button>
      {/each}
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

  .chat-header {
    height: 37px;
  }

  .chat-response {
    height: calc(100% - 74px);
  }

  .chat-footer {
    height: 37px;
  }

  .chat-item {
    padding: 4px;
    border-radius: 2px;
  }
</style>
