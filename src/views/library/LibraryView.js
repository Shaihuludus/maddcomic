import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { activeTab, pendingComicPath } from "../../state/appState.js";

export function useLibraryView() {
  const folders = ref([]);
  const error = ref(null);
  const selectedFolder = ref(null);
  const comics = ref([]);
  const covers = ref({});

  async function loadFolders() {
    try {
      folders.value = await invoke("load_folders");
    } catch (e) {
      error.value = String(e);
    }
  }

  async function selectFolder(path) {
    selectedFolder.value = path;
    comics.value = [];
    covers.value = {};
    try {
      comics.value = await invoke("list_comics_in_folder", { folderPath: path });
    } catch (e) {
      error.value = String(e);
      return;
    }
    for (const comic of comics.value) {
      const comicPath = comic.path;
      invoke("load_comic_info", { comicPath })
        .then((info) => {
          if (selectedFolder.value === path) covers.value[comicPath] = info.first_page_data_url;
        })
        .catch(() => {});
    }
  }

  async function addFolder() {
    try {
      const path = await invoke("open_folder_dialog");
      if (!path) return;
      if (folders.value.some((f) => f.path === path)) return;
      folders.value.push({ path });
      await saveFolders();
    } catch (e) {
      error.value = String(e);
    }
  }

  async function removeFolder(path) {
    folders.value = folders.value.filter((f) => f.path !== path);
    if (selectedFolder.value === path) {
      selectedFolder.value = null;
      comics.value = [];
      covers.value = {};
    }
    await saveFolders();
  }

  async function saveFolders() {
    try {
      await invoke("save_folders", { folders: folders.value });
      error.value = null;
    } catch (e) {
      error.value = String(e);
    }
  }

  onMounted(loadFolders);

  function openComic(path) {
    pendingComicPath.value = path;
    activeTab.value = "comic";
  }

  return { folders, error, selectedFolder, comics, covers, addFolder, removeFolder, selectFolder, openComic };
}
