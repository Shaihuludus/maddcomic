<script setup>
import { ref } from "vue";
import { useLibraryView } from "./LibraryView.js";
import Accordion from "primevue/accordion";
import AccordionPanel from "primevue/accordionpanel";
import AccordionHeader from "primevue/accordionheader";
import AccordionContent from "primevue/accordioncontent";
import Button from "primevue/button";
import Message from "primevue/message";
import { useI18n } from "../../i18n/index.js";

const { folders, error, selectedFolder, comics, covers, addFolder, removeFolder, selectFolder, openComic } = useLibraryView();
const { t } = useI18n();

const sidebarEl = ref(null);
const sidebarWidth = ref(null);

function startResize(e) {
  e.preventDefault();
  const startX = e.clientX;
  const startWidth = sidebarEl.value.offsetWidth;

  function onMouseMove(e) {
    sidebarWidth.value = Math.max(160, startWidth + (e.clientX - startX));
  }

  function onMouseUp() {
    document.removeEventListener("mousemove", onMouseMove);
    document.removeEventListener("mouseup", onMouseUp);
  }

  document.addEventListener("mousemove", onMouseMove);
  document.addEventListener("mouseup", onMouseUp);
}

function comicBaseName(name) {
  return name.replace(/\.[^.]+$/, "");
}
</script>

<template>
  <div class="library-view">
    <div class="library-layout">
      <div
        ref="sidebarEl"
        class="sidebar-panel"
        :style="sidebarWidth ? { width: sidebarWidth + 'px' } : {}"
      >
        <div class="sidebar">
          <Accordion value="folders">
            <AccordionPanel value="folders">
              <AccordionHeader>{{ t('folders') }}</AccordionHeader>
              <AccordionContent>
                <Button class="add-folder-btn" @click="addFolder">
                  <i class="pi pi-plus" /> {{ t('addFolder') }}
                </Button>
                <div v-if="folders.length > 0" class="folder-list">
                  <div
                    v-for="folder in folders"
                    :key="folder.path"
                    class="folder-item"
                    :class="{ selected: selectedFolder === folder.path }"
                    @click="selectFolder(folder.path)"
                  >
                    <i class="pi pi-folder folder-icon" />
                    <span class="folder-path" :title="folder.path">{{ folder.path }}</span>
                    <Button
                      class="remove-btn"
                      severity="secondary"
                      text
                      size="small"
                      @click.stop="removeFolder(folder.path)"
                    >
                      <i class="pi pi-times" />
                    </Button>
                  </div>
                </div>
              </AccordionContent>
            </AccordionPanel>
          </Accordion>

          <Message v-if="error" severity="error">{{ error }}</Message>
        </div>
      </div>

      <div class="resize-handle" @mousedown="startResize" />

      <div class="main-panel">
        <template v-if="selectedFolder">
          <div v-if="comics.length > 0" class="comic-grid">
            <div
              v-for="comic in comics"
              :key="comic.path"
              class="comic-tile"
              :title="comic.name"
              @dblclick="openComic(comic.path)"
            >
              <div class="tile-cover">
                <img v-if="covers[comic.path]" :src="covers[comic.path]" class="tile-img" alt="" />
                <i v-else class="pi pi-book tile-icon" />
              </div>
              <span class="tile-name">{{ comicBaseName(comic.name) }}</span>
            </div>
          </div>
          <div v-else class="empty-state">
            <i class="pi pi-inbox empty-icon" />
            <p>{{ t('noComicsInFolder') }}</p>
          </div>
        </template>
        <div v-else class="empty-state">
          <i class="pi pi-folder-open empty-icon" />
          <p>{{ t('selectFolderHint') }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.library-view {
  height: 100%;
}

.library-layout {
  display: flex;
  height: 100%;
}

.sidebar-panel {
  width: fit-content;
  min-width: 160px;
  background: rgba(255, 255, 255, 0.6);
  overflow-y: auto;
  overflow-x: hidden;
  flex-shrink: 0;
}

.sidebar {
  display: flex;
  flex-direction: column;
  padding: 12px;
  gap: 8px;
}

.add-folder-btn {
  width: 100%;
}

.folder-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-top: 6px;
}

.folder-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  border-radius: 6px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  cursor: pointer;
  user-select: none;
}

.folder-item:hover {
  background: #e9ecef;
}

.folder-item.selected {
  background: #dbeafe;
  border-color: #93c5fd;
}

.folder-icon {
  color: #6c757d;
  flex-shrink: 0;
}

.folder-item.selected .folder-icon {
  color: #2563eb;
}

.folder-path {
  flex: 1;
  font-size: 0.8rem;
  white-space: nowrap;
  color: #343a40;
}

.remove-btn {
  flex-shrink: 0;
  padding: 2px 4px;
}

.resize-handle {
  width: 5px;
  cursor: col-resize;
  background: #dee2e6;
  flex-shrink: 0;
  transition: background 0.15s;
}

.resize-handle:hover {
  background: #adb5bd;
}

.main-panel {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.comic-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 16px;
}

.comic-tile {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px 8px;
  border-radius: 8px;
  border: 1px solid #e9ecef;
  background: #fff;
  cursor: pointer;
  user-select: none;
  transition: box-shadow 0.15s, border-color 0.15s;
}

.comic-tile:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
  border-color: #93c5fd;
}

.tile-cover {
  width: 80px;
  height: 110px;
  background: #f1f5f9;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #e2e8f0;
}

.tile-icon {
  font-size: 2rem;
  color: #94a3b8;
}

.tile-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 4px;
}

.tile-name {
  font-size: 0.75rem;
  text-align: center;
  color: #374151;
  word-break: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.empty-state {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #6c757d;
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 12px;
}
</style>
