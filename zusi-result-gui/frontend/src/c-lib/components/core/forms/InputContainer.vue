<template>
    <Column :gap="1" :grow="props.grow" class="font-inherit">
        <slot/>
    </Column>
</template>

<script setup lang="ts">
import Column from '@/c-lib/components/core/layout/Column.vue'

const props = withDefaults(defineProps<{
    grow?: boolean
}>(), {
    grow: false,
})
</script>