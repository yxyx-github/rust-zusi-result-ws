<template>
    <thead :class="props.sticky ? 'sticky z-10 top-0 bg-bg ring-1 ring-secondary-400' : ''">
        <slot/>
    </thead>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
    sticky?: boolean
}>(), {
    sticky: false,
})
</script>