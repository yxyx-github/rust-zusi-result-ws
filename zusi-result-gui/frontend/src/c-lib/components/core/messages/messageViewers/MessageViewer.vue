<template>
    <Column :gap="1">
        <Alert
                v-for="(message, index) in props.messages"
                :key="index"
                :severity="message.severity"
                :text="message.text"
                :size="props.size"
                :compact="props.compact"
        />
    </Column>
</template>

<script setup lang="ts">
import Alert from '@/c-lib/components/core/messages/Alert.vue'
import type { Message } from '@/c-lib/types/message.ts'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import Column from '@/c-lib/components/core/layout/Column.vue'

const props = withDefaults(defineProps<SizeProps & {
    messages?: Message[]
}>(), {
    ...defaultSizeProps,
    messages: () => [],
})
</script>