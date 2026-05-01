<script lang="ts">
    export let currentProject: string;
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import LlmSetting from "./llmSetting.svelte";
    import ChatItem from "./chatItem.svelte";
    import Add from "$lib/icons/add.svelte";
    import Close from "$lib/icons/close.svelte";
    import SavedMessages from "./savedMessages.svelte";
    import { listen } from "@tauri-apps/api/event";

    let stagedFiles: any[] = [];

    let unStagedFiles: any[] = [];

    let fileDiff = "";

    let selectedModel = "";

    let chat: any[] = [];

    let userInput = "";

    let llmSettingDialog: HTMLDialogElement;
    let logDialog: HTMLDialogElement;

    let view = "chat";

    let isOpenSavedMessages = false;

    let serverLive = false;
    let error: any[] = [];

    $: {
        if (currentProject != "add" && currentProject != "") {
            invoke("get_staged_files", {
                location: currentProject,
            }).then(function (data: any) {
                stagedFiles = data;
            });

            invoke("get_unstaged_files", {
                location: currentProject,
            }).then(function (data: any) {
                unStagedFiles = data;
            });
        }
    }

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
            console.log("tool", message);
            message = {
                role: "tool",
                content: message,
                tool_calls: [],
                thinking: null,
            };
            chat = [...chat, message];
        });

        listen("app-error", function (data: any) {
            let message: string = data.payload;
            error = [...error, message];
        });
    });

    function getChanges(file: string) {
        invoke("get_file_diff", {
            location: currentProject,
            file: file,
            isUnified: false,
        }).then(function (data: any) {
            fileDiff = data;
        });
    }

    function getUnstagedChanges(file: string) {
        invoke("get_unstaged_file_diff", {
            location: currentProject,
            file: file,
        }).then(function (data: any) {
            fileDiff = data;
        });
    }

    async function sendMessage() {
        if (userInput != "") {
            let message: any;
            console.log("Sending message...");
            if (chat.length == 0) {
                message = {
                    role: "system",
                    content: `You are a coding assistant with access to a local codebase.
## Project git repository location
${currentProject}

## Available tools
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

            let chatToSend = chat.filter(function(data){
              console.log(data);
              return data['role'] != 'tool';
            });

            console.log(chat);

            invoke("send_message", {
                message: [message],
                history: chatToSend,
            }).then(function (data: any) {
                chat = [...chat, data];
                console.log(chat);
            });
        }
    }

    function printFileChangeLine(line: string) {
        if (line != "") {
            let obj = JSON.parse(line);
            if (obj["from_no"] == obj["to_no"]) {
                return (
                    obj["change_type"] +
                    ": " +
                    obj["from_no"] +
                    ": " +
                    obj["content"]
                );
            } else if (obj["from_no"] == "") {
                return (
                    obj["change_type"] +
                    ": " +
                    obj["to_no"] +
                    ": " +
                    obj["content"]
                );
            } else if (obj["to_no"] == "") {
                return (
                    obj["change_type"] +
                    ": " +
                    obj["from_no"] +
                    ": " +
                    obj["content"]
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

    function addFile(e: any, path: string) {
        e.stopPropagation();
        invoke("add_file_index", {
            location: currentProject,
            path: path,
        }).then(function () {
            getChangeFiles();
        });
    }

    function removeFileFromIndex(e: any, path: string) {
        e.stopPropagation();
        invoke("remove_file_index", {
            location: currentProject,
            path: path,
        }).then(function () {
            getChangeFiles();
        });
    }

    function getChangeFiles() {
        invoke("get_staged_files", {
            location: currentProject,
        }).then(function (data: any) {
            stagedFiles = data;
        });

        invoke("get_unstaged_files", {
            location: currentProject,
        }).then(function (data: any) {
            unStagedFiles = data;
        });
    }

    function openSavedMessages() {
        isOpenSavedMessages = !isOpenSavedMessages;
    }
</script>

<dialog bind:this={llmSettingDialog} on:close>
    <LlmSetting bind:selectedModel />
</dialog>

<dialog bind:this={logDialog} on:close>
    <div style="color: var(--text-color)">
        {#each error as errorMessage}
            <div style="padding: 5px;" class="full-border">
                {errorMessage}
            </div>
        {/each}
    </div>
</dialog>

<div class="flex flex-row w-100 h-100">
    <div class="h-100 right-border flex flex-column" style="width: 200px;">
        <button
            class="branch-name hover btn"
            on:click={(_) => changeView("chat")}>chat</button
        >
        <button
            class="branch-name hover btn"
            on:click={(_) => changeView("git")}>git</button
        >
        <div style="margin-top: auto;">
            <button
                class="branch-name hover btn"
                on:click={(_) => openLogDialog()}>Log</button
            >
        </div>
    </div>
    <div
        class="h-100"
        style="display: {view == 'git'
            ? 'block'
            : 'none'}; width: -webkit-fill-available;"
    >
        <div class="h-100 flex">
            <div class="w-50 h-100">
                <div class="h-50 main-scrollbar right-border bottom-border">
                    Staged Files
                    {#each stagedFiles as file}
                        <button
                            on:click={(_) => getChanges(file)}
                            class="commit-file-path hover"
                        >
                            <div>
                                {file}
                            </div>
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <div
                                on:click={function (e) {
                                    removeFileFromIndex(e, file);
                                }}
                            >
                                <Close />
                            </div>
                        </button>
                    {/each}
                </div>
                <div class="h-50 main-scrollbar right-border">
                    Unstaged Files
                    {#each unStagedFiles as file}
                        <button
                            on:click={(_) => getUnstagedChanges(file)}
                            class="commit-file-path hover"
                        >
                            <div>
                                {file}
                            </div>
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <div
                                on:click={function (e) {
                                    addFile(e, file);
                                }}
                            >
                                <Add />
                            </div>
                        </button>
                    {/each}
                </div>
            </div>
            <div class="w-50 h-100 right-border bottom-border main-scrollbar">
                <div>
                    {#each fileDiff.split("\n") as line}
                        <pre
                            style="background-color: {getChangeLineColor(
                                line,
                            )};"
                            class="hover m-0 text-wrap-wrap">{printFileChangeLine(
                                line,
                            )}</pre>
                    {/each}
                </div>
            </div>
        </div>
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
            <button
                class="btn flex flex-row"
                on:click={(_) => openSettingDialog()}
            >
                <div>
                    {#if selectedModel == ""}
                        LLM Settings
                    {:else}
                        {selectedModel}
                    {/if}
                </div>

                <span
                    style="background-color: {serverLive
                        ? 'var(--primary-color)'
                        : 'red'};"
                    class="server-status-indicator"
                ></span>
            </button>
            <button class="btn" on:click={(_) => openSavedMessages()}>
                Saved Messages
            </button>
        </div>
        <div class="flex" style="height: calc(100% - 26px); ">
            <div style="{isOpenSavedMessages ? "width: 60%" : "width: 100%"}; display:flex; flex-direction: column;">
                <div class="main-scrollbar chat-response">
                    {#if chat.length > 0}
                        {#each chat as historyItem}
                            <ChatItem {historyItem} />
                        {/each}
                    {/if}
                </div>
                <form class="w-100 flex top-border" style="flex-grow: 1;">
                    <textarea class="w-100 input" bind:value={userInput}> </textarea>
                    <button class="btn" on:click={(_) => sendMessage()}
                        >Send</button
                    >
                </form>
            </div>
            <div
                class="left-border"
                style="height: calc(100% - 24px); display: {isOpenSavedMessages
                    ? 'flex'
                    : 'none'}; width: 40%"
            >
                <SavedMessages bind:userInput />
            </div>
        </div>
    </div>
</div>

<style>
    .branch-name {
        padding: 10px 5px;
        margin: 0px 5px;
        border-radius: 5px;
        cursor: pointer;
        border: none;
        width: -webkit-fill-available;
        background: none;
        color: var(--text-color);
        text-align: start;
        text-overflow: ellipsis;
        overflow: hidden;
        display: flex;
        align-items: center;
    }

    .commit-file-path {
        padding: 6px;
        cursor: pointer;
        border-radius: 4px;
        background: transparent;
        color: var(--text-color);
        border: none;
        width: -webkit-fill-available;
        text-align: start;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .chat-response {
        height: 90%;
    }

    .server-status-indicator {
        border-radius: 100%;
        height: 15px;
        width: 15px;
        display: inline-block;
        margin-right: 5px;
        margin-left: 5px;
    }
</style>
