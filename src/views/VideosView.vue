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
        <el-card :body-style="{ padding: '0px' }">
          <img :src="video.pic" class="image" />
          <div style="padding: 14px">
            <span>{{ video.name }}</span>
          </div>
        </el-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useRoute, useRouter } from 'vue-router';
  import { Back } from '@element-plus/icons-vue';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMounted, ref } from 'vue';
  import { s } from '@tauri-apps/api/shell-efff51a2';

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
  .video-item {
    display: inline-block;
  }
</style>
