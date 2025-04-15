<template>
    <ContextMenu :is="Column" :items="menuItems">
        <List>
            <SubList text="Icon">
                <ListItem>
                    <Icon name="x" class="text-red-600"/>
                </ListItem>
            </SubList>
            <SubList text="ButtonDemo">
                <ListItem>
                    <ButtonDemo/>
                </ListItem>
            </SubList>
            <SubList text="ToggleControlDemo">
                <ListItem>
                    <ToggleControlDemo/>
                </ListItem>
            </SubList>
            <SubList text="SignDemo">
                <ListItem>
                    <SignDemo/>
                </ListItem>
            </SubList>
            <SubList text="TableDemo">
                <ListItem>
                    <TableDemo/>
                </ListItem>
            </SubList>
            <SubList text="MessageDemo">
                <ListItem>
                    <MessageDemo/>
                </ListItem>
            </SubList>
            <SubList text="MenuDemo">
                <ListItem>
                    <MenuDemo/>
                </ListItem>
            </SubList>
            <SubList text="FormDemo">
                <ListItem>
                    <FormDemo/>
                </ListItem>
            </SubList>
            <SubList text="ListDemo">
                <ListItem>
                    <ListDemo/>
                </ListItem>
            </SubList>
            <SubList text="ModalDemo">
                <ListItem>
                    <ModalDemo/>
                </ListItem>
            </SubList>
            <SubList text="DialogDemo">
                <ListItem>
                    <DialogDemo/>
                </ListItem>
            </SubList>
            <SubList text="TextDemo">
                <ListItem>
                    <TextDemo/>
                </ListItem>
            </SubList>
            <SubList text="LayoutDemo">
                <ListItem>
                    <LayoutDemo/>
                </ListItem>
            </SubList>
            <SubList text="DragAndDropDemo">
                <ListItem>
                    <DragAndDropDemo/>
                </ListItem>
            </SubList>
            <SubList text="WidgetDemo" show>
                <SubList text="BarDemo">
                    <ListItem>
                        <BarDemo/>
                    </ListItem>
                </SubList>
                <SubList text="DataViewDemo">
                    <ListItem>
                        <DataViewDemo/>
                    </ListItem>
                </SubList>
            </SubList>
        </List>
    </ContextMenu>
</template>

<script setup lang="ts">
import Icon from '@/c-lib/components/core/icons/Icon.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import ButtonDemo from '@/components/componentDemo/ButtonDemo.vue'
import MenuDemo from '@/components/componentDemo/MenuDemo.vue'
import ContextMenu from '@/c-lib/components/core/menus/context/ContextMenu.vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import FormDemo from '@/components/componentDemo/FormDemo.vue'
import MessageDemo from '@/components/componentDemo/MessageDemo.vue'
import TableDemo from '@/components/componentDemo/TableDemo.vue'
import SignDemo from '@/components/componentDemo/SignDemo.vue'
import ListDemo from '@/components/componentDemo/ListDemo.vue'
import List from '@/c-lib/components/core/lists/List.vue'
import SubList from '@/c-lib/components/core/lists/SubList.vue'
import ListItem from '@/c-lib/components/core/lists/ListItem.vue'
import ModalDemo from '@/components/componentDemo/ModalDemo.vue'
import TextDemo from '@/components/componentDemo/TextDemo.vue'
import ToggleControlDemo from '@/components/componentDemo/ToggleControlDemo.vue'
import LayoutDemo from '@/components/componentDemo/LayoutDemo.vue'
import BarDemo from '@/components/componentDemo/widgetDemo/BarDemo.vue'
import DragAndDropDemo from '@/components/componentDemo/DragAndDropDemo.vue'
import DataViewDemo from '@/components/componentDemo/widgetDemo/DataViewDemo.vue'
import DialogDemo from '@/components/componentDemo/DialogDemo.vue'

const menuItems: MenuItem[] = [
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
    { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
    { label: 'CCCCCCCCCC', onClick: () => console.log('C'), severity: 'primary' },
]
</script>