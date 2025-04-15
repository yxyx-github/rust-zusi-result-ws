<template>
    [{{ props.value }}]
</template>

<script setup lang="ts">
import type { AttributeType, AttributeViewProps } from '@/c-lib/types/dataView.ts'

const props = defineProps<AttributeViewProps<AttributeType>>()
</script>