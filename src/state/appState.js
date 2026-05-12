import { ref } from "vue";

export const activeTab = ref("comic");
export const pendingComicPath = ref(null);

export const language = ref(localStorage.getItem("language") ?? "en");

export function setLanguage(lang) {
  language.value = lang;
  localStorage.setItem("language", lang);
}
