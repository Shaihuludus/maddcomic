<script setup>
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
import ComicView from "./views/comic/ComicView.vue";
import LibraryView from "./views/library/LibraryView.vue";
import SettingsView from "./views/SettingsView.vue";
import { activeTab } from "./state/appState.js";
import { useI18n } from "./i18n/index.js";

const { t } = useI18n();
</script>

<template>
  <div class="app-wrapper">
    <Tabs v-model:value="activeTab" class="app-tabs">
      <TabList class="app-tab-list">
        <Tab value="comic">{{ t('tabComic') }}</Tab>
        <Tab value="library">{{ t('tabLibrary') }}</Tab>
        <Tab value="settings">{{ t('tabSettings') }}</Tab>
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
