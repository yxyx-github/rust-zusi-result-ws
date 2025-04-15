<template>
    <tr :class="{ 'tr-hover': props.highlightOnHover, 'tr-focus': props.highlightOnFocus }">
        <slot/>
    </tr>
</template>

<script setup lang="ts">
import type { HighlightProps } from '@/c-lib/types/highlight.ts'
import { defaultHighlightProps } from '@/c-lib/types/defaults/highlight'

const props = withDefaults(defineProps<HighlightProps>(), defaultHighlightProps as any)
</script>

<style scoped>
@reference "../../../assets/tailwind/index.css";

.tr-hover {
    @apply hover:bg-primary-50
}
body .tr-focus {
    @apply focus-within:bg-primary-100
}
</style>