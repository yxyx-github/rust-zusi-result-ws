<template>
    <div id="modals" ref="modals" class="relative z-10 h-0 w-full"></div>
    <div id="loadingIndicators" class="relative z-20 w-full"></div>
    <div id="menus" class="relative z-30 w-full"></div>
    <LoadingIndicatorScreen v-if="useLoadingIndicators.has()"/>
    <DialogViewer/>
    <Column class="relative z-0 w-screen h-screen" :="$attrs" :inert="inert">
        <slot/>
    </Column>
</template>

<script setup lang="ts">
import { useLoadingIncicatorsStore } from '@/c-lib/stores/loadingIndicators'
import LoadingIndicatorScreen from '@/c-lib/components/core/progress/loading/LoadingIndicatorScreen.vue'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import DialogViewer from '@/c-lib/components/core/modals/DialogViewer.vue'

defineOptions({
    inheritAttrs: false,
})

const useLoadingIndicators = useLoadingIncicatorsStore()

const modals = ref<HTMLElement | null>(null)

const modalObserver = ref<MutationObserver | null>(null)

const inert = ref(false)

function initObserver() {
    const observer = new MutationObserver(() => {
        inert.value = (modals.value?.children.length ?? 0) > 0
    })
    observer.observe(modals.value as HTMLElement, {
        subtree: false,
        childList: true,
    })
    modalObserver.value = observer
}

onMounted(() => initObserver())

onBeforeUnmount(() => modalObserver.value?.disconnect())
</script>