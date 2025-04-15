import type { ModalProps } from '@/c-lib/types/modal.ts'

export const defaultModalProps: ModalProps = {
    trigger: false,
    title: '',
    threshold: 'md',
    closable: true,
    resizable: true,
}