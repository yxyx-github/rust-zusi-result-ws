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

const isCompact = computed<boolean>(() => props.compact)

const { sizeClass } = useSize(props.size, isCompact)

const fontBoldnessClass = computed(() => isCompact.value ? 'font-normal' : 'font-semibold')

const baseButtonClass = 'inline-block rounded-xs shadow-md text-font-50 outline outline-0 focus:outline-2 disabled:cursor-not-allowed disabled:brightness-75'
const successClass = `${baseButtonClass} bg-success-500 hover:bg-success-600 active:bg-success-700 outline-success-800`
const warningClass = `${baseButtonClass} bg-warning-500 hover:bg-warning-600 active:bg-warning-700 outline-warning-800`
const errorClass = `${baseButtonClass} bg-error-500 hover:bg-error-600 active:bg-error-700 outline-error-800`
const secondaryClass = `${baseButtonClass} bg-secondary-400 hover:bg-secondary-500 active:bg-secondary-600 outline-secondary-700`
// bg-secondary-500 hover:bg-secondary-600 active:bg-secondary-700 outline-secondary-800
const primaryClass = `${baseButtonClass} bg-primary-500 hover:bg-primary-600 active:bg-primary-700 outline-primary-800`

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
    return `${typeClass} ${sizeClass.value} ${fontBoldnessClass.value}`
})

defineExpose({
    element,
})
</script>