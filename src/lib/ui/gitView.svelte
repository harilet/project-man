<script lang="ts">
  export let currentProject: string;
  import { invoke } from "@tauri-apps/api/core";
  import Add from "$lib/icons/add.svelte";
  import Close from "$lib/icons/close.svelte";

  type BranchInfo = { name: string; is_head: boolean; is_remote: boolean };
  type CommitInfo = {
    id: string;
    short_id: string;
    summary: string;
    author: string;
    time: number;
  };

  let stagedFiles: string[] = [];
  let unStagedFiles: string[] = [];
  let fileDiff = "";

  let branches: BranchInfo[] = [];
  let currentBranch = "";
  let commitLog: CommitInfo[] = [];

  let commitMessage = "";
  let showHistory = false;
  let busy = false;
  let lastProject = "";
  let statusMessage = "";

  $: localBranches = branches.filter((branch) => !branch.is_remote);
  $: remoteBranches = branches.filter((branch) => branch.is_remote);

  let branchDialog: HTMLDialogElement;
  let branchDialogMode: "create" | "rename" = "create";
  let branchNameInput = "";
  let branchSourceName = "";
  let checkoutAfterCreate = true;

  let confirmDialog: HTMLDialogElement;
  let branchToDelete = "";

  $: if (
    currentProject &&
    currentProject !== "add" &&
    currentProject !== lastProject
  ) {
    lastProject = currentProject;
    loadAll();
  }

  function loadAll() {
    loadChanges();
    loadBranches();
    loadLog();
  }

  function loadChanges() {
    invoke("get_staged_files", { location: currentProject }).then(
      (data: any) => (stagedFiles = data),
    );
    invoke("get_unstaged_files", { location: currentProject }).then(
      (data: any) => (unStagedFiles = data),
    );
  }

  function loadBranches() {
    invoke("get_branches", { location: currentProject }).then(
      (data: any) => (branches = data),
    );
    invoke("get_current_branch_name", { location: currentProject }).then(
      (data: any) => (currentBranch = data),
    );
  }

  function loadLog() {
    invoke("get_commit_log", { location: currentProject, limit: 50 }).then(
      (data: any) => (commitLog = data),
    );
  }

  function refresh() {
    loadAll();
  }

  function getChanges(file: string) {
    invoke("get_file_diff", {
      location: currentProject,
      file: file,
      isUnified: false,
    }).then((data: any) => (fileDiff = data));
  }

  function getUnstagedChanges(file: string) {
    invoke("get_unstaged_file_diff", {
      location: currentProject,
      file: file,
    }).then((data: any) => (fileDiff = data));
  }

  function addFile(e: any, path: string) {
    e.stopPropagation();
    invoke("add_file_index", { location: currentProject, path }).then(() =>
      loadChanges(),
    );
  }

  function removeFileFromIndex(e: any, path: string) {
    e.stopPropagation();
    invoke("remove_file_index", { location: currentProject, path }).then(() =>
      loadChanges(),
    );
  }

  function discardFile(e: any, path: string) {
    e.stopPropagation();
    invoke("discard_file_changes", { location: currentProject, path }).then(
      () => {
        fileDiff = "";
        loadChanges();
      },
    );
  }

  function commit() {
    if (!canCommit) return;
    busy = true;
    invoke("commit", { location: currentProject, message: commitMessage })
      .then(() => {
        commitMessage = "";
        fileDiff = "";
        loadChanges();
        loadLog();
      })
      .finally(() => (busy = false));
  }

  function amendCommit() {
    if (!canCommit) return;
    busy = true;
    invoke("amend_commit", { location: currentProject, message: commitMessage })
      .then(() => {
        commitMessage = "";
        fileDiff = "";
        loadChanges();
        loadLog();
      })
      .finally(() => (busy = false));
  }

  function switchBranch(e: any) {
    const name = e.target.value;
    if (!name || name === currentBranch) return;

    const isRemote = remoteBranches.some((branch) => branch.name === name);
    const command = isRemote ? "checkout_remote_branch" : "checkout_branch";

    busy = true;
    invoke(command, { location: currentProject, name })
      .then((output: any) => {
        statusMessage = output || (isRemote ? `Checked out ${name}` : "");
        fileDiff = "";
        loadAll();
      })
      .finally(() => (busy = false));
  }

  function openCreateDialog() {
    branchDialogMode = "create";
    branchNameInput = "";
    checkoutAfterCreate = true;
    branchDialog.showModal();
  }

  function openRenameDialog() {
    if (!currentBranch) return;
    branchDialogMode = "rename";
    branchSourceName = currentBranch;
    branchNameInput = currentBranch;
    branchDialog.showModal();
  }

  function submitBranchDialog() {
    const name = branchNameInput.trim();
    if (!name) return;

    if (branchDialogMode === "create") {
      invoke("create_branch", {
        location: currentProject,
        name: name,
        checkout: checkoutAfterCreate,
      }).then(() => {
        branchDialog.close();
        fileDiff = "";
        loadAll();
      });
    } else {
      invoke("rename_branch", {
        location: currentProject,
        oldName: branchSourceName,
        newName: name,
      }).then(() => {
        branchDialog.close();
        loadAll();
      });
    }
  }

  function openDeleteDialog() {
    if (!currentBranch) return;
    branchToDelete = currentBranch;
    confirmDialog.showModal();
  }

  function confirmDelete() {
    invoke("delete_branch", {
      location: currentProject,
      name: branchToDelete,
    }).then(() => {
      confirmDialog.close();
      fileDiff = "";
      loadAll();
    });
  }

  function remoteAction(action: "fetch" | "pull" | "push") {
    busy = true;
    statusMessage = "";
    const command =
      action === "fetch"
        ? "fetch_repo"
        : action === "pull"
          ? "pull_repo"
          : "push_repo";

    const args: any = { location: currentProject, remote: "origin" };
    if (action !== "fetch") {
      args.branch = currentBranch;
    }

    invoke(command, args)
      .then((output: any) => {
        statusMessage = output || `${action} complete`;
        loadBranches();
        if (action !== "fetch") {
          loadChanges();
          loadLog();
        }
      })
      .finally(() => (busy = false));
  }

  function printFileChangeLine(line: string) {
    if (line != "") {
      let obj = JSON.parse(line);
      if (obj["from_no"] == obj["to_no"]) {
        return (
          obj["change_type"] + ": " + obj["from_no"] + ": " + obj["content"]
        );
      } else if (obj["from_no"] == "") {
        return obj["change_type"] + ": " + obj["to_no"] + ": " + obj["content"];
      } else if (obj["to_no"] == "") {
        return (
          obj["change_type"] + ": " + obj["from_no"] + ": " + obj["content"]
        );
      } else {
        return (
          obj["change_type"] +
          ": " +
          obj["from_no"] +
          "->" +
          obj["to_no"] +
          ": " +
          obj["content"]
        );
      }
    }
  }

  function getChangeLineColor(line: string) {
    if (line != "") {
      let obj = JSON.parse(line);
      if (obj["change_type"] == "+") {
        return "#00be7e4d";
      } else if (obj["change_type"] == "-") {
        return "#ff1d5733";
      } else {
        return "";
      }
    }
  }

  function formatTime(seconds: number) {
    return new Date(seconds * 1000).toLocaleString();
  }

  $: canCommit = !busy && stagedFiles.length > 0;
</script>

<div class="git-root flex flex-column h-100 w-100">
  <div class="git-toolbar flex flex-justify-between flex-align-center bottom-border">
    <div class="flex flex-align-center toolbar-group">
      <span class="label">Branch</span>
      <select
        class="branch-select"
        value={currentBranch}
        on:change={switchBranch}
        disabled={busy}
      >
        <optgroup label="Local">
          {#each localBranches as branch}
            <option value={branch.name}
              >{branch.name}{branch.is_head ? " *" : ""}</option
            >
          {/each}
        </optgroup>
        {#if remoteBranches.length > 0}
          <optgroup label="Remote">
            {#each remoteBranches as branch}
              <option value={branch.name}>{branch.name}</option>
            {/each}
          </optgroup>
        {/if}
      </select>
      <button class="btn btn-ghost" on:click={openCreateDialog}>+ New</button>
      <button class="btn btn-ghost" on:click={openRenameDialog}>Rename</button>
      <button class="btn btn-ghost" on:click={openDeleteDialog}>Delete</button>
    </div>
    <div class="flex flex-align-center toolbar-group">
      <button
        class="btn btn-ghost"
        disabled={busy}
        on:click={() => remoteAction("fetch")}>Fetch</button
      >
      <button
        class="btn btn-ghost"
        disabled={busy}
        on:click={() => remoteAction("pull")}>Pull</button
      >
      <button
        class="btn btn-ghost"
        disabled={busy}
        on:click={() => remoteAction("push")}>Push</button
      >
      <button class="btn btn-ghost" on:click={refresh}>Refresh</button>
      <button
        class="btn btn-ghost {showHistory ? 'active' : ''}"
        on:click={() => (showHistory = !showHistory)}>History</button
      >
    </div>
  </div>

  {#if busy || statusMessage}
    <div class="git-status label flex flex-align-center bottom-border">
      {#if busy}<span class="status-busy">Working&hellip;</span>{/if}
      {#if statusMessage}<span class="status-text">{statusMessage}</span>{/if}
    </div>
  {/if}

  <div class="flex git-main w-100">
    <div class="files-col flex flex-column right-border h-100">
      <div class="panel-head flex flex-justify-between flex-align-center">
        <span class="label">Staged</span>
        <span class="label">{stagedFiles.length}</span>
      </div>
      <div class="main-scrollbar file-list">
        {#each stagedFiles as file}
          <button class="commit-file-path hover" on:click={() => getChanges(file)}>
            <div class="file-name">{file}</div>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              class="file-action"
              title="Unstage"
              on:click={(e) => removeFileFromIndex(e, file)}
            >
              <Close />
            </div>
          </button>
        {/each}
        {#if stagedFiles.length === 0}
          <div class="empty label">Nothing staged</div>
        {/if}
      </div>

      <div class="panel-head flex flex-justify-between flex-align-center top-border">
        <span class="label">Unstaged</span>
        <span class="label">{unStagedFiles.length}</span>
      </div>
      <div class="main-scrollbar file-list">
        {#each unStagedFiles as file}
          <button
            class="commit-file-path hover"
            on:click={() => getUnstagedChanges(file)}
          >
            <div class="file-name">{file}</div>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="file-action" title="Discard changes" on:click={(e) => discardFile(e, file)}>
              <span class="discard-glyph">&#8635;</span>
            </div>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="file-action" title="Stage" on:click={(e) => addFile(e, file)}>
              <Add />
            </div>
          </button>
        {/each}
        {#if unStagedFiles.length === 0}
          <div class="empty label">No changes</div>
        {/if}
      </div>

      <div class="commit-box flex flex-column top-border">
        <textarea
          class="input commit-input"
          placeholder="Commit message"
          bind:value={commitMessage}
        ></textarea>
        <div class="flex commit-actions">
          <button
            class="btn btn-primary commit-btn"
            disabled={!canCommit}
            on:click={commit}>Commit</button
          >
          <button
            class="btn commit-btn"
            disabled={!canCommit}
            on:click={amendCommit}>Amend</button
          >
        </div>
      </div>
    </div>

    <div class="flex flex-column diff-col h-100">
      <div class="panel-head flex flex-justify-between flex-align-center">
        <span class="label">Diff</span>
        <span class="label">Changes</span>
      </div>
      <div class="main-scrollbar" style="flex: 1; min-height: 0;">
        {#each fileDiff.split("\n") as line}
          <pre
            style="background-color: {getChangeLineColor(line)};"
            class="hover m-0 text-wrap-wrap diff-line">{printFileChangeLine(
                line,
              )}</pre>
        {/each}
      </div>
    </div>
  </div>

  {#if showHistory}
    <div class="history flex flex-column top-border">
      <div class="panel-head flex flex-justify-between flex-align-center">
        <span class="label">History</span>
        <span class="flex flex-align-center history-head-right">
          <span class="label">{commitLog.length} commits</span>
        </span>
      </div>
      <div class="main-scrollbar history-list">
        {#each commitLog as commitEntry}
          <div class="commit-item flex flex-align-center">
            <span class="commit-id">{commitEntry.short_id}</span>
            <span class="commit-summary">{commitEntry.summary}</span>
            <span class="commit-author label">{commitEntry.author}</span>
            <span class="commit-time label">{formatTime(commitEntry.time)}</span>
          </div>
        {/each}
        {#if commitLog.length === 0}
          <div class="empty label">No commits yet</div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<dialog bind:this={branchDialog} on:close>
  <div class="dialog-pad">
    <div class="dialog-head">
      <span class="label"
        >{branchDialogMode === "create" ? "New Branch" : "Rename Branch"}</span
      >
      <span class="dialog-title"
        >{branchDialogMode === "create"
          ? "Create a branch"
          : "Rename branch"}</span
      >
    </div>
    <div class="dialog-body">
      <div class="field">
        <div class="label field-label">Branch Name</div>
        <input
          class="w-100 input setting-input"
          placeholder="feature/my-branch"
          bind:value={branchNameInput}
          on:keydown={(e) => e.key === "Enter" && submitBranchDialog()}
        />
      </div>
      {#if branchDialogMode === "create"}
        <label class="checkbox-row flex flex-align-center">
          <input type="checkbox" bind:checked={checkoutAfterCreate} />
          <span>Checkout after creating</span>
        </label>
      {/if}
      <div class="flex dialog-actions">
        <button class="btn btn-ghost w-50" on:click={() => branchDialog.close()}
          >Cancel</button
        >
        <button class="btn btn-primary w-50" on:click={submitBranchDialog}
          >{branchDialogMode === "create" ? "Create" : "Rename"}</button
        >
      </div>
    </div>
  </div>
</dialog>

<dialog bind:this={confirmDialog} on:close>
  <div class="dialog-pad">
    <div class="dialog-head">
      <span class="label">Delete Branch</span>
      <span class="dialog-title">Are you sure?</span>
    </div>
    <div class="dialog-body">
      <p class="dialog-note">
        This will permanently delete the branch
        <span class="accent">{branchToDelete}</span>.
      </p>
      <div class="flex dialog-actions">
        <button class="btn btn-ghost w-50" on:click={() => confirmDialog.close()}
          >Cancel</button
        >
        <button class="btn btn-danger w-50" on:click={confirmDelete}
          >Delete</button
        >
      </div>
    </div>
  </div>
</dialog>

<style>
  .git-root {
    overflow: hidden;
  }

  .git-toolbar {
    padding: 8px 12px;
    gap: 12px;
    background: var(--surface-color);
    flex: 0 0 auto;
  }

  .git-status {
    padding: 6px 14px;
    gap: 10px;
    background: var(--background-color);
    flex: 0 0 auto;
    overflow: hidden;
  }

  .status-busy {
    color: var(--primary-color);
  }

  .status-text {
    color: var(--muted-color);
    text-transform: none;
    letter-spacing: 0.02em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .history-head-right {
    gap: 12px;
  }

  .history-close {
    border: none;
    background: transparent;
    color: var(--muted-color);
    display: flex;
    align-items: center;
    padding: 0;
    cursor: pointer;
  }

  .history-close:hover {
    color: var(--text-color);
  }

  .history-close :global(svg) {
    height: 13px;
    width: 13px;
    fill: currentColor;
  }

  .toolbar-group {
    gap: 6px;
  }

  .branch-select {
    appearance: none;
    -webkit-appearance: none;
    background-color: var(--background-color);
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'><path d='M0 0l5 6 5-6z' fill='%23888888'/></svg>");
    background-repeat: no-repeat;
    background-position: right 10px center;
    border: 1px solid var(--border-strong);
    color: var(--text-color);
    font-family: inherit;
    font-size: 12px;
    padding: 7px 28px 7px 10px;
    min-width: 160px;
    max-width: 240px;
    cursor: pointer;
    outline: none;
  }

  .branch-select option {
    background: var(--surface-color);
    color: var(--text-color);
  }

  .branch-select optgroup {
    background: var(--surface-color);
    color: var(--muted-color);
  }

  .branch-select:focus {
    border-color: var(--primary-color);
  }

  .btn.active {
    border-color: var(--primary-color);
    color: var(--primary-color);
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .git-main {
    flex: 1;
    min-height: 0;
  }

  .files-col {
    width: 340px;
    flex-shrink: 0;
    min-width: 0;
  }

  .diff-col {
    flex: 1;
    min-width: 0;
  }

  .panel-head {
    padding: 10px 14px;
    background: var(--surface-color);
    flex: 0 0 auto;
  }

  .file-list {
    flex: 1;
    min-height: 0;
  }

  .commit-file-path {
    padding: 9px 14px;
    cursor: pointer;
    border-radius: 0px;
    background: transparent;
    color: var(--text-color);
    border: none;
    border-bottom: 1px solid var(--border-color);
    width: -webkit-fill-available;
    text-align: start;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
    transition: background 0.15s ease;
  }

  .file-name {
    font-size: 12px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    direction: rtl;
    text-align: left;
    flex: 1;
    min-width: 0;
  }

  .file-action {
    flex-shrink: 0;
    color: var(--muted-color);
    display: flex;
    align-items: center;
  }

  .file-action:hover {
    color: var(--primary-color);
  }

  .file-action :global(svg) {
    height: 15px;
    width: 15px;
  }

  .discard-glyph {
    font-size: 15px;
    line-height: 1;
  }

  .empty {
    padding: 16px 14px;
  }

  .commit-box {
    flex: 0 0 auto;
    padding: 12px;
    gap: 10px;
    background: var(--surface-color);
  }

  .commit-input {
    border: 1px solid var(--border-strong);
    background: var(--background-color);
    padding: 10px 12px;
    min-height: 74px;
    font-size: 13px;
    resize: none;
    width: 100%;
  }

  .commit-input:focus {
    border-color: var(--primary-color);
  }

  .commit-actions {
    gap: 8px;
  }

  .commit-btn {
    flex: 1;
  }

  .diff-line {
    font-size: 12px;
    padding: 0 14px;
  }

  .history {
    flex: 0 0 auto;
    height: 210px;
    background: var(--surface-color);
  }

  .history-list {
    flex: 1;
    min-height: 0;
  }

  .commit-item {
    gap: 12px;
    padding: 8px 14px;
    border-bottom: 1px solid var(--border-color);
    font-size: 12px;
  }

  .commit-item:hover {
    background: var(--hover-color);
  }

  .commit-id {
    color: var(--primary-color);
    font-size: 11px;
    letter-spacing: 0.08em;
    flex-shrink: 0;
  }

  .commit-summary {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .commit-author,
  .commit-time {
    flex-shrink: 0;
  }

  .btn-danger {
    border-color: #d84a4a;
    color: #d84a4a;
  }

  .btn-danger:hover {
    background: #d84a4a;
    border-color: #d84a4a;
    color: #ffffff;
  }

  .dialog-head {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 22px;
    border-bottom: 1px solid var(--border-color);
  }

  .dialog-title {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--text-color);
  }

  .dialog-body {
    padding: 22px;
  }

  .dialog-note {
    margin: 0 0 20px 0;
    font-size: 13px;
    line-height: 1.6;
    color: var(--text-color);
  }

  .field {
    margin-bottom: 18px;
  }

  .field-label {
    margin-bottom: 8px;
  }

  .setting-input {
    padding: 11px 14px;
    border: 1px solid var(--border-strong);
    background: var(--background-color);
    transition: border-color 0.15s ease;
  }

  .setting-input:focus {
    border-color: var(--primary-color);
  }

  .checkbox-row {
    gap: 10px;
    font-size: 12px;
    color: var(--text-color);
    margin-bottom: 20px;
    cursor: pointer;
  }

  .dialog-actions {
    gap: 10px;
  }
</style>
