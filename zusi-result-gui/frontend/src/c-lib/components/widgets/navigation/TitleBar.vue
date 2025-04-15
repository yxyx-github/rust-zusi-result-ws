<template>
    <ContentBox direction="row" alignItems="center" compact card class="bg-secondary-400 border-secondary-400">
        <ButtonBar v-if="$slots.left" :grow="false" alignItems="center">
            <slot name="left"/>
        </ButtonBar>
        <div v-if="$slots.default" class="grow text-center whitespace-nowrap overflow-x-auto text-font-50 font-bold">
            <slot/>
        </div>
        <ButtonBar v-if="$slots.right" :grow="false" alignItems="center">
            <slot name="right"/>
        </ButtonBar>
    </ContentBox>
</template>

<script setup lang="ts">
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
</script>