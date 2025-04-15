<template>
    <ButtonBar v-for="size in ['sm', 'md', 'lg'] as Size[]" :key="size">
        <Tag :size="size" type="primary" label="Tag"/>
        <Tag :size="size" type="secondary" label="Tag"/>
        <Tag :size="size" type="success" label="Tag"/>
        <Tag :size="size" type="error" label="Tag"/>
        <Tag :size="size" type="warning" label="Tag"/>
        <Tag :size="size" type="empty" label="Tag"/>
        <Tag :size="size" type="none" label="Tag"/>
    </ButtonBar>
    <ButtonBar v-for="size in ['sm', 'md', 'lg'] as Size[]" :key="size">
        <Tag :size="size" compact type="primary" label="Tag"/>
        <Tag :size="size" compact type="secondary" label="Tag"/>
        <Tag :size="size" compact type="success" label="Tag"/>
        <Tag :size="size" compact type="error" label="Tag"/>
        <Tag :size="size" compact type="warning" label="Tag"/>
        <Tag :size="size" compact type="empty" label="Tag"/>
        <Tag :size="size" compact type="none" label="Tag"/>
    </ButtonBar>
</template>

<script setup lang="ts">
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import Tag from '@/c-lib/components/core/signs/Tag.vue'
import type { Size } from '@/c-lib/types'
</script>