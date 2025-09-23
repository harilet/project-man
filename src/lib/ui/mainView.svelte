<script lang="ts">
  export let currentProject: string;
  import { invoke } from "@tauri-apps/api/core";
  import ModelDropDown from "./modelDropDown.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import LlmSetting from "./llmSetting.svelte";
  import ChatItem from "./chatItem.svelte";

  let stagedFiles: any[] = [];

  let fileDiff = "";

  let selectedModel = "";

  let llmResponse = "";

  let history: any[] = [];

  let chat: any[] = [];

  let userInput = "";

  let branchName = "";

  let llmSettingDialog: HTMLDialogElement;

  $: {
    if (currentProject != "add" && currentProject != "") {
      invoke("get_staged_files", {
        location: currentProject,
      }).then(function (data: any) {
        stagedFiles = data;
      });
    }
  }

  onMount(() => {
    invoke("get_current_branch_name", {
      location: currentProject,
    }).then(function (data: any) {
      branchName = data;
    });

    llmSettingDialog.addEventListener("click", function (event) {
      var rect = llmSettingDialog.getBoundingClientRect();
      var isInDialog =
        rect.top <= event.clientY &&
        event.clientY <= rect.top + rect.height &&
        rect.left <= event.clientX &&
        event.clientX <= rect.left + rect.width;
      if (!isInDialog) {
        llmSettingDialog.close();
      }
    });

    listen("get-history", function (data: any) {
      history = data.payload;
      console.log(history);
    });

    listen("tool-call", function (data: any) {
      let message: string = data.payload;
      chat = [...chat, { role: "tool", content: message }];
      console.log(message);
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
    message.push(
      "The current repo you are using is " +
        currentProject.replaceAll("\\", "/")
    );
    message.push(
      "create seperate commit messages for each file and summarize into one single commit message ideally 1 to 2 lines"
    );

    message.push("The current branch name is " + branchName);

    if (userInput != "") {
      message.push(userInput);
      userInput = "";
    }

    let chat_history = message.map(function (value) {
      return { role: "user", content: value };
    });

    chat = [...chat, ...chat_history];
    console.log(chat);

    invoke("send_message", {
      model: selectedModel,
      messages: message,
      history: history,
    }).then(function (data: any) {
      llmResponse = data;
      chat = [...chat, { role: "assistant", content: data }];
    });
  }

  function sendMessage() {
    if (userInput != "") {
      let message = [];
      message.push(userInput);
      userInput = "";
      let chat_history = message.map(function (value) {
        return { role: "user", content: value };
      });

      chat = [...chat, ...chat_history];
      console.log(chat);
      invoke("send_message", {
        model: selectedModel,
        messages: message,
        history: history,
      }).then(function (data: any) {
        llmResponse = data;
        chat = [...chat, { role: "assistant", content: data }];
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

  function getChangeLineColor(line: string) {
    if (line != "") {
      let obj = JSON.parse(line);
      if (obj["change_type"] == "+") {
        return "#00be7e4d";
      } else if (obj["change_type"] == "-") {
        return "#ff1d5733";
      } else {
        return "";
      }
    }
  }

  function openDialog() {
    if (llmSettingDialog.open) {
      llmSettingDialog.close();
    } else {
      llmSettingDialog.showModal();
    }
  }
</script>

<dialog bind:this={llmSettingDialog} on:close>
  <LlmSetting bind:selectedModel />
</dialog>

<div class="flex flex-row w-100 h-100">
  <div class="w-50 h-100">
    <div class="chat-header">
      <button class="btn" on:click={(_) => openDialog()}>
        {#if selectedModel == ""}
          LLM Settings
        {:else}
          {selectedModel}
        {/if}
      </button>
    </div>
    <div class="main-scrollbar chat-response">
      {#if chat.length > 0}
        {#each chat as historyItem}
          <ChatItem {historyItem} />
        {/each}
      {/if}
    </div>
    <div class="chat-footer">
      <div class="flex endpoint-input top-border h-100">
        {#if chat.length > 0}
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
        <pre
          style="background-color: {getChangeLineColor(line)};"
          class="hover m-0 text-wrap-wrap">{printFileChangeLine(
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
</style>
