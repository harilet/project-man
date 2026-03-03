<script lang="ts">
    export let currentProject: string;
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import LlmSetting from "./llmSetting.svelte";
    import ChatItem from "./chatItem.svelte";
    import Add from "$lib/icons/add.svelte";
    import Close from "$lib/icons/close.svelte";

    let stagedFiles: any[] = [];

    let unStagedFiles: any[] = [];

    let fileDiff = "";

    let selectedModel = "";
    let user_prompt = "";
    let system_prompt = "";

    let chat: any[] = [];

    let userInput = "";

    let llmSettingDialog: HTMLDialogElement;

    let view = "chat";

    let progress = "";

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
            console.log("Sending message...", chat.length);
            if (chat.length == 0) {
                let staged_diff = "";
                let stagedFiles: any = await invoke("get_staged_files", {
                    location: currentProject,
                });
                console.log(stagedFiles);
                for (let file in stagedFiles) {
                    console.log("file", stagedFiles[file]);
                    let file_diff = await invoke("get_file_diff", {
                        location: currentProject,
                        file: stagedFiles[file],
                        isUnified: true,
                    });
                    console.log(file_diff);
                    staged_diff += file_diff;
                }
                console.log(staged_diff);

                message = {
                    role: "user",
                    content: staged_diff,
                    tool_calls: [],
                    thinking: null,
                };
                console.log(message);
                chat = [...chat, message];
                console.log(chat);
            }

            message = {
                role: "user",
                content: userInput,
                tool_calls: [],
                thinking: null,
            };
            chat = [...chat, message];
            userInput = "";

            invoke("send_message", {
                message: [message],
                history: chat,
            }).then(function (data: any) {
                console.log(data);
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

    function openDialog() {
        if (llmSettingDialog.open) {
            llmSettingDialog.close();
        } else {
            llmSettingDialog.showModal();
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
</script>

<dialog bind:this={llmSettingDialog} on:close>
    <LlmSetting bind:selectedModel bind:system_prompt bind:user_prompt />
</dialog>

<div class="flex flex-row w-100 h-100">
    <div class="w-25 h-100 right-border">
        <button
            class="branch-name hover btn"
            on:click={(_) => changeView("chat")}>chat</button
        >
        <button
            class="branch-name hover btn"
            on:click={(_) => changeView("git")}>git</button
        >
    </div>
    <div
        class="w-75 h-100"
        style="display: {view == 'git' ? 'block' : 'none'};"
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
        class="w-75 h-100"
        style="display: {view == 'chat' ? 'block' : 'none'};"
    >
        <progress value={progress.split("/")[0]} max={progress.split("/")[1]}
            >%</progress
        >
        <div class="chat-header">
            <button class="btn" on:click={(_) => openDialog()}>
                {#if selectedModel == ""}
                    LLM Settings
                {:else}
                    {selectedModel}
                {/if}
            </button>
        </div>
        <div class="h-100">
            <div class="main-scrollbar chat-response">
                {#if chat.length > 0}
                    {#each chat as historyItem}
                        <ChatItem {historyItem} />
                    {/each}
                {/if}
            </div>
            <div class="chat-footer">
                <div class="flex endpoint-input top-border h-100">
                    <form class="w-100 flex">
                        <input class="w-100 input" bind:value={userInput} />
                        <button class="btn" on:click={(_) => sendMessage()}
                            >Send</button
                        >
                    </form>
                </div>
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

    .chat-header {
        height: 24px;
    }

    .chat-response {
        height: calc(100% - 74px);
    }

    .chat-footer {
        height: 37px;
    }
</style>
