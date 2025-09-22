<script lang="ts">
  import { onMount } from "svelte";
  import ModelDropDown from "./modelDropDown.svelte";
  import { invoke } from "@tauri-apps/api/core";
  export let selectedModel = "";

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
    });
  });

  $:{
    console.log("model"+selectedModel);
    invoke("set_ollama_setting", {
      name: "model",
      value: selectedModel,
    });
  }

  function setOllamaServerUrl() {
    invoke("set_ollama_setting", {
      name: "ollama_server",
      value: ollama_server,
    });
  }
</script>

<div class="h-100 flex flex-column">
  <div class="text-color" style="margin-bottom: 25px;">LLM Settings</div>
  <div style="margin-bottom: 25px;">
    <div class="text-color">Model</div>
    <div>
      <ModelDropDown bind:selectedModel />
    </div>
  </div>

  <div style="margin-bottom: 25px;">
    <div class="text-color">Server Url</div>
    <form class="w-100 flex">
      <input
        class="w-100 input full-border"
        style="padding: 5px;"
        bind:value={ollama_server}
      />
      <button class="btn" on:click={(_) => setOllamaServerUrl()}>Send</button>
    </form>
  </div>
</div>
