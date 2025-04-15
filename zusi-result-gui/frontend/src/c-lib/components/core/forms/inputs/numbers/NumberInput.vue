<template>
    <InputWrapper :grow="!hasWidth" :size="props.size" :compact="props.compact" :disabled="props.disabled" :showClearButton="false" @clear="clear">
        <input
                :="$attrs"
                :class="`${props.class} ${inputClass} text-right font-inherit ${sizeClass}`"
                v-focus="props.initialFocus"
                :disabled="props.disabled"
                type="number"
                v-model="value"
        />
    </InputWrapper>
</template>

<script setup lang="ts">
import { useSize } from '@/c-lib/composables/core/layout/size'
import { vFocus } from '@/c-lib/directives/v-focus'
import { defaultInputProps, defaultNumberInputProps } from '@/c-lib/types/defaults/input'
import type { InputProps, NumberInputProps } from '@/c-lib/types/input.ts'
import InputWrapper from '@/c-lib/components/core/forms/InputWrapper.vue'
import { useInputClass } from '@/c-lib/composables/core/inputs/inputClass'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import { computed } from 'vue'
import { useHasWidth } from '@/c-lib/composables/core/layout/hasWidth'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<InputProps & NumberInputProps & SizeProps & {
    class?: string
}>(), {
    ...defaultInputProps,
    ...defaultNumberInputProps,
    ...defaultSizeProps,
    class: '',
})

const value = defineModel<number | null>({
    required: true,
    set: (value: number | string | null) =>
        typeof value === 'string' ? null : value,
})

const { sizeClass } = useSize(props.size, props.compact)

const inputClass = useInputClass(props.severity)

const hasWidth = useHasWidth(props.class)

function clear() {
    value.value = null
}
</script>