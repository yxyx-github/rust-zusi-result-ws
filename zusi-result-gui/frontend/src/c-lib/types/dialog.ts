import type { MenuItem } from '@/c-lib/types/menu.ts'
import type { Severity } from '@/c-lib/types/index.ts'

export type Dialog = {
    id: string
    config: DialogConfig
}

export type DialogConfig = {
    title: string
    content: string
    actions: DialogAction[]
}

export type DialogAction = MenuItem & {
    children?: undefined
    name: string
    onClick?: (e: Event) => void
}

export type ConfirmDialogConfig = {
    title: string
    content: string
    actionLabel?: string
    actionSeverity?: Severity
    actions?: undefined
}