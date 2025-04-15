<template>
    <span class="font-inherit" v-resize="resized" :class="showClass" ref="container"><slot/></span>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, watch } from 'vue'
import { vResize } from '@/c-lib/directives/v-resize'
import type { Threshold } from '@/c-lib/types/screens.ts'
import { useThresholdClass } from '@/c-lib/composables/core/layout/thresholdClass'

const emit = defineEmits(['show'])

const props = withDefaults(defineProps<{
    show?: boolean | null
    threshold?: Threshold
}>(), {
    show: null,
    threshold: 'sm'
})

const { largeClass } = useThresholdClass(props.threshold, 'hidden', 'block')

const showClass = computed(() => props.show === null ? largeClass.value : (props.show ? 'block' : 'hidden'))

const container = ref(null)

function resized() {
    const shown = props.show === null ? (container.value === null ? null : (window.getComputedStyle(container.value).display === 'block')) : props.show
    emit('show', shown)
}

onMounted(resized)
watch(() => props.show, () =>
    resized()
)
</script>