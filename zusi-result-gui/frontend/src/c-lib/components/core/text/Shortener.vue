<template>
    <div v-if="isShortened" class="float-left">
        <ToggleControl variant="link" severity="secondary" v-model:show="show"/>
    </div>
    <span v-if="show || !isShortened" :title="text">{{ props.text }}</span>
    <span v-else :title="text">{{ shortenedText }} <Button @click="show = true" variant="link" severity="secondary" label="..."/></span>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import ToggleControl from '@/c-lib/components/core/controls/toggle/ToggleControl.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import Form from '@/c-lib/components/core/forms/Form.vue'

const props = withDefaults(defineProps<{
    text: string
    length?: number
}>(), {
    length: 10,
})

const show = ref(false)

const isShortened = computed(() => shortenedText.value !== null)

const shortenedText = computed(() => {
    if (props.text.length > props.length) {
        return props.text.substring(0, props.length)
    } else {
        return null
    }
})
</script>