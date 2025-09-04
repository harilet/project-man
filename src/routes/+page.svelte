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

  let recentProjects: any[] = [];

  let openProjects: any[] = [];

  let currentProject = "";

  let projectLocation = "";

  let error: any[] = [];

  let serverLive = false;

  $: {
    if (openProjects.length > 0) {
      currentProject = openProjects[openProjects.length - 1].key;
    } else {
      currentProject = "add";
    }
  }

  onMount(() => {
    listen("app-error", function (data: any) {
      let message: string = data.payload;
      error = [...error, message];
    });

    listen("ollama-server-status", function (data: any) {
      let message: string = data.payload;
      if (message == "live") {
        serverLive = true;
      } else {
        serverLive = false;
      }
    });

    if (openProjects.length > 0) {
      currentProject = openProjects[openProjects.length - 1].key;
    } else {
      currentProject = "add";
    }

    invoke("get_recent_projects").then((data) => {
      recentProjects = (data as string[]).reverse();
    });

    invoke("start_ollama_server_check");
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
    let projectName =
      projectLocation.split("\\")[projectLocation.split("\\").length - 1];
    invoke("set_projects", { name: projectName, path: projectLocation });

    openProjects = [
      ...openProjects,
      {
        name: projectName,
        key: projectLocation,
      },
    ];
    currentProject = projectLocation;
    projectLocation = "";
  }

  function openRecent(project: string) {
    projectLocation = project;
    openProject();
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>

<ErrorToast bind:message={error} />

<div class="container flex flex-column">
  <Tabs bind:tabItems={openProjects} bind:currentTab={currentProject} />
  <div style="height:calc(100% - 44px);">
    <div
      class="flex flex-row w-100 h-100"
      class:hidden={currentProject !== "add"}
    >
      <div class="w-50 h-100 flex flex-justify-center flex-align-center">
        <div class="flex flex-column recent-column">
          <div>recent</div>
          <div class="recent-projects">
            {#each recentProjects as recentProject}
              <button
                class="btn w-100 recent-project"
                on:click={(_) => openRecent(recentProject)}
              >
                {recentProject}
              </button>
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
            <div class="flex endpoint-input top-border left-border">
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
</div>
<div class="w-100 footer-class top-border flex flex-align-center">
  <div class="w-100">hu</div>
  <span
    style="background-color: {serverLive ? 'var(--primary-color)' : 'red'};"
    class="server-status-indicator"
  ></span>
</div>

<style>
  .container {
    height: calc(100% - 70px);
  }
  .app-bar {
    width: 100%;
    height: 35px;
  }

  .endpoint-input {
    height: 40px;
  }

  .new-session {
    margin: 4px;
  }

  .footer-class {
    height: 34px;
  }

  .recent-project {
    margin: 5px 0px;
    border-radius: 2px;
  }

  .recent-column {
    padding: 5px;
  }

  .server-status-indicator {
    border-radius: 100%;
    height: 15px;
    width: 15px;
    display: inline-block;
    margin-right: 5px;
  }
</style>
