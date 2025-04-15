<template>
    <li class="border border-bg p-0" :class="{ 'tr-hover': props.highlightOnHover, 'tr-focus': props.highlightOnFocus }">
        <template v-if="$slots.default">
            <Column v-if="props.isSubListParent">
                <slot/>
            </Column>
            <ContentBox v-else direction="col" :size="props.size" :compact="props.compact">
                <slot/>
            </ContentBox>
        </template>
        <div v-else :class="sizeClass">
            <Label :size="props.size" :compact="props.compact" :text="props.text"/>
        </div>
    </li>
</template>

<script setup lang="ts">
import Label from '@/c-lib/components/core/forms/Label.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import { useSize } from '@/c-lib/composables/core/layout/size'
import type { HighlightProps } from '@/c-lib/types/highlight.ts'
import { defaultHighlightProps } from '@/c-lib/types/defaults/highlight'
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

const props = withDefaults(defineProps<HighlightProps & SizeProps & {
    text?: string
    isSubListParent?: boolean
}>(), {
    ...defaultHighlightProps,
    ...defaultSizeProps,
    text: '',
    isSubListParent: false,
})

const { sizeClass } = useSize(props.size, props.compact as boolean)
</script>

<style scoped>
@reference "../../../assets/tailwind/index.css";

.tr-hover {
    @apply hover:border-secondary-200
}

body .tr-focus {
    @apply focus-within:border-secondary-300
}
</style>