<script setup>
import { useComicView } from "./ComicView";

const {
  statusMessage,
  isComicLoadedStatus,
  showLoadedOverlay,
  pageCount,
  currentPage,
  canGoToPreviousPage,
  canGoToNextPage,
  isFullyVisibleMode,
  firstPageDataUrl,
  openComicFile,
  toggleImageVisibilityMode,
  dismissLoadedOverlay,
  goToPreviousPage,
  goToNextPage,
  handleArrowNavigation,
} = useComicView();
</script>

<template>
  <v-container fluid class="comic-view pa-0">
    <div class="comic-layout">
      <v-sheet class="sidebar-panel pa-2" rounded="0">
        <div class="sidebar d-flex flex-column ga-3 align-start">
          <v-btn color="primary" size="default" density="compact" class="open-btn" @click="openComicFile">
          Open comic
          </v-btn>

          <v-btn
            color="primary"
            size="default"
            density="compact"
            variant="tonal"
            class="open-btn"
            :disabled="!firstPageDataUrl"
            @click="toggleImageVisibilityMode"
          >
            {{ isFullyVisibleMode ? "Zoom to width" : "Show full page" }}
          </v-btn>

          <v-alert
            v-if="statusMessage && !isComicLoadedStatus"
            :type="statusMessage === 'Selection canceled.' ? 'info' : 'error'"
            variant="tonal"
            density="comfortable"
          >
            {{ statusMessage }}
          </v-alert>
        </div>
      </v-sheet>

      <v-sheet class="preview-panel" rounded="0">
        <div v-if="showLoadedOverlay && isComicLoadedStatus" class="loaded-overlay pa-3">
          <v-alert type="success" variant="tonal" closable @click:close="dismissLoadedOverlay">
            {{ statusMessage }}
          </v-alert>
        </div>

        <div
          v-if="firstPageDataUrl"
          class="comic-scroll"
          :class="{ 'full-visible-mode': isFullyVisibleMode }"
          tabindex="0"
          @keydown="handleArrowNavigation"
        >
          <img
            :src="firstPageDataUrl"
            alt="Comic page"
            class="comic-image"
            :class="{ 'fully-visible-image': isFullyVisibleMode }"
          />
        </div>

        <v-sheet v-if="firstPageDataUrl && pageCount > 0" class="bottom-bar px-3 py-2" rounded="0">
          <v-btn
            size="small"
            variant="tonal"
            color="primary"
            :disabled="!canGoToPreviousPage"
            @click="goToPreviousPage"
          >
            Previous
          </v-btn>

          <span class="page-indicator">Page {{ currentPage }} / {{ pageCount }}</span>

          <v-btn
            size="small"
            variant="tonal"
            color="primary"
            :disabled="!canGoToNextPage"
            @click="goToNextPage"
          >
            Next
          </v-btn>
        </v-sheet>
      </v-sheet>
    </div>
  </v-container>
</template>

<style scoped src="./ComicView.scoped.css"></style>