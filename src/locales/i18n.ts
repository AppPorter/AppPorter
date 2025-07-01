import de from '@/locales/de.json'
import en from '@/locales/en.json'
import es from '@/locales/es.json'
import fr from '@/locales/fr.json'
import ja from '@/locales/ja.json'
import ko from '@/locales/ko.json'
import ru from '@/locales/ru.json'
import zhHans from '@/locales/zh-hans.json'
import zhHant from '@/locales/zh-hant.json'
import type { LanguageType } from '#/LanguageType'
import { createI18n } from 'vue-i18n'

export function setupI18n(initialLanguage: LanguageType) {
  return createI18n({
    legacy: false,
    locale: initialLanguage,
    fallbackLocale: 'en',
    messages: { en, 'zh-hans': zhHans, 'zh-hant': zhHant, fr, de, es, ja, ko, ru },
  })
}
