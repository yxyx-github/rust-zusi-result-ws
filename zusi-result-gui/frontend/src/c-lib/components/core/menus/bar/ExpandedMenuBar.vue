<template>
    <ButtonBar :direction="props.direction" :wrap="false">
        <template v-for="(item, index) in (props.items as MenuItem[])" :key="index">
            <Dropdown
                    v-if="(item.children ?? []).length > 0"
                    :="props"
                    :initialFocus="props.initialFocus && index === 0"
                    :items="item.children"
                    :label="item.label"
                    :showLabel="item.showLabel === undefined ? props.showLabel : item.showLabel"
                    :threshold="props.threshold"
                    :title="item.title"
                    :severity="item.severity ?? props.severity"
                    :prependIcon="item.icon"
                    :disabled="item.disabled"
                    :size="props.size"
                    :compact="props.compact"
                    textAlign="left"
            />
            <Button
                    v-else
                    :="props"
                    :initialFocus="props.initialFocus && index === 0"
                    :items="item.children"
                    :label="item.label"
                    :showLabel="item.showLabel === undefined ? props.showLabel : item.showLabel"
                    :threshold="props.threshold"
                    :title="item.title"
                    :severity="item.severity ?? props.severity"
                    :prependIcon="item.icon"
                    :disabled="item.disabled"
                    :size="props.size"
                    :compact="props.compact"
                    textAlign="left"
                    @click="item.onClick"
            />
        </template>
    </ButtonBar>
</template>

<script setup lang="ts">
import type { DropdownProps, MenuItem, SubMenuAlign } from '@/c-lib/types/menu.ts'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import Dropdown from '@/c-lib/components/core/menus/dropdown/Dropdown.vue'
import type { ButtonProps } from '@/c-lib/types/button.ts'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import { defaultDropdownProps } from '@/c-lib/types/defaults/menu'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import type { Direction } from '@/c-lib/types/flex.ts'
import { computed } from 'vue'
import type { Threshold } from '@/c-lib/types/screens.ts'

const props = withDefaults(defineProps<ButtonProps & SizeProps & DropdownProps & {
    direction?: Direction
    threshold?: Threshold
}>(), {
    ...defaultButtonProps,
    ...defaultSizeProps,
    ...defaultDropdownProps,
    direction: 'row',
    threshold: 'sm',
}) as any

const hasIcons = computed(() => (props.items as MenuItem[]).some(item => item.icon !== undefined))
</script>