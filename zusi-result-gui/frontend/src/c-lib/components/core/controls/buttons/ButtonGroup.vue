<template>
    <div class="flex flex-row gap-0 items-stretch justify-start button-group" ref="buttonGroupWrapper">
        <Row v-if="enableShadow" :gap="0" alignItems="stretch" class="button-group shadow-md">
            <slot/>
        </Row>
        <slot v-else/>
    </div>
</template>

<script lang="ts" setup>
import Row from '@/c-lib/components/core/layout/Row.vue'
import { computed, ref } from 'vue'

const buttonGroupWrapper = ref(null)

// @ts-ignore
const enableShadow = computed(() => buttonGroupWrapper.value?.innerHTML.includes('shadow-md'))
</script>

<style scoped>
.button-group > :slotted(*) {
    z-index: 0;
    @apply shadow-none
}

.button-group > :slotted(*:focus-within) {
    z-index: 1;
}

.button-group > :slotted(* ~ *) {
    border-top-left-radius: 0 !important;
    border-bottom-left-radius: 0 !important;
}

.button-group > :slotted(:not(:last-child)) {
    border-top-right-radius: 0 !important;
    border-bottom-right-radius: 0 !important;
}
</style>
