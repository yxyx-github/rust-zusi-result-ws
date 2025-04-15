<template>
    <Flex :="props" :class="`bg-bg ${equalAspectRatioSizeClass} ${cardClass}`">
        <slot/>
    </Flex>
</template>

<script setup lang="ts">
import { useSize } from '@/c-lib/composables/core/layout/size'
import type { FlexProps } from '@/c-lib/types/flex.ts'
import { defaultFlexProps } from '@/c-lib/types/defaults/flex'
import Flex from '@/c-lib/components/core/layout/Flex.vue'
import { useCardClass } from '@/c-lib/composables/core/containers/cardClass'
import type { ContainerProps } from '@/c-lib/types/container.ts'
import { defaultContainerProps } from '@/c-lib/types/defaults/container'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

const props = withDefaults(defineProps<FlexProps & ContainerProps & SizeProps>(), {
    ...defaultFlexProps,
    ...defaultContainerProps,
    ...defaultSizeProps,
} as any) as any

const { equalAspectRatioSizeClass } = useSize(props.size, props.compact as boolean)
const cardClass = useCardClass(props.card as boolean)
</script>