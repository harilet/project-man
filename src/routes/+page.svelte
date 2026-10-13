<script lang="ts">
    import FolderOpen from "$lib/icons/folder_open.svelte";
    import ErrorToast from "$lib/ui/errorToast.svelte";
    import MainView from "$lib/ui/mainView.svelte";
    import Titlebar from "$lib/ui/titlebar.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { open } from "@tauri-apps/plugin-dialog";
    import { onMount } from "svelte";
    import dataStore from "$lib/data_store";

    let recentProjects: any[] = [];

    let openProjects: any[] = [];

    let currentProject = "";

    let projectLocation = "";

    let error: any[] = [];

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

        if (openProjects.length > 0) {
            currentProject = openProjects[openProjects.length - 1].key;
        } else {
            currentProject = "add";
        }

        invoke("get_recent_projects").then((data) => {
            recentProjects = (data as string[]).reverse();
        });

        invoke("start_ollama_server_check");

        dataStore.get("primary-color").then((data: any) => {
            if (data != null) {
                const root = document.documentElement;
                root.style.setProperty(`--primary-color`, data);
            }
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
    <Titlebar bind:tabItems={openProjects} bind:currentTab={currentProject} />
</div>

<ErrorToast bind:message={error} />

<div class="main-container flex flex-column">
    <div
        class="landing flex flex-column w-100 h-100"
        class:hidden={currentProject !== "add"}
    >
        <div class="landing-meta flex flex-justify-between flex-align-center">
            <span class="label">01 / Projects</span>
            <span class="label"
                >{recentProjects.length} Recent &middot; Local Repository &middot;
                Git</span
            >
        </div>

        <div class="landing-body flex flex-row w-100 h-100">
            <section class="panel w-50 h-100 flex flex-column">
                <header class="panel-head flex flex-justify-between">
                    <span class="label">02 / Recent</span>
                    <span class="label">Resume</span>
                </header>
                <div class="panel-scroll main-scrollbar">
                    <h1 class="hero">
                        Pick up where<br />
                        <span class="accent">you left off.</span>
                    </h1>

                    <div class="recent-projects">
                        {#each recentProjects as recentProject, i}
                            <button
                                class="recent-project flex flex-align-center"
                                on:click={(_) => openRecent(recentProject)}
                            >
                                <span class="recent-index"
                                    >{String(i + 1).padStart(2, "0")}</span
                                >
                                <span class="recent-path">{recentProject}</span>
                                <span class="recent-arrow">&nearr;</span>
                            </button>
                        {/each}
                        {#if recentProjects.length === 0}
                            <div class="empty label">No recent projects</div>
                        {/if}
                    </div>
                </div>
            </section>

            <section class="panel w-50 h-100 flex flex-column left-border">
                <header class="panel-head flex flex-justify-between">
                    <span class="label">03 / New Session</span>
                    <span class="label">Open Folder</span>
                </header>
                <div class="panel-scroll main-scrollbar">
                    <h1 class="hero">
                        Start a<br />
                        <span class="accent">new workspace.</span>
                    </h1>

                    <div class="field">
                        <div class="label field-label">Project Path</div>
                        <div class="path-input flex flex-align-center">
                            <input
                                class="w-100 input"
                                placeholder="/path/to/your/project"
                                bind:value={projectLocation}
                            />
                            <button
                                class="path-btn"
                                aria-label="Choose folder"
                                on:click={(_) => openFileSelector()}
                            >
                                <FolderOpen />
                            </button>
                        </div>
                    </div>

                    <button
                        class="btn btn-primary btn-block open-btn"
                        on:click={(_) => openProject()}
                    >
                        Open Project
                    </button>

                    <p class="panel-note label">
                        Open an existing folder to browse git changes and chat
                        with your local model.
                    </p>
                </div>
            </section>
        </div>
    </div>
    <div class="w-100 h-100" class:hidden={currentProject === "add"}>
        {#each openProjects as project}
            <div
                class="w-100 h-100"
                class:hidden={currentProject !== project.key}
            >
                <MainView currentProject={project.key} />
            </div>
        {/each}
    </div>
</div>

<style>
    .main-container {
        height: calc(100% - 47px);
    }

    .app-bar {
        width: 100%;
        height: 47px;
        border-bottom: 1px solid var(--border-color);
    }

    .landing {
        overflow: hidden;
    }

    .landing-meta {
        padding: 12px 22px;
        border-bottom: 1px solid var(--border-color);
    }

    .landing-body {
        flex: 1;
        min-height: 0;
    }

    .panel {
        min-width: 0;
    }

    .panel-head {
        padding: 12px 22px;
        border-bottom: 1px solid var(--border-color);
    }

    .panel-scroll {
        padding: 40px 22px;
        flex: 1;
        min-height: 0;
    }

    .hero {
        margin: 0 0 34px 0;
        font-size: clamp(30px, 4.4vw, 58px);
        line-height: 1.02;
        font-weight: 700;
        letter-spacing: -0.03em;
        color: var(--text-color);
    }

    .recent-projects {
        display: flex;
        flex-direction: column;
        border-top: 1px solid var(--border-color);
    }

    .recent-project {
        display: flex;
        gap: 14px;
        padding: 13px 4px;
        border: none;
        border-bottom: 1px solid var(--border-color);
        background: transparent;
        color: var(--text-color);
        text-align: left;
        cursor: pointer;
        width: 100%;
        transition: background 0.15s ease, padding 0.15s ease;
    }

    .recent-project:hover {
        background: var(--hover-color);
        padding-left: 12px;
    }

    .recent-index {
        color: var(--primary-color);
        font-size: 11px;
        letter-spacing: 0.1em;
        flex-shrink: 0;
        padding-top: 2px;
    }

    .recent-path {
        flex: 1;
        min-width: 0;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        font-size: 13px;
    }

    .recent-arrow {
        color: var(--muted-color);
        flex-shrink: 0;
    }

    .recent-project:hover .recent-arrow {
        color: var(--primary-color);
    }

    .empty {
        padding: 16px 4px;
    }

    .field {
        margin-bottom: 18px;
    }

    .field-label {
        margin-bottom: 8px;
    }

    .path-input {
        border: 1px solid var(--border-strong);
        background: var(--surface-color);
        height: 44px;
        transition: border-color 0.15s ease;
    }

    .path-input:focus-within {
        border-color: var(--primary-color);
    }

    .path-input .input {
        padding: 0 14px;
        height: 100%;
    }

    .path-btn {
        height: 100%;
        width: 46px;
        flex-shrink: 0;
        border: none;
        border-left: 1px solid var(--border-strong);
        background: transparent;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: background 0.15s ease;
    }

    .path-btn:hover {
        background: var(--hover-color);
    }

    .path-btn :global(svg) {
        height: 18px;
        width: 18px;
        fill: var(--muted-color);
    }

    .path-btn:hover :global(svg) {
        fill: var(--primary-color);
    }

    .open-btn {
        height: 44px;
    }

    .panel-note {
        margin-top: 22px;
        line-height: 1.7;
        white-space: normal;
        max-width: 42ch;
    }
</style>
