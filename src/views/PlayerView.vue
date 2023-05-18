<template>
  <div class="player-view">
    <video-player v-if="play_url" :url="play_url" />
  </div>
</template>

<script setup lang="ts">
  import VideoPlayer from '../components/VideoPlayer.vue';
  import { invoke } from '@tauri-apps/api/tauri';
  import { useRoute } from 'vue-router';
  import { onMounted, ref } from 'vue';

  const route = useRoute();
  const id = route.query.id;
  const vid = route.query.vid;

  interface Detail {
    id: number;
    name: string;
    pic: string;
    year: string;
    type: string;
    tid: number;
    note: string;
    last: string;
    lang: string;
    actor: string;
    director: string;
    des: string;
    dl: Dl[];
  }

  interface Dl {
    flag: string;
    addrs: Addr[];
  }

  interface Addr {
    title: string;
    url: string;
  }

  const play_url = ref<string>();

  const detail = ref<Detail | undefined>();

  const get_m3u8_addr = (): Addr[] => {
    let dl = detail.value?.dl;
    for (let index in dl) {
      let addrs = dl[index].addrs;
      for (let i in addrs) {
        if (addrs[i].url.indexOf('.m3u8') > 0) {
          return addrs;
        }
      }
    }
    return [];
  };

  const open_first_addr = () => {
    let m3u8Addr = get_m3u8_addr();
    if (m3u8Addr.length > 0) {
      play_url.value = m3u8Addr[0].url;
    }
  };

  const open_detail = () => {
    invoke('open_video', { id: Number(id), vid: Number(vid) }).then((data) => {
      detail.value = <Detail>data;
      open_first_addr();
      console.log(play_url.value);
    });
  };
  onMounted(() => {
    open_detail();
  });
</script>

<style scoped lang="less">
  .player-view {
    height: 100%;
  }
</style>
