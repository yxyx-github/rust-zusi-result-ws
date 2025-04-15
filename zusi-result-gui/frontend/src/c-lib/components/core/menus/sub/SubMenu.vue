<template>
    <div
            @mouseenter="hovered = true"
            @mouseleave="hovered = false"
            ref="menuWrapper"
            class="relative flex flex-col flex-nowrap items-stretch">
        <slot name="trigger"/>
        <ContextMenuWrapper
                v-if="show"
                :x1="coords.x1"
                :y1="coords.y2"
                :x2="coords.x2"
                :y2="coords.y1"
                :absX="coords.absX"
                :absY="coords.absY"
                :preferredXDirection="props.preferredXDirection"
                :preferredYDirection="props.preferredYDirection"
                :position="props.position"
                :limitByOverflowParent="props.limitByOverflowParent"
        >
            <Menu
                    class="whitespace-nowrap"
                    :items="props.items"
                    :selected="props.selected"
                    :path="props.path"
                    :limitByOverflowParent="props.limitByOverflowParent"
                    @updateFocus="emit('updateFocus', $event)"
            >
                <template #menuItem="{ data }" v-if="$slots.menuItem">
                    <slot name="menuItem" :data="data"/>
                </template>
            </Menu>
        </ContextMenuWrapper>
    </div>
</template>

<script lang="ts" setup>
import Menu from '@/c-lib/components/core/menus/Menu.vue'
import { ref, computed } from 'vue'
import type { MenuItem, MenuItemPath, MenuTrigger, SubMenuAlign } from '@/c-lib/types/menu.ts'
import ContextMenuWrapper from '@/c-lib/components/core/menus/context/ContextMenuWrapper.vue'
import type { SubMenuXAlignDirection, SubMenuYAlignDirection } from '@/c-lib/types/menu.ts'
import type { MenuPosition } from '@/c-lib/types/menu.ts'
import { useElementBounding } from '@vueuse/core'

type Coords = {
    x1: number
    y1: number
    x2: number
    y2: number
    absX: number
    absY: number
}

const emit = defineEmits<{
    'updateFocus': [{ path: MenuItemPath, value: boolean }],
}>()

const props = withDefaults(defineProps<{
    items: MenuItem[]
    align?: SubMenuAlign
    preferredXDirection?: SubMenuXAlignDirection
    preferredYDirection?: SubMenuYAlignDirection
    selected?: MenuItemPath
    path?: MenuItemPath
    trigger?: MenuTrigger
    position?: MenuPosition
    limitByOverflowParent?: boolean
}>(), {
    align: 'horizontal',
    preferredXDirection: 'right',
    preferredYDirection: 'bottom',
    selected: () => [],
    path: () => [],
    trigger: 'hover',
    position: 'fixed',
    limitByOverflowParent: false,
})

const menuWrapper = ref<HTMLElement | null>(null)

const hovered = ref(false)

const show = computed(() =>
    (hovered.value && props.trigger === 'hover')
    || (
        (props.path.length !== 0 || props.selected.length !== 0) &&
        JSON.stringify(props.path) === JSON.stringify(props.selected.slice(0, props.path.length))
    )
)

const { x, y, width, height} = useElementBounding(menuWrapper)

const coords = computed<Coords>(() => {
    const coords: Coords = {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
        absX: 0,
        absY: 0,
    }

    const x1 = x.value
    const y1 = y.value
    const x2 = x1 + width.value
    const y2 = y1 + height.value

    if (props.align === 'vertical') {
        coords.x1 = x2
        coords.y1 = y2
        coords.x2 = x1
        coords.y2 = y1
    } else {
        coords.x1 = x1
        coords.y1 = y1
        coords.x2 = x2
        coords.y2 = y2
    }
    coords.absX = x1
    coords.absY = y1

    return coords
})

function element() {
    return menuWrapper.value
}

function clearHover() {
    hovered.value = false
}

defineExpose({
    element,
    clearHover,
})
</script>