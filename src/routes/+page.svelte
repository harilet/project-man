<script lang="ts">
  import FolderOpen from "$lib/icons/folder_open.svelte";
  import Tabs from "$lib/ui/tabs.svelte";
  import Titlebar from "$lib/ui/titlebar.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  let recentProjects = ["project 1", "project 2"];

  let openProjects: any[] = [
    { key: "C:UsersAsusDocuments\tauriproject-man", name: "project-man" },
  ];

  let currentProject = "";

  let projectLocation = "";
  
  $: error = [""];
  
  onMount(() => {
    if (openProjects.length > 0) {
      currentProject = openProjects[0].key;
    } else {
      currentProject = "add";
    }

    listen("error", function (data: any) {
      let message: string = data.payload;
      error = [...error, message];
    });
  });

  async function openFileSelector() {
    const file = await open({
      multiple: false,
      directory: true,
    });
    if (file != null) {
      projectLocation = file;
    }
  }

  function openProject() {
    console.log(projectLocation);
    openProjects = [
      ...openProjects,
      {
        name: projectLocation.split("\\")[
          projectLocation.split("\\").length - 1
        ],
        key: projectLocation,
      },
    ];
    currentProject = projectLocation;
    projectLocation = "";
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>

<main class="container flex flex-column">
  <Tabs tabItems={openProjects} bind:currentTab={currentProject} />
  <div style="height:calc(100% - 42px);">
    {#if currentProject == "add"}
      <div class="flex flex-row w-100 h-100">
        <div class="w-50 h-100 flex flex-justify-center flex-align-center">
          <div class="flex flex-column">
            <div>recent</div>
            <div>
              {#each recentProjects as recentProject}
                <div>
                  {recentProject}
                </div>
              {/each}
            </div>
          </div>
        </div>
        <div
          class="w-50 h-100 left-border flex flex-justify-center flex-align-center"
        >
          <div class="w-100 flex flex-column new-session">
            <div>New</div>
            <div>
              <div class="flex endpoint-input full-border">
                <input class="w-100 input" bind:value={projectLocation} />
                <button class="w-10 btn" on:click={(_) => openFileSelector()}>
                  <FolderOpen />
                </button>
              </div>
              <button class="w-100 btn" on:click={(_) => openProject()}
                >Open</button
              >
            </div>
          </div>
        </div>
      </div>
    {:else}
      <div class="flex flex-row w-100 h-100">
        <div class="w-50 h-100">
          <div class="h-50 bottom-border">tl</div>
          <div class="h-50">bl</div>
        </div>
        <div class="w-50 h-100 left-border">r</div>
      </div>
    {/if}
  </div>
</main>

<style>
  .container {
    height: calc(100% - 35px);
  }
  .app-bar {
    width: 100%;
    height: 35px;
  }

  .endpoint-input {
    height: 40px;
    border: 1px solid var(--border-color);
  }

  .new-session {
    margin: 4px;
  }
</style>
