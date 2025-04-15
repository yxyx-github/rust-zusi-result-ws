<template>
    <Row v-if="show" alignItems="start" justifyItems="between" :class="`rounded-xs shadow-md text-font-50 ${alertClass}`">
        <div class="grow" :class="sizeClass">
            <slot>{{ props.text }}</slot>
        </div>
        <Button v-if="props.closable" :size="props.size" :compact="props.compact" severity="secondary" variant="ghost" invertFont prependIcon="x" :title="$t('c.close')" @click="close"/>
    </Row>
</template>

<script setup lang="ts">
import Row from '@/c-lib/components/core/layout/Row.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import { computed } from 'vue'
import type { Severity } from '@/c-lib/types'
import { useSize } from '@/c-lib/composables/core/layout/size'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

const props = withDefaults(defineProps<SizeProps & {
    severity?: Severity
    text?: string
    closable?: boolean
}>(), {
    ...defaultSizeProps,
    severity: 'primary',
    text: '',
    closable: true,
})

const show = defineModel<boolean>('show', { default: true })

function close() {
    show.value = false
}

const { sizeClass } = useSize(props.size, props.compact)

const alertClass = computed(() => {
    switch (props.severity) {
        case 'success':
            return 'bg-success-500'
        case 'warning':
            return 'bg-warning-500'
        case 'error':
            return 'bg-error-500'
        case 'secondary':
            return 'bg-secondary-400'
        default:
        case 'primary':
            return 'bg-primary-500'
    }
})
</script>