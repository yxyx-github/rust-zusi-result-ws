<template>
    <teleport to="#loadingIndicators">
        <ScreenOverlay/>
        <Row class="fixed top-0 right-0 bottom-0 left-0" alignItems="center" justifyItems="center">
            <LoadingIndicator :size="20"/>
        </Row>
    </teleport>
</template>

<script lang="ts" setup>
import ScreenOverlay from '@/c-lib/components/core/containers/ScreenOverlay.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import LoadingIndicator from '@/c-lib/components/core/progress/loading/LoadingIndicator.vue'
</script>