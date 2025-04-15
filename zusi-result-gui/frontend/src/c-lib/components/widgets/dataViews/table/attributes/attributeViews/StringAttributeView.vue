<template>
    <div v-if="props.attribute.type.limit">
        <Shortener :text="props.value" :length="props.attribute.type.limit"/>
    </div>
    <template v-else>
        {{ props.value }}
    </template>
</template>

<script setup lang="ts">
import type { AttributeViewProps, StringAttributeType } from '@/c-lib/types/dataView.ts'
import Shortener from '@/c-lib/components/core/text/Shortener.vue'

const props = defineProps<AttributeViewProps<StringAttributeType>>()
</script>