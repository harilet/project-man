<script lang="ts">
  import { onMount } from "svelte";
  import ModelDropDown from "./modelDropDown.svelte";
  import { invoke } from "@tauri-apps/api/core";
  export let selectedModel = "";
  export let user_prompt = "";
  export let system_prompt = "";

  let ollama_settings: any;
  let ollama_server = "";

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

      if ("system_prompt" in ollama_settings) {
        system_prompt = ollama_settings["system_prompt"];
      }

      if ("user_prompt" in ollama_settings) {
        user_prompt = ollama_settings["user_prompt"];
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

    if (system_prompt != "") {
      invoke("set_ollama_setting", {
        name: "system_prompt",
        value: system_prompt,
      });
    }

    if (user_prompt != "") {
      invoke("set_ollama_setting", {
        name: "user_prompt",
        value: user_prompt,
      });
    }
  }

  function setTheme() {
  const root = document.documentElement;

  let theme = {
        "text-color": "#ffffff",
        "hover-color": "#0b0809",
        "primary-color": "#c27aff",
        "background-color": "#0a0a0a",
        "border-color": "color-mix(in oklab, var(--primary-color) 30%, transparent)",
        "fill-color": "#171717",
};

  Object.entries(theme).forEach(([key, value]) => {
    root.style.setProperty(`--${key}`, value);
  });
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
    <div class="setting-item-session">
      <div class="text-color">System Prompt</div>
      <textarea
        class="w-100 input full-border prompt-textarea"
        bind:value={system_prompt}
      ></textarea>
    </div>

    <div class="setting-item-session">
      <div class="text-color">User Prompt</div>
      <textarea
        class="w-100 input full-border prompt-textarea"
        bind:value={user_prompt}
      ></textarea>
    </div>

    <button class="btn" on:click={(_) => setTheme()}
      >Chnage theme</button
    >

    <button class="btn" on:click={(_) => saveOllamaServerSettings()}
      >Save</button
    >
  </div>
</div>

<style>
  .setting-item-session {
    margin-bottom: 25px;
  }
  .prompt-textarea {
    padding: 5px;
    max-width: calc(100% - 12px);
    field-sizing: content;
  }
</style>
