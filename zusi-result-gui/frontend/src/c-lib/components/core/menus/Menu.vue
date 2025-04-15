<template>
    <Row
            :gap="0"
            alignItems="stretch"
            class="menu bg-bg border border-secondary-200 rounded-xs overflow-auto shadow-md"
            shrink
    >
        <Column
                :gap="0"
                grow
                alignItems="stretch"
        >
            <template v-for="(item, index) in props.items" :key="index">
                <SubMenu
                        ref="subMenu"
                        v-if="item.children && item.children.length > 0"
                        :items="item.children"
                        :selected="props.selected"
                        :path="[...props.path, index]"
                        :limitByOverflowParent="props.limitByOverflowParent"
                        @updateFocus="emit('updateFocus', $event)"
                >
                    <template #trigger>
                        <MenuButtonWrapper
                                :item="{ onClick: e => onSubMenuParentClick(e, index), ...item }"
                                :hasIcon="hasIcons"
                                hasSubMenu
                                :enableFocus="JSON.stringify([...props.path, index]) === JSON.stringify(props.selected)"
                                :path="[...props.path, index]"
                                @updateFocus="emit('updateFocus', $event)"
                        >
                            <template #menuItem="{ data }" v-if="$slots.menuItem">
                                <slot name="menuItem" :data="data"/>
                            </template>
                        </MenuButtonWrapper>
                    </template>
                    <template #menuItem="{ data }" v-if="$slots.menuItem">
                        <slot name="menuItem" :data="data"/>
                    </template>
                </SubMenu>
                <MenuButtonWrapper
                        v-else
                        :item="item"
                        :hasIcon="hasIcons"
                        :hasSubMenu="false"
                        :enableFocus="JSON.stringify([...props.path, index]) === JSON.stringify(props.selected)"
                        :path="[...props.path, index]"
                        @updateFocus="emit('updateFocus', $event)"
                >
                    <template #menuItem="{ data }" v-if="$slots.menuItem">
                        <slot name="menuItem" :data="data"/>
                    </template>
                </MenuButtonWrapper>
            </template>
        </Column>
    </Row>
</template>

<script lang="ts" setup>
import Column from '@/c-lib/components/core/layout/Column.vue'
import type { MenuItem, MenuItemPath } from '@/c-lib/types/menu.ts'
import { computed, ref, watch } from 'vue'
import SubMenu from '@/c-lib/components/core/menus/sub/SubMenu.vue'
import MenuButtonWrapper from '@/c-lib/components/core/menus/MenuButtonWrapper.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'

const emit = defineEmits<{
    'updateFocus': [{ path: MenuItemPath, value: boolean }],
}>()

const props = withDefaults(defineProps<{
    items: MenuItem[]
    selected?: MenuItemPath
    path?: MenuItemPath
    limitByOverflowParent?: boolean
}>(), {
    selected: () => [],
    path: () => [],
    limitByOverflowParent: false,
})

const hasIcons = computed(() => props.items.some(item => item.icon !== undefined))

function onSubMenuParentClick(e: Event, index: number) {
    e.stopPropagation()
    emit('updateFocus', { path: [...props.path, index, 0], value: true })
}
</script>

<style scoped>
.menu > :slotted(*) {
    z-index: 0;
}

.menu > :slotted(*:focus-within) {
    z-index: 1;
}
</style>