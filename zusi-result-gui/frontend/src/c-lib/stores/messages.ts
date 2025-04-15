import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { LocatedMessage, Message, MessageLocation, MessageOptions } from '@/c-lib/types/message.ts'
import { v4 as uuidv4 } from 'uuid'
import { defaultMessageOptions } from '@/c-lib/types/defaults/message'

function fillOptionDefaults(options: MessageOptions): Required<MessageOptions> {
    return {
        ...defaultMessageOptions,
        ...options,
    }
}

export const useMessagesStore = defineStore('messages', () => {
    const currentMessages = ref<LocatedMessage[]>([])

    function overrideMessage(location: MessageLocation) {
        currentMessages.value.find(message => message.location === location && message.allowOverride)?.remove()
    }

    function add(message: Message, options: MessageOptions): LocatedMessage {
        const finalOptions = fillOptionDefaults(options)
        overrideMessage(finalOptions.location)
        const newMessage: LocatedMessage = {
            id: uuidv4(),
            remove: () => currentMessages.value = currentMessages.value.filter(m => m.id !== newMessage.id),
            location: finalOptions.location,
            allowOverride: finalOptions.allowOverride,
            removeOn: finalOptions.removeOn,
            removeAfter: finalOptions.removeAfter,
            message,
        }
        currentMessages.value.push(newMessage)
        return newMessage
    }

    function set(messages: Message[], options: MessageOptions): LocatedMessage[] {
        clear(options.location)
        const addedMessages: LocatedMessage[] = []
        messages.forEach(message => addedMessages.push(add(message, options)))
        return addedMessages
    }

    function clear(location?: MessageLocation) {
        if (location === undefined) {
            currentMessages.value = []
        } else {
            currentMessages.value = currentMessages.value.filter(m => m.location !== location)
        }
    }

    function messages(location?: MessageLocation): LocatedMessage[] {
        if (location === undefined) {
            return currentMessages.value
        } else {
            return currentMessages.value.filter(message => message.location === location)
        }
    }

    function hasMessages(location?: MessageLocation): boolean {
        if (location === undefined) {
            return currentMessages.value.length > 0
        } else {
            return currentMessages.value.some(message => message.location === location)
        }
    }

    function remove(event: string, location?: MessageLocation) {
        currentMessages.value = currentMessages.value.filter(message =>
            !message.removeOn.includes(event) ||
            !(location === undefined || message.location === location)
        )
    }

    return {
        add,
        set,
        clear,
        messages,
        hasMessages,
        remove,
    }
})
