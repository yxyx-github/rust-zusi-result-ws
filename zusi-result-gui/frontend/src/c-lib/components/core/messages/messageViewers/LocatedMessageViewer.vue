<template>
    <Alert
            class="relative"
            :severity="props.message.message.severity"
            :size="props.size"
            :compact="props.compact"
            @update:show="handleUpdateShow($event)"
            @mouseenter="isHovered = true"
            @mouseleave="isHovered = false"
    >
        <span>
            {{ props.message.message.text }}
            <span class="float-right ml-1 -mr-4" v-if="remainingSeconds !== null">
                <span class="text-xs opacity-70">
                    ({{ remainingSeconds }} s)
                </span>
            </span>
        </span>
        <div v-if="progress !== null" class="absolute right-0 bottom-0 left-0 h-0">
            <div class="border-b-2 border-fg opacity-30 h-0 -mt-(2px)" :style="`width: ${progress ?? 0}%;`"></div>
        </div>
    </Alert>
</template>

<script setup lang="ts">
import Alert from '@/c-lib/components/core/messages/Alert.vue'
import type { LocatedMessage } from '@/c-lib/types/message.ts'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import { computed, onMounted, onUnmounted, ref } from 'vue'

const props = withDefaults(defineProps<SizeProps & {
    message: LocatedMessage
}>(), defaultSizeProps)

function handleUpdateShow(value: boolean) {
    if (!value) {
        props.message.remove()
    }
}

const isHovered = ref<boolean>(false)

const animationFrame = ref<any | null>(null) // TODO: insert correct type
const initialTimestamp = ref<number | null>(null)
const currentTimestamp = ref<number | null>(null)
const hoverStartTimestamp = ref<number | null>(null)

const currentOrHoverStartTimestamp = computed<number | null>(() => hoverStartTimestamp.value === null ? currentTimestamp.value : hoverStartTimestamp.value)
const progress = computed<number | null>(() =>
    initialTimestamp.value === null || currentOrHoverStartTimestamp.value === null
        ? null
        : (100 / props.message.removeAfter * (initialTimestamp.value + props.message.removeAfter - currentOrHoverStartTimestamp.value))
) // in %
const remainingSeconds = computed<number | null>(() =>
    initialTimestamp.value === null || currentOrHoverStartTimestamp.value === null
        ? null
        : Math.ceil((initialTimestamp.value + props.message.removeAfter - currentOrHoverStartTimestamp.value) / 1000)
)

onMounted(() => {
    if (props.message.removeAfter !== -1) {
        animationFrame.value = requestAnimationFrame(handleAnimationFrame)
    }
})

// TODO: insert correct type
function handleAnimationFrame(timestamp: number) {
    currentTimestamp.value = timestamp
    if (isHovered.value && hoverStartTimestamp.value === null) {
        hoverStartTimestamp.value = timestamp
    } else if (!isHovered.value && hoverStartTimestamp.value !== null) {
        if (initialTimestamp.value === null) {
            initialTimestamp.value = 0
        }
        initialTimestamp.value += timestamp - hoverStartTimestamp.value
        hoverStartTimestamp.value = null
    }

    if (initialTimestamp.value === null) {
        initialTimestamp.value = timestamp
    } else if (!isHovered.value && initialTimestamp.value + props.message.removeAfter - timestamp < 0) {
        currentTimestamp.value = null
        props.message.remove()
    }
    animationFrame.value = requestAnimationFrame(handleAnimationFrame)
}

onUnmounted(() => cancelAnimationFrame(animationFrame.value))
</script>