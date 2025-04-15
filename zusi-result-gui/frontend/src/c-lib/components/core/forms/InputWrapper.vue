<template>
    <Row class="relative font-inherit" :="props" @keydown.ctrl.delete="emit('clear')">
        <Row :grow="props.grow" class="border-b border-fg focus-within:border-bg mb-(-1px)">
            <slot/>
        </Row>
        <Button
                v-if="props.showClearButton && !props.disabled"
                severity="secondary"
                variant="link"
                :size="props.size"
                :compact="props.compact"
                narrow
                :focusable="false"
                :class="`absolute top-0 right-0`"
                :title="$t('c.clear')"
                prependIcon="x"
                @click="emit('clear')"
        />
    </Row>
</template>

<script setup lang="ts">
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import type { FlexProps } from '@/c-lib/types/flex.ts'
import { defaultFlexProps } from '@/c-lib/types/defaults/flex'

const emit = defineEmits(['clear'])

const props = withDefaults(defineProps<FlexProps & SizeProps & {
    showClearButton?: boolean
    disabled?: boolean
}>(), {
    ...defaultFlexProps,
    grow: true,
    ...defaultSizeProps,
    showClearButton: true,
    disabled: false,
} as any) as any
</script>