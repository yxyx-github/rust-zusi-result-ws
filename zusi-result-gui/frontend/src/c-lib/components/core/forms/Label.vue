<template>
    <label :class="`inline-block text-font-700 ${fontSizeClass}`">
        <slot>{{ props.text }}</slot>
    </label>
</template>

<script setup lang="ts">
import { useSize } from '@/c-lib/composables/core/layout/size'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

const props = withDefaults(defineProps<SizeProps & {
    text?: string
}>(), {
    ...defaultSizeProps,
    text: '',
})

const { fontSizeClass } = useSize(props.size, props.compact)
</script>