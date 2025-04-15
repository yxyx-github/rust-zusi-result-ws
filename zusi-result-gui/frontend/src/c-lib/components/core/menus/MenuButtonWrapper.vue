<template>
    <slot
            name="menuItem"
            :data="data"
    >
        <DefaultMenuItem :data="data"/>
    </slot>
</template>

<script setup lang="ts">
import type { MenuItem, MenuItemData, MenuItemPath } from '@/c-lib/types/menu.ts'
import DefaultMenuItem from '@/c-lib/components/core/menus/DefaultMenuItem.vue'
import { computed } from 'vue'

const emit = defineEmits<{
    'updateFocus': [{ path: MenuItemPath, value: boolean }],
}>()

const props = defineProps<{
    item: MenuItem
    hasIcon: boolean
    hasSubMenu: boolean
    enableFocus: boolean
    path: MenuItemPath
}>()

const data = computed(() => ({ item: props.item, hasIcon: props.hasIcon, hasSubMenu: props.hasSubMenu, enableFocus: props.enableFocus, updateFocus: (value) => updateFocus(value) } as MenuItemData))

function updateFocus(value: boolean) {
    emit('updateFocus', { path: props.path, value: value })
}
</script>