import type { Severity } from '@/c-lib/types/index.ts'

export type Message = {
    severity: Severity
    text: string
}

export type MessageOptions = {
    location?: MessageLocation
    allowOverride?: boolean
    removeOn?: RemoveOnEvent[]
    removeAfter?: number // duration in milliseconds
}

export type RemoveOnEvent = 'unmount' | string

export type MessageLocation = string

export type LocatedMessage = Required<MessageOptions> & {
    id: string,
    remove: () => void
    message: Message
}