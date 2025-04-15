<template>
    <button
            :="$attrs"
            :class="`aspect-square ${sizeClass} flex flex-row justify-center items-center outline outline-2 rounded-xs disabled:outline-secondary-200 disabled:text-font-300 ${outlineSeverityClass}`"
            v-focus="props.initialFocus"
            :disabled="props.disabled"
            @keydown.ctrl.delete="clear"
            @click="toggle"
            @keydown.enter.prevent="submit"
            type="button"
    >
        <Icon :class="`font-inherit ${iconClass}`" :name="iconName"/>
    </button>
</template>

<script setup lang="ts">
import { vFocus } from '@/c-lib/directives/v-focus'
import { defaultCheckboxProps, defaultInputProps } from '@/c-lib/types/defaults/input'
import type { CheckboxProps, InputProps, TextInputProps } from '@/c-lib/types/input.ts'
import Icon from '@/c-lib/components/core/icons/Icon.vue'
import { computed } from 'vue'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<InputProps & CheckboxProps & SizeProps>(), {
    ...defaultInputProps,
    ...defaultCheckboxProps,
    ...defaultSizeProps,
})

const value = defineModel<boolean | null>({ required: true })

const sizeClass = computed(() => {
    switch (props.size) {
        case 'sm':
            return `${props.compact ? 'h-5 text-sm' : 'h-7 text-lg'}`
        case 'md':
            return `${props.compact ? 'h-6 text-base' : 'h-9 text-xl'}`
        case 'lg':
            return `${props.compact ? 'h-7 text-lg' : 'h-11 text-2xl'}`
    }
})

const outlineSeverityClass = computed(() => {
    switch (props.severity) {
        case 'success':
            return 'outline-success-400 hover:outline-success-600 focus:outline-success-700'
        case 'warning':
            return 'outline-warning-400 hover:outline-warning-600 focus:outline-warning-700'
        case 'error':
            return 'outline-error-400 hover:outline-error-600 focus:outline-error-700'
        case 'secondary':
            return 'outline-secondary-300 hover:outline-secondary-500 focus:outline-secondary-600'
        default:
        case 'primary':
            return 'outline-primary-400 hover:outline-primary-600 focus:outline-primary-700'
    }
})

const iconClass = computed(() => props.grayScaleFalse && value.value === false ? 'grayscale opacity-20' : '')

const iconName = computed(() => value.value === null ? props.nullIcon : (value.value ? props.trueIcon : props.falseIcon))

function toggle() {
    value.value = value.value === null ? true : !value.value
}

function submit(e: Event) {
    (e.target as HTMLFormElement).form?.requestSubmit()
}

function clear() {
    if (props.default !== undefined) {
        value.value = props.default
    }
}
</script>