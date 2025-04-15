<template>
    <SubMenu
            ref="menu"
            :items="props.items"
            :align="props.align"
            :preferredXDirection="props.preferredXDirection"
            :preferredYDirection="props.preferredYDirection"
            :position="props.position"
            :selected="selected"
            :trigger="props.trigger"
            :limitByOverflowParent="props.limitByOverflowParent"
            @click="onAnyClick"
            @keydown.up.prevent="onUp"
            @keydown.right.prevent="onRight"
            @keydown.down.prevent="onDown"
            @keydown.left.prevent="onLeft"
            @keydown.esc.prevent="onEsc"
            @updateFocus="onUpdateFocus"
    >
        <template #trigger>
            <slot name="trigger">
                <Button v-if="$slots.default"
                        class="grow"
                        :="props"
                        ref="button"
                        :prependIcon="props.prependIcon === null && props.appendIcon === '' ? 'more-vertical' : props.prependIcon"
                        @click.stop="onClick"
                >
                    <slot/>
                </Button>
                <Button v-else
                        class="grow"
                        :="props"
                        ref="button"
                        :prependIcon="props.prependIcon === null && props.appendIcon === '' ? 'more-vertical' : props.prependIcon"
                        @click.stop="onClick"
                />
            </slot>
        </template>
        <template #menuItem="{ data }" v-if="$slots.menuItem">
            <slot name="menuItem" :data="data"/>
        </template>
    </SubMenu>
</template>

<script setup lang="ts">
import SubMenu from '@/c-lib/components/core/menus/sub/SubMenu.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import type { DropdownProps } from '@/c-lib/types/menu.ts'
import type { ButtonProps } from '@/c-lib/types/button.ts'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'
import { ref, watch } from 'vue'
import { useMenuKeyboardManager } from '@/c-lib/composables/core/menus/menuKeyboardManager'
import { defaultDropdownProps } from '@/c-lib/types/defaults/menu'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

const props = withDefaults(defineProps<Omit<ButtonProps, 'prependIcon'> & SizeProps & DropdownProps & {
    prependIcon?: string | null
}>(), {
    ...defaultButtonProps,
    ...defaultSizeProps,
    ...defaultDropdownProps,
    prependIcon: null,
}) as any

const button = ref<any>(null)
const menu = ref<any>(null)

const { selected, onUp, onRight, onDown, onLeft, onEsc, onUpdateFocus } = useMenuKeyboardManager(
    props.items,
    () => menu.value.element(),
    () => button.value?.element()?.focus(),
)

watch(() => selected.value.length, (newValue, oldValue) => {
    if (oldValue === 0) {
        document.addEventListener('click', onAnyClick)
    } else if (newValue === 0) {
        document.removeEventListener('click', onAnyClick)
    }
})

function onClick() {
    if (props.trigger === 'click') {
        if (selected.value.length > 0) {
            onAnyClick()
        } else {
            onDown()
        }
    }
}

function onAnyClick() {
    selected.value = []
    menu.value.clearHover()
}
</script>