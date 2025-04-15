import { messages as cMessages } from '../../c-lib/locale/de-DE/index'

const messages = {
    address: 'Adresse',
    c: cMessages,
}

const numberFormats = {
    currency: {
        style: 'currency',
        currency: 'EUR',
    },
    decimal: {
        style: 'decimal',
    },
}

const dateTimeFormats = {
    date: {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
    },
    datetime: {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
    },
}

const deDE = {
    messages,
    numberFormats,
    dateTimeFormats,
}

export default deDE