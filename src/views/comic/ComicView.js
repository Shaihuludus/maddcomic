import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { pendingComicPath } from "../../state/appState.js";
import { t } from "../../i18n/index.js";

export function useComicView() {
  const selectedComicPath = ref("");
  const _statusKey = ref(null);
  const _statusArgs = ref([]);
  const statusMessage = computed(() =>
    _statusKey.value ? t(_statusKey.value, ..._statusArgs.value) : ""
  );
  const pageCount = ref(0);
  const firstPageDataUrl = ref("");
  const firstPageName = ref("");
  const currentPage = ref(0);
  const showLoadedOverlay = ref(false);
  let loadedOverlayTimeoutId = null;
  const isFullyVisibleMode = ref(false);
  const isComicLoadedStatus = computed(() => _statusKey.value === "comicLoaded");
  const canGoToPreviousPage = computed(() => currentPage.value > 1);
  const canGoToNextPage = computed(() => pageCount.value > 0 && currentPage.value < pageCount.value);

  function setStatus(key, ...args) {
    _statusKey.value = key;
    _statusArgs.value = args;
  }

  function resetState() {
    _statusKey.value = null;
    _statusArgs.value = [];
    pageCount.value = 0;
    firstPageDataUrl.value = "";
    firstPageName.value = "";
    currentPage.value = 0;
    showLoadedOverlay.value = false;
    if (loadedOverlayTimeoutId !== null) {
      clearTimeout(loadedOverlayTimeoutId);
      loadedOverlayTimeoutId = null;
    }
    isFullyVisibleMode.value = false;
  }

  async function applyComic(path) {
    const comicInfo = await invoke("load_comic_info", { comicPath: path });
    pageCount.value = comicInfo.file_count;
    firstPageDataUrl.value = comicInfo.first_page_data_url;
    firstPageName.value = comicInfo.first_page_name;
    currentPage.value = 1;
    setStatus("comicLoaded", comicInfo.file_count);
    showLoadedOverlay.value = true;
    loadedOverlayTimeoutId = setTimeout(() => {
      showLoadedOverlay.value = false;
      loadedOverlayTimeoutId = null;
    }, 2500);
  }

  async function loadComicByPath(path) {
    resetState();
    selectedComicPath.value = path;
    try {
      await applyComic(path);
    } catch (error) {
      setStatus("failedToOpen", String(error));
    }
  }

  watch(pendingComicPath, (path) => {
    if (path) {
      pendingComicPath.value = null;
      loadComicByPath(path);
    }
  });

  async function openComicFile() {
    resetState();
    try {
      const selectedPath = await invoke("open_comic_file");
      if (!selectedPath) {
        setStatus("selectionCanceled");
        return;
      }
      selectedComicPath.value = selectedPath;
      await applyComic(selectedPath);
    } catch (error) {
      setStatus("failedToOpen", String(error));
    }
  }

  function dismissLoadedOverlay() {
    showLoadedOverlay.value = false;
    if (loadedOverlayTimeoutId !== null) {
      clearTimeout(loadedOverlayTimeoutId);
      loadedOverlayTimeoutId = null;
    }
  }

  function toggleImageVisibilityMode() {
    if (!firstPageDataUrl.value) return;
    isFullyVisibleMode.value = !isFullyVisibleMode.value;
  }

  async function goToPage(pageNumber) {
    if (!selectedComicPath.value || pageCount.value <= 0) return;
    if (pageNumber < 1 || pageNumber > pageCount.value) return;
    try {
      const page = await invoke("load_comic_page", {
        comicPath: selectedComicPath.value,
        pageIndex: pageNumber - 1,
      });
      firstPageDataUrl.value = page.page_data_url;
      firstPageName.value = page.page_name;
      currentPage.value = pageNumber;
      _statusKey.value = null;
    } catch (error) {
      setStatus("failedToSwitchPage", String(error));
    }
  }

  async function goToPreviousPage() {
    if (!canGoToPreviousPage.value) return;
    await goToPage(currentPage.value - 1);
  }

  async function goToNextPage() {
    if (!canGoToNextPage.value) return;
    await goToPage(currentPage.value + 1);
  }

  function handleArrowNavigation(event) {
    if (!firstPageDataUrl.value) return;
    if (event.key === "ArrowLeft") {
      event.preventDefault();
      void goToPreviousPage();
    } else if (event.key === "ArrowRight") {
      event.preventDefault();
      void goToNextPage();
    }
  }

  onMounted(() => globalThis.addEventListener("keydown", handleArrowNavigation));
  onUnmounted(() => globalThis.removeEventListener("keydown", handleArrowNavigation));

  return {
    selectedComicPath,
    statusMessage,
    isComicLoadedStatus,
    showLoadedOverlay,
    pageCount,
    firstPageDataUrl,
    firstPageName,
    currentPage,
    canGoToPreviousPage,
    canGoToNextPage,
    isFullyVisibleMode,
    openComicFile,
    dismissLoadedOverlay,
    toggleImageVisibilityMode,
    goToPreviousPage,
    goToNextPage,
  };
}
