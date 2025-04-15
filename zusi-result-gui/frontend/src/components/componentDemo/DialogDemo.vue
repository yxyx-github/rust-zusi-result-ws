<template>
    <ButtonBar>
        <Button @click="openDialog" label="Open Dialog"/>
        <Button @click="openConfirmDialog" label="Open Confirm Dialog"/>
    </ButtonBar>
</template>

<script setup lang="ts">
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import { useDialogsStore } from '@/c-lib/stores/dialogs'

const dialogs = useDialogsStore()

function openDialog() {
    dialogs.dialog({
        title: 'Dialog',
        content: 'Dialog Content',
        actions: [
            { name: 'yes', label: 'Yes', severity: 'success' },
            { name: 'no', label: 'No', severity: 'error' },
        ],
    }).then((actionName: string) => console.log('DialogAction:', actionName))
}

function openConfirmDialog() {
    dialogs.confirm({
        title: 'Confirm',
        content: 'Really want to confirm?',
    }).then((actionName: string) => console.log('DialogAction:', actionName))
}
</script>