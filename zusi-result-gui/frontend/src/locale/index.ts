import { createI18n } from 'vue-i18n'
import deDE from './de-DE'
import enUS from './en-US'

const i18nConfig = {
    legacy: false,
    locale: 'de-DE',
    fallbackLocale: 'en-US',
    messages: {
        'de-DE': deDE.messages,
        'en-US': enUS.messages
    },
    numberFormats: {
        'de-DE': deDE.numberFormats,
        'en-US': enUS.numberFormats,
    },
    datetimeFormats: {
        'de-DE': deDE.dateTimeFormats,
        'en-US': enUS.dateTimeFormats,
    },
}

export default createI18n(i18nConfig)