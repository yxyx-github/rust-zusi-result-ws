<template>
    <ListItem :size="props.size" :compact="props.compact" isSubListParent>
        <Row :gap="0" alignItems="center">
            <Button severity="secondary" variant="link" :size="props.size" :compact="props.compact" :prependIcon="buttonIcon" @click="toggle"/>
            <ContentBox v-if="$slots.parent" direction="col" :size="props.size" :compact="props.compact">
                <slot name="parent"/>
            </ContentBox>
            <div v-else :class="sizeClass">
                <Label :size="props.size" :compact="props.compact" :text="props.text"/>
            </div>
        </Row>
    </ListItem>
    <List v-if="show" :class="`${indentSizeClass}`">
        <slot/>
    </List>
</template>

<script setup lang="ts">
import List from '@/c-lib/components/core/lists/List.vue'
import ListItem from '@/c-lib/components/core/lists/ListItem.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import { computed } from 'vue'
import Label from '@/c-lib/components/core/forms/Label.vue'
import { useSize } from '@/c-lib/composables/core/layout/size'
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

const props = withDefaults(defineProps<SizeProps & {
    text?: string
}>(), {
    ...defaultSizeProps,
    text: '',
})

const show = defineModel<boolean>('show', { default: false })

const buttonIcon = computed(() => show.value ? 'chevron-down' : 'chevron-right')

const indentSizeClass = computed(() => {
    switch (props.size) {
        case 'sm':
            return `${props.compact ? 'ml-4.5 text-sm' : 'ml-7.5 text-sm'}`
        case 'md':
            return `${props.compact ? 'ml-6 text-base' : 'ml-10 text-base'}`
        case 'lg':
            return `${props.compact ? 'ml-7.5 text-lg' : 'ml-12.5 text-lg'}`
    }
})

const { sizeClass } = useSize(props.size, props.compact)

function toggle() {
    show.value = !show.value
}

</script>