<script setup>
import { ref } from "vue";
import ComicView from "./views/comic/ComicView.vue";
import LibraryView from "./views/LibraryView.vue";
import SettingsView from "./views/SettingsView.vue";

const activeTab = ref("comic");
</script>

<template>
  <v-app class="bg-blue-grey-lighten-5">
    <v-app-bar density="compact" color="primary">
      <v-tabs v-model="activeTab" align-tabs="center" class="w-100" color="secondary">
        <v-tab value="comic" selected-class="active-tab">Comic view</v-tab>
        <v-tab value="library" selected-class="active-tab">Library</v-tab>
        <v-tab value="settings" selected-class="active-tab">Settings</v-tab>
      </v-tabs>
    </v-app-bar>

    <v-main class="app-main">
      <v-window v-model="activeTab">
        <v-window-item value="comic">
          <ComicView />
        </v-window-item>

        <v-window-item value="library">
          <LibraryView />
        </v-window-item>

        <v-window-item value="settings">
          <SettingsView />
        </v-window-item>
      </v-window>
    </v-main>
  </v-app>
</template>

<style scoped src="./App.scoped.css"></style>
<style src="./App.css"></style>
