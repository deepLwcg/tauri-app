<template>
  <div class="setting-theme">
    <el-button :icon="Sunny" size="large" circle @click="toggleDark()" />
  </div>
  <router-view />
</template>
<script lang="ts" setup>
  import { toggleDark } from './composables';
  import { Sunny } from '@element-plus/icons-vue';
</script>

<style lang="less">
  html,
  body,
  #app {
    font-family: cgwcc, system-ui, Avenir, Helvetica, Arial, sans-serif;
    height: 100%;
    width: 100%;
    padding: 0;
    margin: 0;
  }
  .setting-theme {
    position: fixed;
    right: 20px;
    top: 20px;
  }
  .dplayer-full > .dplayer-full-in-icon {
    display: none !important;
  }
</style>
