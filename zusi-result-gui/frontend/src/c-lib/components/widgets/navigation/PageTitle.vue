<template>
    <span :class="largeClass">
        {{ longTitle }}
    </span>
    <span :class="smallClass">
        {{ shortTitle }}
    </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Threshold } from '@/c-lib/types/screens.ts'
import { useThresholdClass } from '@/c-lib/composables/core/layout/thresholdClass'

const props = withDefaults(defineProps<{
    appName: string
    pageName?: string
    threshold?: Threshold
}>(), {
    pageName: '',
    threshold: 'sm'
})

const longTitle = computed(() => props.pageName === '' ? '' : `${props.pageName} - ${props.appName}`)
const shortTitle = computed(() => props.pageName === '' ? props.appName : props.pageName)

const { smallClass, largeClass } = useThresholdClass(props.threshold, 'hidden', 'inline-block')
</script>