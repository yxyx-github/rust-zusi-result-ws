<template>
    <Column :gap="1">
        <ToggleControl variant="ghost" severity="secondary" v-model:show="show" label="Toggle"/>
        <div v-if="show">Something to toggle...</div>
    </Column>
</template>

<script setup lang="ts">
import ToggleControl from '@/c-lib/components/core/controls/toggle/ToggleControl.vue'
import { ref } from 'vue'
import Column from '@/c-lib/components/core/layout/Column.vue'

const show = ref(false)
</script>