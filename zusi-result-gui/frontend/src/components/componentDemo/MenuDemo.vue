<template>
    <ContextMenu :items="menuItems" class="grow">
        <Menu :items="inlineMenuItems"/>
        <Column alignItems="center">
            <Dropdown :items="menuItems" preferredXDirection="left" preferredYDirection="top" trigger="hover" position="fixed">Dropup</Dropdown>
            <Dropdown :items="menuItems" preferredXDirection="right" preferredYDirection="bottom">
                Dropdown
                <template #menuItem="{ data }">
                    <CustomMenuItem v-if="data.item.isLink ?? false" :data="data"/>
                    <DefaultMenuItem v-else :data="data"/>
                </template>
            </Dropdown>
        </Column>
    </ContextMenu>
    <div class="border border-blue-600">
        <ContextMenu :items="menuItems" v-slot:trigger="{ onContextMenu }">
            <div class="bg-amber-50 h-32" @contextmenu.prevent.stop="onContextMenu">
                ContextMenu without extra trigger container
            </div>
        </ContextMenu>
    </div>
</template>

<script setup lang="ts">
import Menu from '@/c-lib/components/core/menus/Menu.vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import ContextMenu from '@/c-lib/components/core/menus/context/ContextMenu.vue'
import Dropdown from '@/c-lib/components/core/menus/dropdown/Dropdown.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import CustomMenuItem from '@/components/componentDemo/menuDemo/CustomMenuItem.vue'
import DefaultMenuItem from '@/c-lib/components/core/menus/DefaultMenuItem.vue'

const menuItems: MenuItem[] = [
    { label: 'First', onClick: () => console.log('First'), icon: 'clipboard', isLink: true },
    { label: 'Second A (With Title)', icon: 'clipboard', title: 'Second A Title', isLink: true, onClick: () => console.log('Second') },
    { label: 'Second B (With Title)', icon: 'clipboard', title: 'Second B Title', isLink: true, children: [
        { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
        { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
        { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
        { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
        { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
        { label: 'CCCCCCCCCC', children: [
            { label: 'AAAAAAAAAA', onClick: () => console.log('A'), icon: 'clipboard' },
            { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
            { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
        ] },
        { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
        { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
        { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
        { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
        { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
        { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
    ] },
    { label: 'Third (Primary)', onClick: () => console.log('Third'), severity: 'primary', isLink: true },
]

const inlineMenuItems: MenuItem[] = [
    { label: 'TestA', onClick: () => console.log('TestA') },
    { label: 'TestB', onClick: () => console.log('TestB') },
    { label: 'TestC', onClick: () => console.log('TestC') },
]
</script>