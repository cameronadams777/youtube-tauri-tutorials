<template>
  <div class="custom-title-bar">
    <div class="title-bar-buttons">
      <button class="title-bar-button" @click="() => appWindow.close()">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </button>
      <button
        class="title-bar-button"
        @click="() => appWindow.toggleMaximize()"
      >
        <img
          src="https://api.iconify.design/mdi:window-maximize.svg"
          alt="maximize"
        />
      </button>
      <button class="title-bar-button" @click="() => appWindow.minimize()">
        <img
          src="https://api.iconify.design/mdi:window-minimize.svg"
          alt="minimize"
        />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
</script>

<style>
.custom-title-bar {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 25px;
  display: flex;
  justify-content: space-between;
  background-color: white;
}

.title-bar-buttons {
  display: flex;
  align-items: center;
}

.title-bar-button {
  width: 30px;
  height: 25px;
  display: flex;
  justify-content: center;
  align-items: center;
  border: none;
  background-color: white;
  color: white;
}

.title-bar-button:hover {
  background-color: blue;
  color: white;
}
</style>
