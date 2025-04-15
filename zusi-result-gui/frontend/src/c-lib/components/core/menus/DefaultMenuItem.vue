<template>
    <Button
            ref="element"
            :overrideFocus="props.data.enableFocus ? true : null"
            @click="props.data.item.onClick"
            variant="menu"
            :label="props.data.item.label"
            :showLabel="props.data.item.showLabel === undefined ? defaultButtonProps.showLabel : props.data.item.showLabel"
            textAlign="left"
            :title="props.data.item.title"
            :severity="props.data.item.severity"
            :prependIcon="props.data.item.icon ?? (props.data.hasIcon ? ' ' : '')"
            :appendIcon="props.data.hasSubMenu ? 'chevron-right' : ''"
            :disabled="props.data.item.disabled"
    />
</template>
c
<script setup lang="ts">
import type { MenuItemData } from '@/c-lib/types/menu.ts'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import { ref, watch } from 'vue'
import { useActiveElement } from '@vueuse/core'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'

const props = defineProps<{
    data: MenuItemData
}>()

const element = ref<any>(null)

const activeElement = useActiveElement()

const hasFocus = ref(false)

watch(() => activeElement.value, () => {
    const newFocus = activeElement.value === element.value.element()
    if (newFocus || hasFocus.value) {
        props.data.updateFocus(newFocus)
    }
    hasFocus.value = newFocus
})
</script>