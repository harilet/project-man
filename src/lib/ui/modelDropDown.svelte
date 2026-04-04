<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let selectedModel: string;

  let isDropdownOpen = false;

  let allModels: any[] = [];

  const handleDropdownClick = () => {
    isDropdownOpen = !isDropdownOpen;
    if (isDropdownOpen) {
      invoke("get_all_local_models").then(function (data: any) {
        let data2 = data as string[];
        data2 = data2.map((value, index) => {
          return JSON.parse(value);
        });

        allModels = data2;
      });
    }
  };

  function handleDropdownFocusLoss(relatedTarget: any) {
    if (
      relatedTarget.relatedTarget instanceof HTMLElement &&
      relatedTarget.currentTarget.contains(relatedTarget.relatedTarget)
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
            on:click={(_) => onOptionCLick(model["name"])}
            class="btn w-100 option-item"
          >
            <div class="flex">
              <div style="margin: 2px;padding: 0px 4px;" class="full-border">
                {model["name"]}
              </div>
              <div style="margin: 2px;padding: 0px 4px;" class="full-border">
                {model["architecture"]}
              </div>
              <div style="margin: 2px;padding: 0px 4px;" class="full-border">
                {model["context"]}
              </div>
            </div>
            <div class="flex">
              {#each model["capabilities"] as capability}
                <div style="margin: 2px;padding: 0px 4px;" class="full-border">
                  {capability}
                </div>
              {/each}
            </div>
          </button>
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
    position: absolute;
    background: black;
    overflow: auto;
    height: 70%;
  }

  .option-item {
    margin: 5px;
    display: flex;
    flex-direction: column;
  }
</style>
