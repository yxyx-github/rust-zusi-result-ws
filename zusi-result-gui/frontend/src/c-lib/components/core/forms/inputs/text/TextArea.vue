<template>
    <InputWrapper :size="props.size" :compact="props.compact" :disabled="props.disabled" @clear="clear">
        <textarea
                :="$attrs"
                :class="`${inputClass} font-inherit ${sizeClass} w-full`"
                v-focus="props.initialFocus"
                :disabled="props.disabled"
                v-model="value"
                ref="element"
                @input="resize"
                :rows="props.rows"
                :cols="props.cols"
        />
    </InputWrapper>
</template>

<script setup lang="ts">
import { useSize } from '@/c-lib/composables/core/layout/size'
import { vFocus } from '@/c-lib/directives/v-focus'
import { defaultInputProps, defaultTextAreaProps } from '@/c-lib/types/defaults/input'
import type { InputProps, TextAreaProps } from '@/c-lib/types/input.ts'
import InputWrapper from '@/c-lib/components/core/forms/InputWrapper.vue'
import { ref } from 'vue'
import { useInputClass } from '@/c-lib/composables/core/inputs/inputClass'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<InputProps & TextAreaProps & SizeProps>(), {
    ...defaultInputProps,
    ...defaultTextAreaProps,
    ...defaultSizeProps,
})

const value = defineModel<string>({ required: true })

const { sizeClass } = useSize(props.size, props.compact)

const inputClass = useInputClass(props.severity)

const element = ref<HTMLTextAreaElement | null>(null)

function resize() {
    if (props.autoResize && element.value !== null) {
        element.value.style.height = 'auto'
        element.value.style.height = `${element.value.scrollHeight}px`
        if (parseFloat(element.value.style.height) >= parseFloat(element.value.style.maxHeight)) {
            element.value.style.overflowY = 'scroll'
            element.value.style.height = element.value.style.maxHeight
        } else {
            element.value.style.overflow = 'hidden'
        }
    }
}

function clear() {
    value.value = ''
}
</script>