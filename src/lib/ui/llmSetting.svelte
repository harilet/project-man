<script lang="ts">
    import { onMount } from "svelte";
    import ModelDropDown from "./modelDropDown.svelte";
    import { invoke } from "@tauri-apps/api/core";
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

    function colorChange(){
      const root = document.documentElement;

      root.style.setProperty(`--primary-color`, color);
    }
</script>

<div class="h-100 flex flex-column">
    <div class="text-color setting-item-session">LLM Settings</div>
    <div class="main-scrollbar">
        <div class="setting-item-session">
            <div class="text-color">Model</div>
            <div>
                <ModelDropDown bind:selectedModel />
            </div>
        </div>

        <div class="setting-item-session">
            <div class="text-color">Server Url</div>
            <form class="w-100 flex">
                <input
                    class="w-100 input full-border"
                    style="padding: 5px;"
                    bind:value={ollama_server}
                />
            </form>
        </div>

        <input type="color" id="favcolor" name="favcolor" bind:value={color} on:change={()=>colorChange()} />

        <button class="btn" on:click={(_) => saveOllamaServerSettings()}
            >Save</button
        >
    </div>
</div>

<style>
    .setting-item-session {
        margin-bottom: 25px;
    }
</style>
