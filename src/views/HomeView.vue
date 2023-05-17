<template>
  <div class="home">
    <el-row class="row-bg" justify="center">
      <el-col :span="16">
        <el-input v-model="search_text" class="search-input" size="large" placeholder="请输入视频名称">
          <template #append>
            <el-button :icon="Search" @click="search()" />
          </template>
        </el-input>
      </el-col>
    </el-row>
    <div class="site-list">
      <div v-for="(site, index) in sites" :key="index" class="site-item">
        <el-badge :value="site.count" class="item">
          <el-button size="large" @click="open_site(site.id)">{{ site.name }}</el-button>
        </el-badge>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue';
  import { Search } from '@element-plus/icons-vue';
  import { ElMessage } from 'element-plus';
  import { invoke } from '@tauri-apps/api/tauri';
  import { useRouter } from 'vue-router';
  const router = useRouter();

  interface Site {
    id: number;
    name: string;
    original_name: string;
    count: number | undefined;
  }

  const search_text = ref<string>('');

  const sites = ref<Site[]>([]);

  const search = () => {
    ElMessage(search_text.value);
  };

  const open_site = (id: number) => {
    router.push({
      name: 'videos',
      query: {
        id: id,
      },
    });
  };

  const get_sites = () => {
    invoke('get_sites').then((result) => {
      sites.value = <Site[]>result;
    });
  };

  onMounted(() => {
    get_sites();
  });
</script>

<style scoped lang="less">
  .home {
    padding-top: 90px;
  }

  .search-input {
    width: 100%;
  }

  .site-list {
    padding: 20px;
    width: 100%;
    box-sizing: border-box;
    text-align: center;
  }

  .site-item {
    margin: 10px 20px;
    display: inline-block;
  }
</style>
