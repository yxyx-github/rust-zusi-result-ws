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

const props = withDefaults(defineProps<ButtonStyleImplementorProps & SizeProps>(), {
    ...defaultButtonStyleImplementorProps,
    ...defaultSizeProps,
})

const buttonElement = ref<HTMLElement | null>(null)

function element(): HTMLElement | null {
    return buttonElement.value
}

const { sizeClass } = useSize(props.size, () => props.compact)

const baseButtonClass = 'rounded-xs text-left active:text-font-500 focus:outline-hidden border-2 border-bg disabled:bg-bg disabled:text-font-300 disabled:font-normal disabled:cursor-not-allowed'
const successClass = `${baseButtonClass} hover:bg-success-300 hover:border-success-300 focus:border-success-800`
const warningClass = `${baseButtonClass} hover:bg-warning-300 hover:border-warning-300 focus:border-warning-800`
const errorClass = `${baseButtonClass} hover:bg-error-300 hover:border-error-300 focus:border-error-800`
const secondaryClass = `${baseButtonClass} hover:bg-secondary-300 hover:border-secondary-300 focus:border-secondary-700`
const primaryClass = `${baseButtonClass} hover:bg-primary-300 hover:border-primary-300 focus:border-primary-800`

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
        case 'primary':
            typeClass = primaryClass
            break
        default:
        case 'secondary':
            typeClass = secondaryClass
            break
    }
    return `${typeClass} ${sizeClass.value}`
})

defineExpose({
    element,
})
</script>