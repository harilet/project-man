<script lang="ts">
    import { fly } from "svelte/transition";

    export let message: string[] = [];

    let visible = false;

    $: if (message.length > 0) {
        visible = true;

        setTimeout(() => {
            message.pop();
            message = message;
            if (message.length < 1) {
                visible = false;
            }
        }, 5000);
    }
</script>

{#if visible}
    <div class="toast">
        <div class="flex flex-col">
            {#each message as item}
                <div
                    class="toast-item"
                    in:fly={{ x: 200 }}
                    out:fly={{ y: 200 }}
                >
                    {item}
                </div>
            {/each}
        </div>
    </div>
{/if}

<style>
    .toast {
        position: fixed;
        bottom: 14px;
        right: 18px;
        z-index: 10;
    }

    .toast-item {
        padding: 12px 18px;
        background-color: var(--surface-color);
        color: var(--text-color);
        border: 1px solid var(--border-strong);
        border-left: 2px solid var(--primary-color);
        border-radius: 0px;
        margin-top: 6px;
        font-size: 12px;
        max-width: 360px;
        box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
    }
</style>
