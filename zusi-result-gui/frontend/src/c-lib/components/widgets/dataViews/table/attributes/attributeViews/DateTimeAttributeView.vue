<template>
    {{ $d(props.value, props.attribute.type.format ?? 'datetime') }}
</template>

<script setup lang="ts">
import type { AttributeViewProps, DateTimeAttributeType } from '@/c-lib/types/dataView.ts'

const props = defineProps<AttributeViewProps<DateTimeAttributeType>>()
</script>