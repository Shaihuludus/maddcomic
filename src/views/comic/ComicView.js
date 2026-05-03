import { computed, nextTick, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function useComicView() {
  const selectedComicPath = ref("");
  const statusMessage = ref("");
  const pageCount = ref(0);
  const firstPageDataUrl = ref("");
  const firstPageName = ref("");
  const currentPage = ref(0);
  const showLoadedOverlay = ref(false);
  let loadedOverlayTimeoutId = null;
  const isFullyVisibleMode = ref(false);
  const isComicLoadedStatus = computed(() => statusMessage.value.startsWith("Comic loaded."));
  const canGoToPreviousPage = computed(() => currentPage.value > 1);
  const canGoToNextPage = computed(() => pageCount.value > 0 && currentPage.value < pageCount.value);

  async function openComicFile() {
    statusMessage.value = "";
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

    try {
      const selectedPath = await invoke("open_comic_file");

      if (selectedPath) {
        selectedComicPath.value = selectedPath;

        const comicInfo = await invoke("load_comic_info", { comicPath: selectedPath });
        pageCount.value = comicInfo.file_count;
        firstPageDataUrl.value = comicInfo.first_page_data_url;
        firstPageName.value = comicInfo.first_page_name;
        currentPage.value = 1;
        statusMessage.value = `Comic loaded. ${comicInfo.file_count} pages found.`;
        showLoadedOverlay.value = true;
        loadedOverlayTimeoutId = setTimeout(() => {
          showLoadedOverlay.value = false;
          loadedOverlayTimeoutId = null;
        }, 2500);

        await nextTick();
        const comicScrollElement = document.querySelector(".comic-scroll");
        if (comicScrollElement instanceof HTMLElement) {
          comicScrollElement.focus();
        }

        return;
      }

      statusMessage.value = "Selection canceled.";
    } catch (error) {
      statusMessage.value = `Failed to open comic: ${String(error)}`;
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
    if (!firstPageDataUrl.value) {
      return;
    }

    isFullyVisibleMode.value = !isFullyVisibleMode.value;
  }

  async function goToPage(pageNumber) {
    if (!selectedComicPath.value || pageCount.value <= 0) {
      return;
    }

    if (pageNumber < 1 || pageNumber > pageCount.value) {
      return;
    }

    try {
      const page = await invoke("load_comic_page", {
        comicPath: selectedComicPath.value,
        pageIndex: pageNumber - 1,
      });

      firstPageDataUrl.value = page.page_data_url;
      firstPageName.value = page.page_name;
      currentPage.value = pageNumber;
      statusMessage.value = "";
    } catch (error) {
      statusMessage.value = `Failed to switch page: ${String(error)}`;
    }
  }

  async function goToPreviousPage() {
    if (!canGoToPreviousPage.value) {
      return;
    }

    await goToPage(currentPage.value - 1);
  }

  async function goToNextPage() {
    if (!canGoToNextPage.value) {
      return;
    }

    await goToPage(currentPage.value + 1);
  }

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