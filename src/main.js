import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
import { definePreset } from '@primeuix/themes'
import Aura from '@primeuix/themes/aura'
import 'primeicons/primeicons.css'
import App from './App.vue'
import { initI18n } from './i18n/index.js'

const BlueGreyPreset = definePreset(Aura, {
  semantic: {
    primary: {
      50: '#ECEFF1',
      100: '#CFD8DC',
      200: '#B0BEC5',
      300: '#90A4AE',
      400: '#78909C',
      500: '#607D8B',
      600: '#546E7A',
      700: '#455A64',
      800: '#37474F',
      900: '#263238',
      950: '#1C242A',
    },
  },
})

initI18n().then(() => {
  createApp(App)
    .use(PrimeVue, {
      theme: {
        preset: BlueGreyPreset,
        options: {
          darkModeSelector: false,
        },
      },
    })
    .mount('#app')
})
