<template>
    <span :class="`inline-block rounded-xs shadow-md ${sizeClass} ${tagClass}`">
        <slot>{{ props.label }}</slot>
    </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSize } from '@/c-lib/composables/core/layout/size'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import type { Severity } from '@/c-lib/types'

const props = withDefaults(defineProps<SizeProps & {
    label?: string
    type?: Severity | 'empty' | 'none'
}>(), {
    ...defaultSizeProps,
    label: '',
    type: 'secondary',
})

const { sizeClass } = useSize(props.size, props.compact)

const tagClass = computed(() => {
    switch (props.type) {
        case 'warning':
            return 'text-white bg-warning-600'
        case 'error':
            return 'text-white bg-error-600'
        case 'success':
            return 'text-white bg-success-600'
        case 'primary':
            return 'text-white bg-primary-600'
        case 'empty':
            return 'text-fg border border-fg'
        case 'none':
            return ''
        default:
        case 'secondary':
            return 'text-bg bg-secondary-500'
    }
})
</script>