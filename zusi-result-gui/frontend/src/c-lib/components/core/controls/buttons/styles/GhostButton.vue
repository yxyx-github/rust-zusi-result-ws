<template>
    <button ref="buttonElement" :="$attrs" v-focus="props.initialFocus" :class="buttonClass" :type="props.type" :disabled="props.disabled" :tabindex="props.focusable ? 0 : -1">
        <slot/>
    </button>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { vFocus } from '@/c-lib/directives/v-focus'
import type { ButtonStyleImplementorProps } from '@/c-lib/types/button.ts'
import { defaultButtonStyleImplementorProps } from '@/c-lib/types/defaults/button'
import { useSize } from '@/c-lib/composables/core/layout/size'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps< ButtonStyleImplementorProps & SizeProps & {
    invertFont?: boolean
}>(), {
    ...defaultButtonStyleImplementorProps,
    ...defaultSizeProps,
    invertFont: false,
})

const buttonElement = ref<HTMLElement | null>(null)

function element(): HTMLElement | null {
    return buttonElement.value
}

const { sizeClass } = useSize(props.size, () => props.compact)

const baseButtonClass = `rounded-xs rounded-xs outline outline-0 focus:outline-2 disabled:cursor-not-allowed ${props.invertFont ? 'text-font-200 active:text-font-400 disabled:text-font-500' : 'text-font-700 active:text-font-500 disabled:text-font-400'}`
const successClass = `${baseButtonClass} hover:bg-success-400 outline-success-500`
const warningClass = `${baseButtonClass} hover:bg-warning-400 outline-warning-500`
const errorClass = `${baseButtonClass} hover:bg-error-400 outline-error-500`
const secondaryClass = `${baseButtonClass} hover:bg-secondary-300 outline-secondary-400`
const primaryClass = `${baseButtonClass} hover:bg-primary-400 outline-primary-500`

const buttonClass = computed(() => {
    let typeClass = ''
    switch (props.severity) {
        case 'success':
            typeClass = successClass
            break
        case 'warning':
            typeClass = warningClass
            break
        case 'error':
            typeClass = errorClass
            break
        case 'secondary':
            typeClass = secondaryClass
            break
        default:
        case 'primary':
            typeClass = primaryClass
            break
    }
    return `${typeClass} ${sizeClass.value}`
})

defineExpose({
    element,
})
</script>