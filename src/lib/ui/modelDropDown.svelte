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

<div class="flex flex-justify-between flex-align-center w-100">
  <div class="w-100 dropdown-wrap" on:focusout={handleDropdownFocusLoss}>
    <button class="btn dropdown-trigger w-100" on:click={handleDropdownClick}>
      <span class="dropdown-value">{selectedModel || "Select a model"}</span>
      <span class="dropdown-caret">&#9662;</span>
    </button>
    <ul style:visibility={isDropdownOpen ? "visible" : "hidden"}>
      {#each allModels as model}
        <li class="flex flex-justify-center">
          <button
            on:click={(_) => onOptionCLick(model["name"])}
            class="btn w-100 option-item"
          >
            <div class="flex option-tags">
              <div class="tag">
                {model["name"]}
              </div>
              <div class="tag">
                {model["architecture"]}
              </div>
              <div class="tag">
                {model["context"]}
              </div>
            </div>
            <div class="flex option-tags">
              {#each model["capabilities"] as capability}
                <div class="tag">
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
  .dropdown-wrap {
    position: relative;
  }

  .dropdown-trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    height: 42px;
    border: 1px solid var(--border-strong);
    background: var(--background-color);
    text-transform: none;
    letter-spacing: 0.02em;
    font-size: 13px;
  }

  .dropdown-value {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dropdown-caret {
    color: var(--muted-color);
    flex-shrink: 0;
  }

  ul {
    list-style-type: none;
    padding: 4px;
    margin: 0px;
    border: 1px solid var(--border-strong);
    position: absolute;
    background: var(--surface-color);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
    overflow: auto;
    width: 100%;
    max-height: 260px;
    z-index: 5;
  }

  .option-item {
    margin: 0px 0px 4px 0px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    padding: 10px;
    border: 1px solid transparent;
    text-transform: none;
    letter-spacing: 0;
  }

  .option-item:hover {
    border-color: var(--border-color);
  }

  .option-tags {
    flex-wrap: wrap;
  }

  .tag {
    margin: 0px 4px 4px 0px;
    padding: 2px 6px;
    border: 1px solid var(--border-color);
    color: var(--muted-color);
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }
</style>
