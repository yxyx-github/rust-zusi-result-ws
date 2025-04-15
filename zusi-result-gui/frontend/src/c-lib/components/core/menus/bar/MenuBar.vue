<template>
    <div v-if="props.direction === 'row'">
        <div :class="smallClass">
            <Dropdown :="props"
                      :prependIcon="props.prependIcon === '' && props.appendIcon === '' ? 'menu' : props.prependIcon"
                      :items="dropdownMenuItems"
                      :align="props.align === null ? 'vertical' : props.align"
                      :size="props.size"
                      :compact="props.compact"
            />
        </div>
        <div :class="largeClass">
            <ExpandedMenuBar :="props" :items="props.items" :align="props.align === null ? 'vertical' : props.align" :threshold="props.buttonLabelThreshold"/>
        </div>
    </div>
    <ExpandedMenuBar v-else :="props" :items="props.items" :align="props.align === null ? 'horizontal' : props.align" :threshold="props.buttonLabelThreshold"/>
</template>

<script setup lang="ts">
import type { ButtonProps } from '@/c-lib/types/button.ts'
import type { SizeProps } from '@/c-lib/types/size.ts'
import type { DropdownProps, MenuItem, SubMenuAlign } from '@/c-lib/types/menu.ts'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import { defaultDropdownProps } from '@/c-lib/types/defaults/menu'
import Dropdown from '@/c-lib/components/core/menus/dropdown/Dropdown.vue'
import { computed } from 'vue'
import ExpandedMenuBar from '@/c-lib/components/core/menus/bar/ExpandedMenuBar.vue'
import type { Direction } from '@/c-lib/types/flex.ts'
import { useThresholdClass } from '@/c-lib/composables/core/layout/thresholdClass'
import type { Threshold } from '@/c-lib/types/screens.ts'

const props = withDefaults(defineProps<ButtonProps & SizeProps & Omit<DropdownProps, 'align'> & {
    direction?: Direction
    align?: SubMenuAlign | null
    collapseThreshold?: Threshold
    buttonLabelThreshold?: Threshold
}>(), {
    ...defaultButtonProps,
    ...defaultSizeProps,
    ...defaultDropdownProps,
    direction: 'row',
    align: null,
    collapseThreshold: 'sm',
    buttonLabelThreshold: 'sm',
})

const dropdownMenuItems = computed<MenuItem[]>(() => mapDropdownMenuItem(props.items))

function mapDropdownMenuItem(menuItems: MenuItem[]): MenuItem[] {
    return menuItems.map(item => ({
        ...item,
        showLabel: true,
        children: item.children === undefined ? item.children : mapDropdownMenuItem(item.children),
    }))
}

const { smallClass, largeClass } = useThresholdClass(props.collapseThreshold, 'hidden', 'block')
</script>