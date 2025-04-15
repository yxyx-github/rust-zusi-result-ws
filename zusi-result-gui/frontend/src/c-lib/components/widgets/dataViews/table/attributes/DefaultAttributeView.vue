<template>
    <StringAttributeView v-if="props.attribute.type.name === 'string'" :attribute="props.attribute as Attribute<StringAttributeType>" :value="props.value"/>
    <NumberAttributeView v-else-if="props.attribute.type.name === 'number'" :attribute="props.attribute as Attribute<NumberAttributeType>" :value="props.value"/>
    <DateTimeAttributeView v-else-if="props.attribute.type.name === 'datetime'" :attribute="props.attribute  as Attribute<DateTimeAttributeType>" :value="props.value"/>
    <template v-else>
        {{ props.value }}
    </template>
</template>

<script setup lang="ts">
import type { Attribute, AttributeType, AttributeValue, AttributeViewProps, DateTimeAttributeType, NumberAttributeType, StringAttributeType } from '@/c-lib/types/dataView.ts'
import NumberAttributeView from '@/c-lib/components/widgets/dataViews/table/attributes/attributeViews/NumberAttributeView.vue'
import StringAttributeView from '@/c-lib/components/widgets/dataViews/table/attributes/attributeViews/StringAttributeView.vue'
import { computed } from 'vue'
import DateTimeAttributeView from '@/c-lib/components/widgets/dataViews/table/attributes/attributeViews/DateTimeAttributeView.vue'

const props = defineProps<AttributeViewProps<AttributeType>>()
</script>