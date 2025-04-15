<template>
    <Modal title="Test Modal" severity="secondary" label="Open Test Modal" trigger>
        Modal
    </Modal>
    <ConfirmModal label="Delete" trigger @confirm="confirm" @abort="abort">
        Do you really want to do something critical?
    </ConfirmModal>
</template>

<script setup lang="ts">
import Modal from '@/c-lib/components/core/modals/Modal.vue'
import ConfirmModal from '@/c-lib/components/core/modals/ConfirmModal.vue'

function confirm() {
    console.log('confirm')
}

function abort() {
    console.log('abort')
}
</script>