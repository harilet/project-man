<script lang="ts">
  import FolderOpen from "$lib/icons/folder_open.svelte";
  import ErrorToast from "$lib/ui/errorToast.svelte";
  import MainView from "$lib/ui/mainView.svelte";
  import ModelDropDown from "$lib/ui/modelDropDown.svelte";
  import Tabs from "$lib/ui/tabs.svelte";
  import Titlebar from "$lib/ui/titlebar.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  let recentProjects = ["project 1", "project 2"];

  let openProjects: any[] = [
    {
      key: "C:\\Users\\Asus\\Documents\\tauri\\project-man",
      name: "project-man",
    },
  ];

  let currentProject = "";

  let projectLocation = "";

  let error: any[] = [];

  onMount(() => {
    listen("app-error", function (data: any) {
      let message: string = data.payload;
      error = [...error, message];
    });

    if (openProjects.length > 0) {
      currentProject = openProjects[0].key;
    } else {
      currentProject = "add";
    }
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

<ErrorToast bind:message={error} />

<main class="container flex flex-column">
  <Tabs tabItems={openProjects} bind:currentTab={currentProject} />
  <div style="height:calc(100% - 42px);">
    <div
      class="flex flex-row w-100 h-100"
      class:hidden={currentProject !== "add"}
    >
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
    <div class="w-100 h-100" class:hidden={currentProject === "add"}>
      {#each openProjects as project}
        <div class="w-100 h-100" class:hidden={currentProject !== project.key}>
          <MainView currentProject={project.key} />
        </div>
      {/each}
    </div>
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
