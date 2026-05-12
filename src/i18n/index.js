import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { language, setLanguage } from "../state/appState.js";

export const availableLanguages = ref([]);
const currentTranslation = ref({});

export async function initI18n() {
  try {
    availableLanguages.value = await invoke("list_languages");
  } catch (e) {
    console.error("Failed to list languages:", e);
  }
  await loadLanguage(language.value);
}

export async function loadLanguage(code) {
  try {
    currentTranslation.value = await invoke("load_language", { code });
    setLanguage(code);
  } catch (e) {
    console.error(`Failed to load language '${code}':`, e);
    if (code !== "en") await loadLanguage("en");
  }
}

export function t(key, ...args) {
  const val = currentTranslation.value[key] ?? key;
  return String(val).replace(/\{(\d+)\}/g, (_, i) => String(args[+i] ?? ""));
}

export function useI18n() {
  return { t };
}
