<template>
    <DialogModal v-model:show="show" :config="dialogConfig" :trigger="props.trigger">
        <template #trigger v-if="props.trigger">
            <slot name="trigger"/>
        </template>
        <slot/>
    </DialogModal>
</template>

<script setup lang="ts">
import type { ButtonProps } from '@/c-lib/types/button.ts'
import type { ModalProps } from '@/c-lib/types/modal.ts'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'
import { defaultModalProps } from '@/c-lib/types/defaults/modal'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import DialogModal from '@/c-lib/components/core/modals/DialogModal.vue'
import { computed } from 'vue'
import type { DialogConfig } from '@/c-lib/types/dialog.ts'
import { useI18n } from 'vue-i18n'

const i18n = useI18n()

const emit = defineEmits(['confirm', 'abort'])

const props = withDefaults(defineProps<ButtonProps & SizeProps & ModalProps>(), {
    ...defaultButtonProps,
    ...defaultSizeProps,
    ...defaultModalProps,
    title: 'Confirm',
    closable: false,
    resizable: false,
})

const show = defineModel<boolean>('show', { default: false })

const dialogConfig = computed<DialogConfig>(() => ({
    title: props.title,
    content: '',
    actions: [
        { name: 'confirm', label: props.title, severity: props.severity, onClick: confirm },
        { name: 'abort', label: i18n.t('c.cancel'), severity: 'secondary', onClick: abort },
    ],
}))

function confirm() {
    emit('confirm')
    show.value = false
}

function abort() {
    emit('abort')
    show.value = false
}
</script>