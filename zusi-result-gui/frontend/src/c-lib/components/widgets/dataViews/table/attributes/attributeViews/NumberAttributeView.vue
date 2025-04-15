<template>
    <div class="text-end">
        {{ $n(props.value, props.attribute.type.format ?? 'decimal') }}
    </div>
</template>

<script setup lang="ts">
import type { AttributeViewProps, NumberAttributeType } from '@/c-lib/types/dataView.ts'

const props = defineProps<AttributeViewProps<NumberAttributeType>>()
</script>