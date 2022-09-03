<script setup lang="ts">
import { save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const message = ref("");

const saveFileContents = async () => {
  try {
    const savePath = await save();
    if (!savePath) return;
    await invoke("save_file", { path: savePath, contents: message.value });

    /**
     * Another option for  saving files but with a hard coded file name and directory:
     * await writeTextFile("test.txt", message.value, {
          dir: BaseDirectory.Desktop,
       });
     */
  } catch (err) {
    console.error(err);
  }
};
</script>

<template>
  <div class="form-container">
    <textarea v-model="message" rows="10" />
    <button @click="saveFileContents">Save File</button>
  </div>
</template>

<style>
.form-container {
  display: flex;
  flex-direction: column;
}
</style>
