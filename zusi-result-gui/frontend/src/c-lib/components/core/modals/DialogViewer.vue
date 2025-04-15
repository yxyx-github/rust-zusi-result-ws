<template>
    <DialogModal show v-for="dialog in dialogs.dialogs" :key="dialog.id" :config="dialog.config"/>
</template>

<script setup lang="ts">
import { useDialogsStore } from '@/c-lib/stores/dialogs'
import Modal from '@/c-lib/components/core/modals/Modal.vue'
import MenuBar from '@/c-lib/components/core/menus/bar/MenuBar.vue'
import DialogModal from '@/c-lib/components/core/modals/DialogModal.vue'

const dialogs = useDialogsStore()
</script>