<template>
    <component :is="props.is" class="flex" :class="containerClass">
        <slot/>
    </component>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import type { FlexProps } from '@/c-lib/types/flex.ts'
import { defaultFlexProps } from '@/c-lib/types/defaults/flex'
import type { Threshold } from '@/c-lib/types/screens.ts'

const props = withDefaults(defineProps<FlexProps>(), defaultFlexProps as any) as any

const containerClass = computed(() => `
        ${buildClass(props.direction, value => `flex-${value}`)}
        ${buildClass(props.alignItems, value => `items-${value}`)}
        ${buildClass(props.justifyItems, value => `justify-${value}`)}
        ${buildClass(props.gap, value => `gap-${value}`)}
        ${buildClass(props.wrap, value => `${value ? 'flex-wrap' : 'flex-nowrap'}`)}
        ${buildClass(props.grow, value => `${value ? 'grow' : 'grow-0'}`)}
        ${buildClass(props.shrink, value => `${value ? 'shrink' : 'shrink-0'}`)}
    `)

function buildClass(data: { [key in Threshold]: any } | any, buildSingle: (value: any) => string) {
    if (typeof data === 'object') {
        let className = ''
        for (const [key, value] of Object.entries(data)) {
            className += ` ${key}:${buildSingle(value)}`
        }
        return className.trim()
    } else {
        return buildSingle(data)
    }
}
</script>
