<template>
    <ContentBox :="props" card>
        <slot/>
    </ContentBox>
</template>

<script setup lang="ts">
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import type { FlexProps } from '@/c-lib/types/flex.ts'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultFlexProps } from '@/c-lib/types/defaults/flex'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

const props = withDefaults(defineProps<FlexProps & SizeProps>(), {
    ...defaultFlexProps,
    ...defaultSizeProps,
} as any)
</script>