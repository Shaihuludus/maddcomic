<script setup>
import { computed } from "vue";
import Select from "primevue/select";
import { language} from "../state/appState.js";
import { useI18n, availableLanguages, loadLanguage } from "../i18n/index.js";

const { t } = useI18n();

const currentLang = computed({
  get: () => language.value,
  set: (code) => loadLanguage(code),
});
</script>

<template>
  <div class="settings-view">
    <div class="content-card">
      <h2>{{ t('settings') }}</h2>
      <div class="setting-row">
        <label for="language-select" class="setting-label">{{ t('language') }}</label>
        <Select
          input-id="language-select"
          v-model="currentLang"
          :options="availableLanguages"
          option-label="name"
          option-value="code"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 24px;
}

.content-card {
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 24px;
  background: #fff;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 16px;
}

.setting-label {
  font-size: 0.9rem;
  color: #495057;
  min-width: 80px;
}
</style>
