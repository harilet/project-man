<script lang="ts">
    import { onMount } from "svelte";
    import ModelDropDown from "./modelDropDown.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import dataStore from "$lib/data_store";
    export let selectedModel = "";

    let ollama_settings: any;
    let ollama_server = "";
    let color = "#00d492";

    onMount(() => {
        invoke("get_ollama_setting").then(function (data: any) {
            ollama_settings = JSON.parse(data);
            ollama_server = ollama_settings["ollama_server"];
            if ("ollama_server" in ollama_settings) {
                ollama_server = ollama_settings["ollama_server"];
            }
            if ("model" in ollama_settings) {
                selectedModel = ollama_settings["model"];
            }
        });
    });

    function saveOllamaServerSettings() {
        if (ollama_server != "") {
            invoke("set_ollama_setting", {
                name: "ollama_server",
                value: ollama_server,
            });
        }

        if (selectedModel != "") {
            invoke("set_ollama_setting", {
                name: "model",
                value: selectedModel,
            });
        }
    }

    async function colorChange(){
      const root = document.documentElement;

      root.style.setProperty(`--primary-color`, color);
      await dataStore.set("primary-color", color);
    }
</script>

<div class="h-100 flex flex-column">
    <div class="dialog-head">
        <span class="label">01 / Settings</span>
        <span class="dialog-title">LLM Settings</span>
    </div>
    <div class="main-scrollbar settings-scroll">
        <div class="setting-item-session">
            <div class="label field-label">Model</div>
            <div>
                <ModelDropDown bind:selectedModel />
            </div>
        </div>

        <div class="setting-item-session">
            <div class="label field-label">Server Url</div>
            <form class="w-100 flex">
                <input
                    class="w-100 input setting-input"
                    bind:value={ollama_server}
                />
            </form>
        </div>

        <div class="setting-item-session">
            <div class="label field-label">Accent Color</div>
            <input
                class="color-input full-border"
                type="color"
                id="favcolor"
                name="favcolor"
                bind:value={color}
                on:change={() => colorChange()}
            />
        </div>

        <button
            class="btn btn-primary btn-block"
            on:click={(_) => saveOllamaServerSettings()}>Save Settings</button
        >
    </div>
</div>

<style>
    .dialog-head {
        display: flex;
        flex-direction: column;
        gap: 8px;
        padding: 22px;
        border-bottom: 1px solid var(--border-color);
    }

    .dialog-title {
        font-size: 20px;
        font-weight: 700;
        letter-spacing: -0.01em;
        color: var(--text-color);
    }

    .settings-scroll {
        padding: 22px;
        flex: 1;
        min-height: 0;
    }

    .setting-item-session {
        margin-bottom: 26px;
    }

    .field-label {
        margin-bottom: 8px;
    }

    .setting-input {
        padding: 11px 14px;
        border: 1px solid var(--border-strong);
        background: var(--background-color);
        transition: border-color 0.15s ease;
    }

    .setting-input:focus {
        border-color: var(--primary-color);
    }

    .color-input {
        width: 64px;
        height: 38px;
        padding: 0;
        background: var(--background-color);
        cursor: pointer;
    }
</style>
