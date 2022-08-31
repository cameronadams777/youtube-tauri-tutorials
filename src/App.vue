<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const num = ref(0);

const double = async () => {
  try {
    const response = await invoke("double", { test: num.value });
    console.log(response);
  } catch (err) {
    console.error(err);
  }
};
</script>

<template>
  <div class="form-container">
    <input v-model="num" type="number" />
    <button @click="double">Send Ping</button>
  </div>
</template>
