<template>
    <Form @submit.prevent>
        <InputContainer>
            <Label text="Text to shorten:"/>
            <TextArea v-model="textToShorten" autoResize placeholder="Text to shorten..."/>
        </InputContainer>
        <InputContainer>
            <Label text="Maximum length:"/>
            <NumberInput class="w-16" v-model="length"/>
        </InputContainer>
        <div>
            <Shortener :text="textToShorten" :length="length"/>
        </div>
    </Form>
</template>

<script setup lang="ts">
import Shortener from '@/c-lib/components/core/text/Shortener.vue'
import TextArea from '@/c-lib/components/core/forms/inputs/text/TextArea.vue'
import Label from '@/c-lib/components/core/forms/Label.vue'
import InputContainer from '@/c-lib/components/core/forms/InputContainer.vue'
import { ref } from 'vue'
import NumberInput from '@/c-lib/components/core/forms/inputs/numbers/NumberInput.vue'
import Form from '@/c-lib/components/core/forms/Form.vue'

const textToShorten = ref('')
const length = ref(10)
</script>