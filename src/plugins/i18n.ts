import en from '@/locales/en.json'
import zh from '@/locales/zh.json'
import { createI18n } from 'vue-i18n'

export default createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  messages: { en, zh },
})
