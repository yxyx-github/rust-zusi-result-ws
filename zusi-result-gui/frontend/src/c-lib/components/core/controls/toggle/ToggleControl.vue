<template>
    <Row :gap="gapSize">
        <Button :="{ ...$attrs, ...props, label: '' }" @click="toggle" :prependIcon="show ? 'chevron-up' : 'chevron-down'" :size="props.size" :compact="props.compact"/>
        <Label v-if="$slots.default || props.label !== ''" class="self-center" :size="props.size" :compact="props.compact">
            <slot>
                {{ props.label }}
            </slot>
        </Label>
    </Row>
</template>

<script lang="ts" setup>
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import Label from '@/c-lib/components/core/forms/Label.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import type { ButtonProps } from '@/c-lib/types/button.ts'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'
import { useSize } from '@/c-lib/composables/core/layout/size'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

defineOptions({
    inheritAttrs: false,
})

const props = withDefaults(defineProps<ButtonProps & SizeProps>(), {
    ...defaultButtonProps,
    ...defaultSizeProps,
})

const show = defineModel<boolean>('show', { default: false })

const { gapSize } = useSize(props.size, props.compact as boolean)

function toggle() {
    show.value = !show.value
}
</script>