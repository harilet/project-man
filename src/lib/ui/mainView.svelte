<script lang="ts">
  export let currentProject: string;
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import LlmSetting from "./llmSetting.svelte";
  import ChatItem from "./chatItem.svelte";
  import SavedMessages from "./savedMessages.svelte";
  import GitView from "./gitView.svelte";
  import { listen } from "@tauri-apps/api/event";

  let selectedModel = "";

  let chat: any[] = [];

  let userInput = "";

  let llmSettingDialog: HTMLDialogElement;
  let logDialog: HTMLDialogElement;

  let view = "chat";

  let isOpenSavedMessages = false;

  let serverLive = false;
  let error: any[] = [];

  onMount(() => {
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

    logDialog.addEventListener("click", function (event) {
      var rect = logDialog.getBoundingClientRect();
      var isInDialog =
        rect.top <= event.clientY &&
        event.clientY <= rect.top + rect.height &&
        rect.left <= event.clientX &&
        event.clientX <= rect.left + rect.width;
      if (!isInDialog) {
        logDialog.close();
      }
    });

    listen("ollama-server-status", function (data: any) {
      let message: string = data.payload;
      if (message == "live") {
        serverLive = true;
      } else {
        serverLive = false;
      }
    });

    listen("tool-execution", function (data: any) {
      let message: any = data.payload;
      message = {
        role: "tool",
        content: message["tool_output"],
        name: message["tool_name"],
      };
      chat = [...chat, message];
    });

    listen("app-error", function (data: any) {
      let message: string = data.payload;
      error = [...error, message];
    });
  });

  async function sendMessage() {
    if (userInput != "") {
      let message: any;
      if (chat.length == 0) {
        message = {
          role: "system",
          content: `You are a coding assistant with access to a local codebase.
## Project git repository location
${currentProject}

## Only use the following tools to interact with the codebase. Do not use any other tools or commands that directly interact with the file system or git. Always use these tools to interact with the codebase and git.
- read_repo_file
- list_dir
- search_code
- read_multiple_files
- get_staged_diff

## Guidelines
- Always explore before reading: use list_dir or search_code to locate relevant files first
- Prefer read_multiple_files over repeated read_repo_file calls
- Search before reading: use search_code to find where something is defined or used
- Never guess file paths; verify them with list_dir or search_code first"
- Less word faster and more work
`,
        };
        chat = [...chat, message];
      }

      message = {
        role: "user",
        content: userInput,
      };
      userInput = "";
      chat = [...chat, message];

      invoke("send_message", {
        message: [message],
        history: chat,
      }).then(function (data: any) {
        chat = [...chat, data];
      });
    }
  }

  function openSettingDialog() {
    if (llmSettingDialog.open) {
      llmSettingDialog.close();
    } else {
      llmSettingDialog.showModal();
    }
  }

  function openLogDialog() {
    if (logDialog.open) {
      logDialog.close();
    } else {
      logDialog.showModal();
    }
  }

  function changeView(viewChange: string) {
    view = viewChange;
  }

  function openSavedMessages() {
    isOpenSavedMessages = !isOpenSavedMessages;
  }
</script>

<dialog bind:this={llmSettingDialog} on:close>
  <LlmSetting bind:selectedModel />
</dialog>

<dialog bind:this={logDialog} on:close>
  <div class="dialog-pad">
    <div class="label dialog-title">Application Logs</div>
    {#each error as errorMessage}
      <div class="log-item full-border">
        {errorMessage}
      </div>
    {/each}
  </div>
</dialog>

<div class="flex flex-row w-100 h-100">
  <div class="h-100 right-border flex flex-column sidebar" style="width: 190px;">
    <div class="sidebar-head label">Workspace</div>
    <button
      class="branch-name {view == 'chat' ? 'active' : ''}"
      on:click={(_) => changeView("chat")}
      >Chat</button
    >
    <button
      class="branch-name {view == 'git' ? 'active' : ''}"
      on:click={(_) => changeView("git")}
      >Git</button
    >
    <div style="margin-top: auto;">
      <button class="branch-name" on:click={(_) => openLogDialog()}>Logs</button>
    </div>
  </div>
  <div
    class="h-100"
    style="display: {view == 'git'
      ? 'block'
      : 'none'}; width: calc(100% - 190px);"
  >
    <GitView {currentProject} />
  </div>
  <div
    class="h-100"
    style="display: {view == 'chat'
      ? 'block'
      : 'none'}; width: -webkit-fill-available;"
  >
    <div
      class="chat-header bottom-border flex flex-justify-between flex-align-center"
    >
      <button class="btn chat-header-btn" on:click={(_) => openSettingDialog()}>
        <span class="label">Model</span>
        <div class="chat-model">
          {#if selectedModel == ""}
            LLM Settings
          {:else}
            {selectedModel}
          {/if}
        </div>

        <span
          style="background-color: {serverLive
            ? 'var(--primary-color)'
            : '#d84a4a'};"
          class="server-status-indicator"
        ></span>
      </button>
      <button
        class="btn chat-header-btn {isOpenSavedMessages ? 'active' : ''}"
        on:click={(_) => openSavedMessages()}
      >
        Saved Messages
      </button>
    </div>
    <div class="flex" style="height: calc(100% - 46px); ">
      <div
        style="{isOpenSavedMessages
          ? 'width: 60%'
          : 'width: 100%'}; display:flex; flex-direction: column;"
      >
        <div class="main-scrollbar chat-response">
          {#if chat.length > 0}
            {#each chat as historyItem}
              <ChatItem {historyItem} />
            {/each}
          {/if}
        </div>
        <form class="composer w-100 flex top-border">
          <textarea
            class="w-100 input composer-input"
            placeholder="Type a message..."
            bind:value={userInput}
          > </textarea>
          <button class="btn btn-primary composer-btn" on:click={(_) => sendMessage()}
            >Send</button
          >
        </form>
      </div>
      <div
        class="left-border"
        style="height: 100%; display: {isOpenSavedMessages
          ? 'flex'
          : 'none'}; width: 40%"
      >
        <SavedMessages bind:userInput />
      </div>
    </div>
  </div>
</div>

<style>
  .sidebar {
    background: var(--surface-color);
  }

  .sidebar-head {
    padding: 14px 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .branch-name {
    padding: 11px 16px;
    margin: 0px;
    border-radius: 0px;
    cursor: pointer;
    border: none;
    border-left: 2px solid transparent;
    width: -webkit-fill-available;
    background: none;
    color: var(--muted-color);
    text-align: start;
    text-overflow: ellipsis;
    overflow: hidden;
    display: flex;
    align-items: center;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-size: 11px;
    font-family: inherit;
    transition: background 0.15s ease, color 0.15s ease,
      border-color 0.15s ease;
  }

  .branch-name:hover {
    background: var(--hover-color);
    color: var(--text-color);
  }

  .branch-name.active {
    color: var(--text-color);
    border-left-color: var(--primary-color);
    background: var(--hover-color);
  }

  .chat-response {
    flex: 1;
    min-height: 0;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .chat-header {
    padding: 8px 14px;
    gap: 10px;
    background: var(--surface-color);
  }

  .chat-header-btn {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .chat-header-btn.active {
    border-color: var(--primary-color);
    color: var(--primary-color);
  }

  .chat-model {
    color: var(--text-color);
    text-transform: none;
    letter-spacing: 0.02em;
    font-size: 12px;
  }

  .server-status-indicator {
    border-radius: 100%;
    height: 8px;
    width: 8px;
    display: inline-block;
  }

  .composer {
    flex: 0 0 auto;
    background: var(--surface-color);
  }

  .composer-input {
    padding: 14px 16px;
    min-height: 84px;
    font-size: 13px;
  }

  .composer-btn {
    flex-shrink: 0;
    border-left: none;
  }

  .dialog-pad {
    padding: 22px;
    height: 100%;
    overflow: auto;
    color: var(--text-color);
  }

  .dialog-title {
    display: block;
    margin-bottom: 16px;
    padding-bottom: 14px;
    border-bottom: 1px solid var(--border-color);
  }

  .log-item {
    padding: 10px 12px;
    margin-bottom: 8px;
    font-size: 12px;
    background: var(--background-color);
  }
</style>
