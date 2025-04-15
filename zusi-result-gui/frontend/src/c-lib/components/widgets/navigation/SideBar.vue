<template>
    <ContentBox direction="col" alignItems="stretch" compact card class="bg-secondary-400 border-secondary-400">
        <ButtonBar direction="col" v-if="$slots.top" :grow="false" alignItems="center">
            <slot name="top"/>
        </ButtonBar>
        <ButtonBar direction="col" v-if="$slots.bottom" :grow="false" alignItems="center">
            <slot name="bottom"/>
        </ButtonBar>
    </ContentBox>
</template>

<script setup lang="ts">
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
</script>