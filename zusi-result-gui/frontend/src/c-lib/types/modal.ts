import type { Threshold } from '@/c-lib/types/screens.ts'

export type ModalProps = {
    trigger?: boolean
    title?: string
    threshold?: Threshold
    closable?: boolean
    resizable?: boolean
}