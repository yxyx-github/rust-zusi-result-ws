<template>
    <InputWrapper :grow="!hasWidth" :size="props.size" :compact="props.compact" :disabled="props.disabled" :showClearButton="false" @clear="clear">
        <input
                :="$attrs"
                :class="`${props.class} ${inputClass} font-inherit ${sizeClass}`"
                v-focus="props.initialFocus"
                :disabled="props.disabled"
                :type="props.type === 'datetime' ? 'datetime-local' : props.type"
                v-model="value"
        />
    </InputWrapper>
</template>

<script setup lang="ts">
import { useSize } from '@/c-lib/composables/core/layout/size'
import { vFocus } from '@/c-lib/directives/v-focus'
import { defaultDateTimeInputProps, defaultInputProps } from '@/c-lib/types/defaults/input'
import type { DateTimeInputProps, InputProps } from '@/c-lib/types/input.ts'
import InputWrapper from '@/c-lib/components/core/forms/InputWrapper.vue'
import DateTime from '@/c-lib/helpers/DateTime'
import { useInputClass } from '@/c-lib/composables/core/inputs/inputClass'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import { useHasWidth } from '@/c-lib/composables/core/layout/hasWidth'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<InputProps & DateTimeInputProps & SizeProps & {
    class?: string
}>(), {
    ...defaultInputProps,
    ...defaultDateTimeInputProps,
    ...defaultSizeProps,
    class: '',
})

const value = defineModel<Date | string>({
    required: true,
    get: (value: Date | string) => {
        switch (props.type) {
            case 'date':
                return DateTime.format.dateInputValue(new Date(value))
            case 'time':
                return DateTime.format.timeInputValue(new Date(value))
            case 'datetime':
                return DateTime.format.dateTimeLocaleInputValue(new Date(value))
        }
    },
    set: (value: Date | string) => {
        if (typeof value === 'string') {
            return new Date(value)
        } else {
            return value
        }
    },
})

const { sizeClass } = useSize(props.size, props.compact)

const inputClass = useInputClass(props.severity)

const hasWidth = useHasWidth(props.class)

function clear() {
    value.value = new Date(Date.now())
}
</script>