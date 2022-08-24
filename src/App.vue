<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";

const contents = ref("");

const readFileContents = async () => {
  try {
    const selectedPath = await open({
      multiple: false,
    });
    if (!selectedPath) return;
    contents.value = await readTextFile(selectedPath as string);
  } catch (err) {
    console.error(err);
  }
};
</script>

<template>
  <p v-if="contents.length">{{ contents }}</p>
  <button @click="readFileContents">Open File Explorer</button>
</template>
