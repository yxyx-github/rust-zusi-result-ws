<template>
    <ButtonBar>
        <Dropdown
                :preferredXDirection="props.preferredXDirection"
                :preferredYDirection="props.preferredYDirection"
                :trigger="props.trigger"
                :position="props.position"
                :limitByOverflowParent="props.limitByOverflowParent"
                :size="props.size"
                :compact="props.compact"
                :items="menuItems"
                :label="dropdownLabel"
                :disabled="props.disabled"
                prependIcon=""
                @keydown.ctrl.delete="clear"
        >
            <template #trigger v-if="$slots.trigger">
                <slot name="trigger"/>
            </template>
            <template #menuItem="{ data }" v-if="$slots.menuItem">
                <slot name="menuItem" :data="data"/>
            </template>
        </Dropdown>
    </ButtonBar>
</template>

<script setup lang="ts">
import type { InputProps, SelectItem, SelectProps } from '@/c-lib/types/input.ts'
import { defaultInputProps, defaultSelectProps } from '@/c-lib/types/defaults/input'
import Dropdown from '@/c-lib/components/core/menus/dropdown/Dropdown.vue'
import { computed, watch } from 'vue'
import type { DropdownProps, MenuItem } from '@/c-lib/types/menu.ts'
import { defaultDropdownProps } from '@/c-lib/types/defaults/menu'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'

const props = withDefaults(defineProps<Omit<DropdownProps, 'items'> & InputProps & SelectProps & SizeProps>(), {
    ...defaultDropdownProps,
    ...defaultInputProps,
    ...defaultSelectProps,
    ...defaultSizeProps,
} as any) as any

const value = defineModel<SelectItem | null>({ required: true })

watch(() => props.items, () => {
    if (!(props.items as SelectItem[]).some(item => item.value === value.value?.value)) {
        clear()
    }
})

const menuItems = computed<MenuItem[]>(() => (props.items as SelectItem[]).map(selectItem => ({
    label: selectItem.label ?? selectItem.value,
    onClick: () => value.value = selectItem,
})))

const dropdownLabel = computed(() => value.value === null ? '...' : value.value.label ?? value.value.value)

function clear() {
    value.value = props.default
}
</script>