<script setup>
import { useComicView } from "./ComicView";
import Button from 'primevue/button';
import Message from 'primevue/message';

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
  <div class="comic-view">
    <div class="comic-layout">
      <div class="sidebar-panel">
        <div class="sidebar">
          <Button class="open-btn" @click="openComicFile">
            Open comic
          </Button>

          <Button
            class="open-btn"
            severity="secondary"
            :disabled="!firstPageDataUrl"
            @click="toggleImageVisibilityMode"
          >
            {{ isFullyVisibleMode ? "Zoom to width" : "Show full page" }}
          </Button>

          <Message
            v-if="statusMessage && !isComicLoadedStatus"
            :severity="statusMessage === 'Selection canceled.' ? 'info' : 'error'"
          >
            {{ statusMessage }}
          </Message>
        </div>
      </div>

      <div class="preview-panel">
        <div v-if="showLoadedOverlay && isComicLoadedStatus" class="loaded-overlay">
          <Message severity="success" closable @close="dismissLoadedOverlay">
            {{ statusMessage }}
          </Message>
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

        <div v-if="firstPageDataUrl && pageCount > 0" class="bottom-bar">
          <Button
            size="small"
            severity="secondary"
            :disabled="!canGoToPreviousPage"
            @click="goToPreviousPage"
          >
            Previous
          </Button>

          <span class="page-indicator">Page {{ currentPage }} / {{ pageCount }}</span>

          <Button
            size="small"
            severity="secondary"
            :disabled="!canGoToNextPage"
            @click="goToNextPage"
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped src="./ComicView.css"></style>
