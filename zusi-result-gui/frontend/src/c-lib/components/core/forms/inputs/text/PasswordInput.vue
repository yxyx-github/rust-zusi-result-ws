<template>
    <TextInput :="{ ...$attrs, ...props }" type="password" v-model="value"/>
</template>

<script setup lang="ts">
import TextInput from '@/c-lib/components/core/forms/inputs/text/TextInput.vue'
import type { InputProps, TextInputProps } from '@/c-lib/types/input.ts'
import { defaultInputProps, defaultTextInputProps } from '@/c-lib/types/defaults/input'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<InputProps & {
    type?: 'password'
} & Omit<TextInputProps, 'type'> & SizeProps>(), {
    ...defaultInputProps,
    ...defaultTextInputProps,
    ...defaultSizeProps,
    type: 'password',
})

const value = defineModel<string>({ required: true })
</script>