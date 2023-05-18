<template>
  <div class="videos-view">
    <div class="back">
      <el-button :icon="Back" size="large" circle @click="router.back()" />
    </div>
    <div class="video-type-list">
      <div v-for="menu in menus" :key="menu.id" class="video-type-item">
        <el-button round text size="small">{{ menu.name }}</el-button>
      </div>
    </div>
    <div v-if="videos" class="video-list">
      <div v-for="video in videos.videos" :key="video.id" class="video-item">
        <div class="video-content" @click="open_video(video.id, video.name)">
          <div class="video-img">
            <img :src="video.pic" :alt="video.name" />
          </div>
          <div class="video-title">{{ video.name }}</div>
          <div class="video-note">{{ video.note }}</div>
          <div class="video-actor">{{ video.actor }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useRoute, useRouter } from 'vue-router';
  import { Back } from '@element-plus/icons-vue';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMounted, ref } from 'vue';

  interface Menu {
    id: number;
    name: string;
  }

  interface Page {
    page: number;
    page_count: number;
    page_size: number;
    record_count: number;
    videos: Video[] | undefined;
  }

  interface Video {
    id: number;
    tid: number;
    name: string;
    type: string;
    pic: string;
    lang: string;
    year: string;
    note: string;
    actor: string;
    last: string;
  }
  const route = useRoute();
  const router = useRouter();
  const id = route.query.id;

  const menus = ref<Menu[]>([]);

  const videos = ref<Page>();

  const get_videos_type = () => {
    invoke('get_videos_type', { id: Number(id) }).then((data) => {
      menus.value = <Menu[]>data;
    });
  };

  const get_videos = () => {
    invoke('get_videos', { id: Number(id) }).then((data) => {
      console.log(data);
      videos.value = <Page>data;
    });
  };

  const open_video = (vid: number, title: string) => {
    invoke('open_player_window', { title: title, url: '/player?id=' + id + '&vid=' + vid });
  };

  onMounted(() => {
    get_videos_type();
    get_videos();
  });
</script>

<style scoped lang="less">
  .back {
    position: fixed;
    left: 20px;
    top: 20px;
  }
  .videos-view {
    padding-top: 50px;
  }
  .video-type-list {
    padding: 20px;
  }
  .video-type-item {
    display: inline-block;
    margin: 5px;
  }
  .video-list {
    padding: 10px 20px;
  }
  .video-list::after {
    display: block;
    content: ' ';
    clear: both;
  }
  .video-item {
    width: 20%;
    float: left;
  }
  .video-content {
    margin: 5px 5px;
    cursor: pointer;
    overflow: hidden;
    line-height: 24px;
  }
  .video-img {
    width: 100%;
    height: 235px;
    line-height: 235px;
    overflow: hidden;
    border-radius: 5px;
    background-color: #222;
  }
  .video-img > img {
    width: 100%;
    vertical-align: middle;
  }
  .video-title {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    margin: 3px 0 2px 0;
  }
  .video-actor,
  .video-note {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    font-size: 10px;
    color: #6b6b6b;
  }
</style>
