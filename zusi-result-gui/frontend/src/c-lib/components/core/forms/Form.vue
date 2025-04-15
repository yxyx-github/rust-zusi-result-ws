<template>
    <Column :is="h('form')" :gap="2" class="w-full">
        <slot/>
    </Column>
</template>

<script setup lang="ts">
import Column from '@/c-lib/components/core/layout/Column.vue'
import { h } from 'vue'
</script>