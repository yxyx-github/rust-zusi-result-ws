<template>
    <slot name="trigger" v-if="$slots.trigger" :onContextMenu="show"/>
    <component
            :="$attrs"
            v-else
            :is="props.is"
            @contextmenu.prevent.stop="show"
    >
        <slot/>
    </component>
    <teleport to="#menus">
        <ContextMenuWrapper ref="contextMenuWrapper" v-if="state.show" :x1="state.xPosition" :y1="state.yPosition">
            <Menu @click="hide"
                  class="whitespace-nowrap"
                  :items="props.items"
                  :selected="selected"
                  @keydown.up.prevent="onUp"
                  @keydown.right.prevent="onRight"
                  @keydown.down.prevent="onDown"
                  @keydown.left.prevent="onLeft"
                  @keydown.esc.prevent="onEsc"
                  @updateFocus="onUpdateFocus"
            >
                <template #menuItem="{ data }" v-if="$slots.menuItem">
                    <slot name="menuItem" :data="data"/>
                </template>
            </Menu>
        </ContextMenuWrapper>
    </teleport>
</template>

<script lang="ts" setup>
import { h, reactive, ref } from 'vue'
import Menu from '@/c-lib/components/core/menus/Menu.vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import ContextMenuWrapper from '@/c-lib/components/core/menus/context/ContextMenuWrapper.vue'
import { useMenuKeyboardManager } from '@/c-lib/composables/core/menus/menuKeyboardManager'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<{
    is?: any
    items: MenuItem[]
}>(), {
    is: h('div'),
})

const contextMenuWrapper = ref<any>(null)

const { selected, onUp, onRight, onDown, onLeft, onEsc, onUpdateFocus } = useMenuKeyboardManager(
    props.items,
    () => contextMenuWrapper.value.element(),
    () => handleClose(),
)

const state = reactive({
    show: false,
    xPosition: 0,
    yPosition: 0,
})

function show(event: MouseEvent) {
    if (state.show) {
        state.show = false
    } else {
        state.xPosition = event.clientX
        state.yPosition = event.clientY
        state.show = true
        onDown()
        document.addEventListener('click', handleClose)
    }
}

function hide() {
    selected.value = []
    state.show = false
    document.removeEventListener('click', handleClose)
}

function handleClose() {
    hide()
}
</script>