<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let selectedModel: string;

  let isDropdownOpen = false;

  let allModels:any[] = [];

  const handleDropdownClick = () => {
    isDropdownOpen = !isDropdownOpen;
    if (isDropdownOpen) {
      invoke("get_all_local_models").then(function (data: any) {
        allModels = data;
      });
    }
  };

  function handleDropdownFocusLoss(relatedTarget: any) {
    if (
      relatedTarget.relatedTarget instanceof HTMLElement &&
      relatedTarget.currentTarget.contains(relatedTarget)
    )
      return;
    isDropdownOpen = false;
  }

  function onOptionCLick(value: string) {
    selectedModel = value;
    isDropdownOpen = false;
  }
</script>

<div class="flex flex-justify-between flex-align-center w-25 h-100">
  <div class="w-100 h-100" on:focusout={handleDropdownFocusLoss}>
    <button class="btn w-100 h-100" on:click={handleDropdownClick}>
      {selectedModel}
    </button>
    <ul style:visibility={isDropdownOpen ? "visible" : "hidden"}>
      {#each allModels as model}
        <li class="flex flex-justify-center">
          <button
            on:click={(_) => onOptionCLick(model)}
            class="btn w-100 option-item">{model}</button
          >
        </li>
      {/each}
    </ul>
  </div>
</div>

<style>
  ul {
    list-style-type: none;
    padding: 0px;
    margin: 0px;
    border: 1px solid var(--border-color);
  }

  .option-item {
    margin: 5px;
  }
</style>
