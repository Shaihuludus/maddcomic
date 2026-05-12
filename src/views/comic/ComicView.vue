<script setup>
import { useComicView } from "./ComicView";
import Button from 'primevue/button';
import Message from 'primevue/message';
import { useI18n } from '../../i18n/index.js';

const { t } = useI18n();

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
} = useComicView();
</script>

<template>
  <div class="comic-view">
    <div class="comic-layout">
      <div class="sidebar-panel">
        <div class="sidebar">
          <Button class="open-btn" @click="openComicFile">
            {{ t('openComic') }}
          </Button>

          <Button
            class="open-btn"
            severity="secondary"
            :disabled="!firstPageDataUrl"
            @click="toggleImageVisibilityMode"
          >
            {{ isFullyVisibleMode ? t('zoomToWidth') : t('showFullPage') }}
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
            {{ t('previous') }}
          </Button>

          <span class="page-indicator">{{ t('pageIndicator', currentPage, pageCount) }}</span>

          <Button
            size="small"
            severity="secondary"
            :disabled="!canGoToNextPage"
            @click="goToNextPage"
          >
            {{ t('next') }}
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped src="./ComicView.css"></style>
