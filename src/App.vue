<template>
  <textarea v-model="content" rows="20" class="content-area" />
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";

const content = ref<string>("");

onMounted(() => {
  appWindow.listen("new-content", () => {
    console.log("new-content event emitted");
    content.value = "";
  });
  appWindow.listen("open-file", async () => {
    try {
      const filePath = await open({
        title: "Select a Text File",
        filters: [{ name: "Text", extensions: ["txt"] }],
      });
      if (!filePath) return;
      const fileContent = await readTextFile(filePath as string, {});
      content.value = fileContent;
    } catch (error) {
      console.error(error);
    }
  });
});
</script>

<style scoped>
.content-area {
  width: 100%;
}
</style>
