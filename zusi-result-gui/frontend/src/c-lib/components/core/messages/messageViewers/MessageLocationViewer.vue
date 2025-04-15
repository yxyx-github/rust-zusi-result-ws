<template>
    <Column :gap="1" v-if="locationMessages.length > 0">
        <LocatedMessageViewer
                v-for="message in locationMessages"
                :key="message.id"
                :message="message"
                :size="props.size"
                :compact="props.compact"
        />
    </Column>
</template>

<script setup lang="ts">
import type { LocatedMessage, MessageLocation } from '@/c-lib/types/message.ts'
import { useMessagesStore } from '@/c-lib/stores/messages'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'
import { computed, onUnmounted } from 'vue'
import Alert from '@/c-lib/components/core/messages/Alert.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import LocatedMessageViewer from '@/c-lib/components/core/messages/messageViewers/LocatedMessageViewer.vue'

const messages = useMessagesStore()

const props = withDefaults(defineProps<SizeProps & {
    location?: MessageLocation
}>(), defaultSizeProps)

const locationMessages = computed(() => messages.messages(props.location))

onUnmounted(() => messages.remove('unmount', props.location))
</script>