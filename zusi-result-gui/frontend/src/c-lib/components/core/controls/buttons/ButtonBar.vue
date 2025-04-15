<template>
    <Flex :direction="props.direction" :wrap="props.wrap" :gap="1" alignItems="stretch">
        <slot/>
    </Flex>
</template>

<script lang="ts" setup>
import type { Direction } from '@/c-lib/types/flex.ts'
import Flex from '@/c-lib/components/core/layout/Flex.vue'

const props = withDefaults(defineProps<{
    direction?: Direction
    wrap?: boolean
}>(), {
    direction: 'row',
    wrap: true,
})
</script>
