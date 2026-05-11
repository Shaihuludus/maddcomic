<script setup>
import { ref } from "vue";
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
import ComicView from "./views/comic/ComicView.vue";
import LibraryView from "./views/library/LibraryView.vue";
import SettingsView from "./views/SettingsView.vue";

const activeTab = ref("comic");
</script>

<template>
  <div class="app-wrapper">
    <Tabs v-model:value="activeTab" class="app-tabs">
      <TabList class="app-tab-list">
        <Tab value="comic">Comic view</Tab>
        <Tab value="library">Library</Tab>
        <Tab value="settings">Settings</Tab>
      </TabList>
      <TabPanels class="app-panels">
        <TabPanel value="comic">
          <ComicView />
        </TabPanel>
        <TabPanel value="library">
          <LibraryView />
        </TabPanel>
        <TabPanel value="settings">
          <SettingsView />
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
</template>

<style scoped src="./AppOnly.css"></style>
<style src="./App.css"></style>
