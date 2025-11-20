<script lang="ts">
  export let currentProject: string;

  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import FileSummrie from "./fileSummrie.svelte";

  let fileSummries: any[] = [];
  let finalSummries = "";
  onMount(() => {
    invoke("generate_commit_message", {
      location: currentProject,
    }).then(function (data: any) {
      console.log(data);
      finalSummries = data;
    });

    listen("file-summrie", (event) => {
      fileSummries = [...fileSummries, event.payload];
    });
  });
</script>

<div class="flex file-summries">
  <div class="main-scrollbar w-50">
    {#each fileSummries as fileSummrie}
      <div>
        <FileSummrie data={fileSummrie} />
      </div>
    {/each}
  </div>
  <div class="w-50">
    {finalSummries}
  </div>
</div>

<style>
  .file-summries {
    height: calc(100% - 40px);
  }
</style>
