<template>
    <InputWrapper :grow="!hasWidth" :size="props.size" :compact="props.compact" :disabled="props.disabled" @clear="clear">
        <input
                :="$attrs"
                :class="`${props.class} ${inputClass} font-inherit ${sizeClass}`"
                v-focus="props.initialFocus"
                :disabled="props.disabled"
                :type="props.type"
                v-model="value"
        />
    </InputWrapper>
</template>

<script setup lang="ts">
import { useSize } from '@/c-lib/composables/core/layout/size'
import { vFocus } from '@/c-lib/directives/v-focus'
import { defaultInputProps, defaultTextInputProps } from '@/c-lib/types/defaults/input'
import type { InputProps, TextInputProps } from '@/c-lib/types/input.ts'
import InputWrapper from '@/c-lib/components/core/forms/InputWrapper.vue'
import { useInputClass } from '@/c-lib/composables/core/inputs/inputClass'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import { useHasWidth } from '@/c-lib/composables/core/layout/hasWidth'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<InputProps & TextInputProps & SizeProps & {
    class?: string
}>(), {
    ...defaultInputProps,
    ...defaultTextInputProps,
    ...defaultSizeProps,
    class: '',
})

const value = defineModel<string>({ required: true })

const { sizeClass } = useSize(props.size, props.compact)

const inputClass = useInputClass(props.severity)

const hasWidth = useHasWidth(props.class)

function clear() {
    value.value = ''
}
</script>