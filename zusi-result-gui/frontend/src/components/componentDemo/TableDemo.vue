<template>
    <Form @submit.prevent>
        <InputContainer>
            <Label text="Enable limitByOverflowParent"/>
            <ButtonBar>
                <Checkbox falseIcon="x" v-model="limitByOverflowParent" :default="false" severity="primary"/>
            </ButtonBar>
        </InputContainer>
    </Form>
    <Table class="h-(600px)">
        <THead sticky>
            <TR>
                <TH/>
                <TH v-for="letter in letters" :key="letter">Column {{ letter }}</TH>
            </TR>
        </THead>
        <TBody>
            <TR v-for="number in numbers" :key="number">
                <TD>{{ number }}</TD>
                <TD v-for="letter in letters" :key="letter">
                    <Row :gap="1">
                        <div class="whitespace-nowrap cursor-default">Value {{ letter }}</div>
                        <Dropdown variant="normal" compact :items="smallMenuItems" label="Focus me!" :position="limitByOverflowParent ? 'absolute' : 'fixed'" :limitByOverflowParent="limitByOverflowParent"/>
                    </Row>
                </TD>
            </TR>
        </TBody>
    </Table>
</template>

<script setup lang="ts">
import Table from '@/c-lib/components/core/tables/Table.vue'
import THead from '@/c-lib/components/core/tables/THead.vue'
import TR from '@/c-lib/components/core/tables/TR.vue'
import TH from '@/c-lib/components/core/tables/TH.vue'
import TBody from '@/c-lib/components/core/tables/TBody.vue'
import TD from '@/c-lib/components/core/tables/TD.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import Dropdown from '@/c-lib/components/core/menus/dropdown/Dropdown.vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import Label from '@/c-lib/components/core/forms/Label.vue'
import MessageViewer from '@/c-lib/components/core/messages/messageViewers/MessageViewer.vue'
import Checkbox from '@/c-lib/components/core/forms/inputs/boolean/Checkbox.vue'
import InputContainer from '@/c-lib/components/core/forms/InputContainer.vue'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import Form from '@/c-lib/components/core/forms/Form.vue'
import { ref } from 'vue'

const letters = ['A', 'B', 'C', 'D', 'E', 'F']
const numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]

const limitByOverflowParent = ref(false)

const smallMenuItems: MenuItem[] = [
    { label: 'TestA', onClick: () => console.log('TestA') },
    { label: 'TestB', onClick: () => console.log('TestB') },
    { label: 'TestC', onClick: () => console.log('TestC') },
]

const menuItems: MenuItem[] = [
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
    { label: 'TestAMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestA') },
    { label: 'TestBMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestB') },
    { label: 'TestCMenuItemMenuItemMenuItemMenuItem', onClick: () => console.log('TestC') },
]
</script>