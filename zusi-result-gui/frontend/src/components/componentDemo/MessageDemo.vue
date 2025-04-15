<template>
    <MessageLocationViewer location="messageDemo"/>
    <Form @submit.prevent="addMessage">
        <InputContainer>
            <Label text="Location:"/>
            <Select v-model="locationValue" :items="locationItems"/>
        </InputContainer>
        <InputContainer>
            <Label text="Severity:"/>
            <Select v-model="severityValue" :items="severityItems"/>
        </InputContainer>
        <InputContainer>
            <Label text="Allow override:"/>
            <ButtonBar>
                <Checkbox v-model="allowOverride" :default="false"/>
            </ButtonBar>
        </InputContainer>
        <InputContainer>
            <Label text="Remove after:"/>
            <ButtonBar alignItems="center">
                <Checkbox v-model="enableRemoveAfter" :default="false"/>
                <NumberInput class="w-24" v-model="removeAfter" :disabled="!enableRemoveAfter"/>
                <Label text="ms"/>
            </ButtonBar>
        </InputContainer>
        <InputContainer>
            <Label text="Message text:"/>
            <TextArea autoResize v-model="text"/>
        </InputContainer>
        <ButtonBar>
            <Button label="Add" type="submit"/>
            <Button label="Set" severity="secondary" type="button" @click="setMessage"/>
        </ButtonBar>
    </Form>
    <Column :gap="1">
        <Alert v-model:show="show">TestAlert with some very very very very very very very very very very very very very very very very very very very very very very very very long text</Alert>
        <Button v-if="!show" label="Reopen" @click="show = true"/>
        <Alert size="sm" compact>compact TestAlert with some text</Alert>
        <Alert size="md" compact>compact TestAlert with some text</Alert>
        <Alert size="lg" compact>compact TestAlert with some text</Alert>
        <Alert :closable="false">TestAlert with some text</Alert>
        <Alert text="TestAlert with some text by prop"/>
        <Alert severity="secondary">TestAlert with some text</Alert>
        <Alert severity="success">TestAlert with some text</Alert>
        <Alert severity="warning">TestAlert with some text</Alert>
        <Alert severity="error">TestAlert with some text</Alert>
    </Column>
</template>

<script setup lang="ts">
import Alert from '@/c-lib/components/core/messages/Alert.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import { computed, ref } from 'vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import MessageLocationViewer from '@/c-lib/components/core/messages/messageViewers/MessageLocationViewer.vue'
import Form from '@/c-lib/components/core/forms/Form.vue'
import InputContainer from '@/c-lib/components/core/forms/InputContainer.vue'
import Label from '@/c-lib/components/core/forms/Label.vue'
import TextArea from '@/c-lib/components/core/forms/inputs/text/TextArea.vue'
import Select from '@/c-lib/components/core/forms/inputs/select/Select.vue'
import type { SelectItem } from '@/c-lib/types/input.ts'
import { useMessagesStore } from '@/c-lib/stores/messages'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import type { Severity } from '@/c-lib/types'
import Checkbox from '@/c-lib/components/core/forms/inputs/boolean/Checkbox.vue'
import MessageViewer from '@/c-lib/components/core/messages/messageViewers/MessageViewer.vue'
import NumberInput from '@/c-lib/components/core/forms/inputs/numbers/NumberInput.vue'

const messages = useMessagesStore()

const allMessages = computed(() => messages.messages())

const locationItems: SelectItem[] = [
    { label: '_', value: '' },
    { label: 'MessageDemo', value: 'messageDemo' },
]
const locationValue = ref<SelectItem>(locationItems[0] ?? null)

const severityItems: SelectItem[] = [
    { label: 'Primary', value: 'primary' },
    { label: 'Secondary', value: 'secondary' },
    { label: 'Success', value: 'success' },
    { label: 'Warning', value: 'warning' },
    { label: 'Error', value: 'error' },
]
const severityValue = ref<SelectItem>(severityItems[0] ?? null)

const allowOverride = ref<boolean>(false)

const enableRemoveAfter = ref<boolean>(false)
const removeAfter = ref<number>(3000)

const text = ref<string>('')

function addMessage() {
    messages.add({
        severity: severityValue.value?.value as Severity,
        text: text.value
    }, {
        location: locationValue.value.value,
        allowOverride: allowOverride.value,
        removeOn: severityValue.value?.value as Severity === 'error' ? ['unmount'] : ['route', 'unmount'],
        removeAfter: enableRemoveAfter.value ? removeAfter.value : -1,
    })
}

function setMessage() {
    messages.set([{
        severity: severityValue.value?.value as Severity,
        text: text.value
    }], {
        location: locationValue.value.value,
        allowOverride: allowOverride.value,
        removeOn: severityValue.value?.value as Severity === 'error' ? ['unmount'] : ['route', 'unmount'],
        removeAfter: enableRemoveAfter.value ? removeAfter.value : -1,
    })
}

const show = ref(true)
</script>