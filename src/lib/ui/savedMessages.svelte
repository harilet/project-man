<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import Close from "$lib/icons/close.svelte";

    let savesMessages: string[] = [];
    let newMessage: string = "";
    let showAddMessage: boolean = false;

    export let userInput;

    onMount(() => {
        invoke("get_saved_messages", {}).then(function (data: any) {
            console.log(data);
            savesMessages = data;
        });
    });

    function toggleAddMessage() {
        showAddMessage = !showAddMessage;
    }

    function toggleMessageExpansion(message: string) {
        userInput = message;
    }

    function addMessage() {
        if (newMessage.trim() !== "") {
            invoke("add_saved_messages", { message: newMessage }).then(
                function (data: any) {
                    savesMessages = data;
                    newMessage = "";
                    showAddMessage = false;
                },
            );
        }
    }

    function deleteMessage(message: string) {
        invoke("delete_saved_message", { message: message }).then(function (
            data: any,
        ) {
            savesMessages = data;
        });
    }

    function truncateText(text: string, maxLength: number = 50): string {
        if (text.length <= maxLength) return text;
        return text.substring(0, maxLength) + "...";
    }
</script>

<div class="saved-messages-container">
    <div class="saved-messages-header">
        <h3>Saved Messages</h3>
        <button class="btn add-message-btn" on:click={toggleAddMessage}>
            {#if showAddMessage}
                Cancel
            {:else}
                + Add
            {/if}
        </button>
    </div>

    {#if showAddMessage}
        <div class="add-message-form">
            <textarea
                class="message-input"
                bind:value={newMessage}
                placeholder="Enter your message..."
                rows="2"
            ></textarea>
            <div class="form-actions">
                <button class="btn cancel-btn" on:click={toggleAddMessage}
                    >Cancel</button
                >
                <button class="btn save-btn" on:click={addMessage}>Save</button>
            </div>
        </div>
    {/if}

    <div class="saved-messages-list">
        {#if savesMessages.length === 0}
            <div class="empty-state">No saved messages</div>
        {:else}
            {#each savesMessages as message (message)}
                <div class="saved-message-item">
                    <button
                        class="message-content"
                        on:click={() => toggleMessageExpansion(message)}
                        aria-label="Toggle message expansion"
                        type="button"
                    >
                        {message}
                    </button>
                    <button
                        class="btn delete-btn"
                        on:click={() => deleteMessage(message)}
                        title="Delete"
                    >
                        <Close />
                    </button>
                </div>
            {/each}
        {/if}
    </div>
</div>

<style>
    .saved-messages-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 8px;
        gap: 8px;
        overflow: hidden;
        width: 100%;
    }

    .saved-messages-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-bottom: 8px;
        border-bottom: 1px solid var(--border-color);
    }

    .saved-messages-header h3 {
        margin: 0;
        font-size: 14px;
        color: var(--text-color);
        font-weight: 600;
    }

    .add-message-btn {
        padding: 4px 8px;
        font-size: 11px;
        background-color: var(--primary-color);
        color: white;
        border: none;
        border-radius: 3px;
        cursor: pointer;
        white-space: nowrap;
    }

    .add-message-form {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .message-input {
        width: 100%;
        padding: 6px;
        border: 1px solid var(--border-color);
        border-radius: 3px;
        background-color: var(--background-color);
        color: var(--text-color);
        font-family: inherit;
        resize: vertical;
        min-height: 40px;
        font-size: 12px;
    }

    .message-input:focus {
        outline: none;
        border-color: var(--primary-color);
    }

    .form-actions {
        display: flex;
        justify-content: flex-end;
        gap: 6px;
    }

    .cancel-btn {
        padding: 4px 10px;
        background-color: transparent;
        color: var(--text-color);
        border: 1px solid var(--border-color);
        border-radius: 3px;
        cursor: pointer;
        font-size: 11px;
    }

    .save-btn {
        padding: 4px 10px;
        background-color: var(--primary-color);
        color: white;
        border: none;
        border-radius: 3px;
        cursor: pointer;
        font-size: 11px;
    }

    .save-btn:hover {
        opacity: 0.9;
    }

    .saved-messages-list {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .empty-state {
        text-align: center;
        color: var(--text-color);
        opacity: 0.6;
        padding: 15px;
        font-style: italic;
        font-size: 12px;
    }

    .saved-message-item {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 6px;
        padding: 8px;
        background-color: var(--card-background);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        min-height: 32px;
    }

    .message-content {
        flex: 1;
        word-wrap: break-word;
        white-space: pre-wrap;
        color: var(--text-color);
        font-size: 12px;
        line-height: 1.3;
        cursor: pointer;
        overflow: hidden;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
        line-clamp: 2;
        text-align: left;
        background: none;
        border: none;
        padding: 0;
        margin: 0;
    }

    .delete-btn {
        padding: 2px;
        background-color: transparent;
        border: none;
        cursor: pointer;
        color: var(--text-color);
        opacity: 0.6;
        flex-shrink: 0;
        min-width: 16px;
        min-height: 16px;
    }

    .delete-btn:hover {
        opacity: 1;
        background-color: #ff1d5722;
        border-radius: 2px;
    }

    .btn {
        transition: all 0.15s ease;
    }

    :global(.main-scrollbar) {
        scrollbar-width: thin;
        scrollbar-color: var(--primary-color) transparent;
    }

    /* Mobile-specific adjustments */
    @media (max-width: 480px) {
        .saved-messages-container {
            padding: 6px;
            gap: 6px;
        }

        .saved-messages-header h3 {
            font-size: 13px;
        }

        .add-message-btn {
            padding: 3px 6px;
            font-size: 10px;
        }

        .message-input {
            padding: 5px;
            font-size: 11px;
        }

        .saved-message-item {
            padding: 6px;
        }

        .message-content {
            font-size: 11px;
        }
    }
</style>
