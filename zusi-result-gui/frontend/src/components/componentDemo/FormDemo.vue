<template>
    <Form @submit.prevent="submit">
        <InputContainer>
            <Label text="TextInput:" size="sm"/>
            <MessageViewer :messages="messages" size="sm" compact/>
            <TextInput v-model="stringValue" :disabled="disabled" placeholder="Placeholder" size="sm" compact/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:" size="md"/>
            <MessageViewer :messages="messages" size="md" compact/>
            <TextInput v-model="stringValue" :disabled="disabled" placeholder="Placeholder" size="md" compact/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:" size="lg"/>
            <MessageViewer :messages="messages" size="lg" compact/>
            <TextInput v-model="stringValue" :disabled="disabled" placeholder="Placeholder" size="lg" compact/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:" size="sm"/>
            <MessageViewer :messages="messages" size="sm"/>
            <TextInput v-model="stringValue" :disabled="disabled" placeholder="Placeholder" size="sm"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:" size="md"/>
            <MessageViewer :messages="messages" size="md"/>
            <TextInput v-model="stringValue" :disabled="disabled" placeholder="Placeholder" size="md"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:" size="lg"/>
            <MessageViewer :messages="messages" size="lg"/>
            <TextInput v-model="stringValue" :disabled="disabled" placeholder="Placeholder" size="lg"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:"/>
            <MessageViewer :messages="messages"/>
            <TextInput severity="primary" v-model="stringValue" :disabled="disabled" placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:"/>
            <MessageViewer :messages="messages"/>
            <TextInput severity="secondary" v-model="stringValue" :disabled="disabled" placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:"/>
            <MessageViewer :messages="messages"/>
            <TextInput severity="success" v-model="stringValue" :disabled="disabled" placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:"/>
            <MessageViewer :messages="messages"/>
            <TextInput severity="warning" v-model="stringValue" :disabled="disabled" placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextInput:"/>
            <MessageViewer :messages="messages"/>
            <TextInput severity="error" v-model="stringValue" :disabled="disabled" placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label text="PasswordInput:"/>
            <MessageViewer :messages="messages"/>
            <PasswordInput v-model="stringValue" :disabled="disabled" placeholder="Password-Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label text="TextArea:"/>
            <MessageViewer :messages="messages"/>
            <TextArea v-model="stringValue" :disabled="disabled" autoResize placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label :text="`NumberInput: ${numberValue}`"/>
            <MessageViewer :messages="messages"/>
            <NumberInput class="w-16" v-model="numberValue" :disabled="disabled" placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label :text="`DateTimeInput: ${dateTimeValue}`"/>
            <MessageViewer :messages="messages"/>
            <DateTimeInput v-model="dateTimeValue" :disabled="disabled" placeholder="Placeholder"/>
        </InputContainer>
        <InputContainer>
            <Label :text="`Select: ${selectValue?.value ?? 'null'}`"/>
            <MessageViewer :messages="messages"/>
            <ButtonBar>
                <Select v-model="selectValue" :items="selectItems" :default="null" :disabled="disabled"/>
                <Button severity="secondary" label="Without First" @click="updateSelectItems('first')"/>
                <Button severity="secondary" label="Without Second" @click="updateSelectItems('second')"/>
                <Button severity="secondary" label="Without Third" @click="updateSelectItems('third')"/>
                <Button severity="secondary" label="All" @click="updateSelectItems('')"/>
            </ButtonBar>
        </InputContainer>
        <InputContainer>
            <Label text="Checkbox: Messages:"/>
            <MessageViewer :messages="messages"/>
            <ButtonBar>
                <Checkbox size="sm" v-model="booleanValue" :default="null" :disabled="disabled"/>
                <Button size="sm" label="Null" @click="booleanValue = null"/>
            </ButtonBar>
            <ButtonBar>
                <Checkbox size="md" v-model="booleanValue" :default="null" :disabled="disabled"/>
                <Button size="md" label="Null" @click="booleanValue = null"/>
            </ButtonBar>
            <ButtonBar>
                <Checkbox size="lg" v-model="booleanValue" :default="null" :disabled="disabled"/>
                <Button size="lg" label="Null" @click="booleanValue = null"/>
            </ButtonBar>
            <ButtonBar>
                <Checkbox size="sm" compact v-model="booleanValue" :default="null" :disabled="disabled"/>
                <Button size="sm" compact label="Null" @click="booleanValue = null"/>
            </ButtonBar>
            <ButtonBar>
                <Checkbox size="md" compact v-model="booleanValue" :default="null" :disabled="disabled"/>
                <Button size="md" compact label="Null" @click="booleanValue = null"/>
            </ButtonBar>
            <ButtonBar>
                <Checkbox size="lg" compact v-model="booleanValue" :default="null" :disabled="disabled"/>
                <Button size="lg" compact label="Null" @click="booleanValue = null"/>
            </ButtonBar>
        </InputContainer>
        <InputContainer>
            <Label text="Checkbox: Disabled:"/>
            <MessageViewer :messages="messages"/>
            <ButtonBar>
                <Checkbox falseIcon="x" v-model="disabled" :default="false" severity="primary"/>
                <Checkbox falseIcon="x" v-model="disabled" :default="false" severity="secondary"/>
                <Checkbox falseIcon="x" v-model="disabled" :default="false" severity="success"/>
                <Checkbox falseIcon="x" v-model="disabled" :default="false" severity="warning"/>
                <Checkbox falseIcon="x" v-model="disabled" :default="false" severity="error"/>
            </ButtonBar>
        </InputContainer>
        <ButtonBar>
            <Button type="submit" label="Submit"/>
        </ButtonBar>
    </Form>
</template>

<script setup lang="ts">
import Form from '@/c-lib/components/core/forms/Form.vue'
import Label from '@/c-lib/components/core/forms/Label.vue'
import TextInput from '@/c-lib/components/core/forms/inputs/text/TextInput.vue'
import { computed, ref } from 'vue'
import type { Message } from '@/c-lib/types/message.ts'
import InputContainer from '@/c-lib/components/core/forms/InputContainer.vue'
import MessageViewer from '@/c-lib/components/core/messages/messageViewers/MessageViewer.vue'
import PasswordInput from '@/c-lib/components/core/forms/inputs/text/PasswordInput.vue'
import TextArea from '@/c-lib/components/core/forms/inputs/text/TextArea.vue'
import Checkbox from '@/c-lib/components/core/forms/inputs/boolean/Checkbox.vue'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import NumberInput from '@/c-lib/components/core/forms/inputs/numbers/NumberInput.vue'
import DateTimeInput from '@/c-lib/components/core/forms/inputs/datetime/DateTimeInput.vue'
import type { SelectItem } from '@/c-lib/types/input.ts'
import Select from '@/c-lib/components/core/forms/inputs/select/Select.vue'

const stringValue = ref('')

const numberValue = ref(0)

const dateTimeValue = ref(new Date(Date.now()))

const selectItems = ref<SelectItem[]>([
    { label: 'First', value: 'first' },
    { label: 'Second', value: 'second' },
    { label: 'Third', value: 'third' },
])
const defaultSelectItems: SelectItem[] = [
    { label: 'First', value: 'first' },
    { label: 'Second', value: 'second' },
    { label: 'Third', value: 'third' },
]
function updateSelectItems(without: string) {
    selectItems.value = defaultSelectItems.filter(item => item.value !== without)
}

const selectValue = ref<SelectItem | null>(selectItems.value[0] ?? null)

const booleanValue = ref<boolean | null>(null)

const disabled = ref(false)

const messages = computed<Message[]>(() => booleanValue.value ? [
    { severity: 'error', text: 'Error' },
    { severity: 'warning', text: 'Warning' },
] : [])

function submit() {
    console.log('submit')
}
</script>