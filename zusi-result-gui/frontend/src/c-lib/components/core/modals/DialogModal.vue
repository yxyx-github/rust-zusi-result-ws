<template>
    <Modal v-model:show="show" :title="props.config.title" :closable="false" :resizable="false" :trigger="props.trigger">
        <template #trigger v-if="props.trigger">
            <slot name="trigger"/>
        </template>
        <slot>
            {{ props.config.content }}
        </slot>
        <MenuBar initialFocus :items="props.config.actions" collapseThreshold="mb" buttonLabelThreshold="mb"/>
    </Modal>
</template>

<script setup lang="ts">
import MenuBar from '@/c-lib/components/core/menus/bar/MenuBar.vue'
import Modal from '@/c-lib/components/core/modals/Modal.vue'
import type { DialogConfig } from '@/c-lib/types/dialog.ts'

const props = withDefaults(defineProps<{
    config: DialogConfig
    trigger?: boolean
}>(), {
    trigger: false,
})

const show = defineModel<boolean>('show', { default: false })
</script>