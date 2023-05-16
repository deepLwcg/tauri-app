<template>
  <nav>
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link>
    <el-button @click="toggleDark()">主题</el-button>
    <el-button @click="open_window()">新窗口</el-button>
  </nav>
  <router-view />
</template>
<script lang="ts" setup>
  import { toggleDark } from './composables';
  import { invoke } from '@tauri-apps/api/tauri';
  const open_window = () => {
    invoke('open_player_window', { title: '新窗口', url: 'http://127.0.0.1:1420/player' });
  };
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

  nav {
    padding: 30px;

    a {
      font-weight: bold;
      color: #2c3e50;

      &.router-link-exact-active {
        color: #42b983;
      }
    }
  }
</style>
