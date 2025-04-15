<template>
    <Flex direction="col" :="props">
        <slot/>
    </Flex>
</template>

<script lang="ts" setup>
import Flex from '@/c-lib/components/core/layout/Flex.vue'
import type { FlexProps } from '@/c-lib/types/flex.ts'

const props = defineProps<Omit<FlexProps, 'direction'>>()
</script>