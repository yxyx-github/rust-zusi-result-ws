import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { ConfirmDialogConfig, Dialog, DialogAction, DialogConfig } from '@/c-lib/types/dialog.ts'
import { v4 as uuidv4 } from 'uuid'
import { useI18n } from 'vue-i18n'

export const useDialogsStore = defineStore('dialogs', () => {
    const i18n = useI18n()

    const dialogs = ref<Dialog[]>([])

    function removeDialog(id: string) {
        dialogs.value = dialogs.value.filter(dialog => dialog.id !== id)
    }

    async function dialog(config: DialogConfig) {
        return new Promise<string>(resolve => {
            const id = uuidv4()
            dialogs.value.push({
                id,
                config: {
                    ...config,
                    actions: config.actions.map((action: DialogAction) => ({
                        ...action,
                        onClick: (e: Event) => {
                            removeDialog(id)
                            action.onClick?.(e)
                            resolve(action.name)
                        },
                    }))
                }
            })
        })
    }

    async function confirm(config: ConfirmDialogConfig) {
        const dialogConfig: DialogConfig = {
            title: config.title,
            content: config.content,
            actions: [
                { name: 'confirm', label: config.actionLabel ?? config.title, severity: config.actionSeverity ?? 'error' },
                { name: 'abort', label: i18n.t('c.cancel'), severity: 'secondary' },
            ],
        }
        return dialog(dialogConfig)
    }

    return {
        dialogs,
        dialog,
        confirm,
    }
})
