<template>
  <div id="dplayer" ref="player"></div>
</template>
<script setup lang="ts">
  import Hls from 'hls.js';
  import DPlayer, { DPlayerEvents } from 'dplayer';
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';

  const toggle_fullscreen = () => {
    invoke('toggle_fullscreen');
  };

  const player = ref<HTMLElement | null>(null);
  onMounted(() => {
    const dp = new DPlayer({
      container: document.getElementById('dplayer'),
      lang: 'zh-cn',
      video: {
        url: 'https://vip.lz-cdn17.com/20230511/12010_751767f8/index.m3u8',
        type: 'customHls',
        customType: {
          customHls: function (video) {
            const hls = new Hls();
            hls.loadSource(video.src);
            hls.attachMedia(video);
          },
        },
      },
    });
    dp.on(<DPlayerEvents>'fullscreen', function () {
      console.log('fullscreen');
      toggle_fullscreen();
    });
    dp.on(<DPlayerEvents>'fullscreen_cancel', function () {
      console.log('fullscreen_cancel');
      toggle_fullscreen();
    });
  });
</script>

<style scoped>
  #dplayer {
    width: 100%;
    height: 100%;
    background: #2c3e50;
  }
</style>
