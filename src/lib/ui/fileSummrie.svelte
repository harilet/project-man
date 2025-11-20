<script lang="ts">
  export let data: string;

  function getFilename() {
    console.log(data);
    return JSON.parse(data)["file"];
  }

  function getContent() {
    return JSON.parse(data)["response"]["changes"];
  }
</script>

<div class="file-summrie-card full-border">
  <div>
    {getFilename()}
  </div>
  <div class="file-summrie-changes">
    {#each getContent() as contentObj}
      <div class="file-summrie-change-item">
        <div class="flex">
          <div>
            {contentObj["action"]}:
          </div>
          <div style="margin-left: 5px;">
            {contentObj["target"]}
          </div>
        </div>

        <div>
          {contentObj["reason"]}
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .file-summrie-card {
    margin: 4px;
    padding: 2px;
  }

  .file-summrie-changes {
    margin-left: 10px;
  }

  .file-summrie-change-item{
    margin: 3px;
  }
</style>
