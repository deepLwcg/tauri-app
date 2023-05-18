<template>
  <div ref="player" class="dplayer"></div>
</template>
<script setup lang="ts">
  import Hls from 'hls.js';
  import DPlayer, { DPlayerEvents } from 'dplayer';
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  const props = defineProps({
    url: { type: String, required: true },
  });
  const toggle_fullscreen = () => {
    invoke('toggle_fullscreen');
  };
  const player = ref<HTMLElement | null>(null);

  const play_video = (url: string) => {
    const dp = new DPlayer({
      container: <HTMLElement | null>player.value,
      lang: 'zh-cn',
      video: {
        url: url,
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
  };

  onMounted(() => {
    console.log(props.url);
    play_video(props.url);
  });
</script>

<style scoped lang="less">
  .dplayer {
    width: 100%;
    height: 100%;
    background: #2c3e50;
  }
</style>
