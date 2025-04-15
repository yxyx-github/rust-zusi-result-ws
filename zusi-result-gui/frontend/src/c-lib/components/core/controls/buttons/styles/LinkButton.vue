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

const props = withDefaults(defineProps<ButtonStyleImplementorProps & SizeProps & {
    narrow?: boolean
}>(), {
    ...defaultButtonStyleImplementorProps,
    ...defaultSizeProps,
    narrow: false,
})

const buttonElement = ref<HTMLElement | null>(null)

function element(): HTMLElement | null {
    return buttonElement.value
}

const { sizeClass, ySizeClass, fontSizeClass } = useSize(props.size, () => props.compact)
const buttonSizeClass = computed(() => props.narrow ? `${ySizeClass.value} ${fontSizeClass.value}` : sizeClass.value)

const baseButtonClass = 'focus:no-underline focus:outline-hidden disabled:cursor-not-allowed  disabled:text-font-300'
const successClass = `${baseButtonClass} text-success-600 focus:text-success-700 hover:text-success-700 active:text-success-900`
const warningClass = `${baseButtonClass} text-warning-600 focus:text-warning-700 hover:text-warning-700 active:text-warning-900`
const errorClass = `${baseButtonClass} text-error-600 focus:text-error-700 hover:text-error-700 active:text-error-900`
const secondaryClass = `${baseButtonClass} text-secondary-500 focus:text-secondary-700 hover:text-secondary-700 active:text-secondary-900`
const primaryClass = `${baseButtonClass} text-primary-600 focus:text-primary-700 hover:text-primary-700 active:text-primary-900`

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
    return `${typeClass} ${buttonSizeClass.value}`
})

defineExpose({
    element,
})
</script>