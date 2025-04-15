import type { MessageOptions } from '@/c-lib/types/message.ts'

export const defaultMessageOptions: Required<MessageOptions> = {
    location: '',
    allowOverride: false,
    removeOn: [],
    removeAfter: -1,
}