<template>
    <TitleBar>
        <template #left>
            <Button prependIcon="check"/>
            <Button prependIcon="check"/>
        </template>
        TitleBarDemo
        <template #right>
            <MenuBar :items="menuItems" preferredXDirection="left" :showLabel="null" buttonLabelThreshold="lg"/>
        </template>
    </TitleBar>
    <Row>
        <SideBar>
            <template #top>
                <MenuBar :items="menuItems" direction="col" :showLabel="false"/>
            </template>
            <template #bottom>
                <Button prependIcon="check"/>
                <Button prependIcon="check"/>
            </template>
        </SideBar>
        <Column :is="ContentBox">
            Page content...
        </Column>
    </Row>
</template>

<script setup lang="ts">
import TitleBar from '@/c-lib/components/widgets/navigation/TitleBar.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import SideBar from '@/c-lib/components/widgets/navigation/SideBar.vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import MenuBar from '@/c-lib/components/core/menus/bar/MenuBar.vue'

const menuItems: MenuItem[] = [
    { label: 'First', onClick: () => console.log('First'), icon: 'clipboard', isLink: true },
    { label: 'Second A (With Title)', icon: 'clipboard', title: 'Second A Title', isLink: true, onClick: () => console.log('Second') },
    {
        label: 'Second B (With Title)', title: 'Second B Title', isLink: true, children: [
            { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
            { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
            { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
            { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
            { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
            {
                label: 'CCCCCCCCCC', children: [
                    { label: 'AAAAAAAAAA', onClick: () => console.log('A'), icon: 'clipboard' },
                    { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
                    { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
                ]
            },
            { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
            { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
            { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
            { label: 'AAAAAAAAAA', onClick: () => console.log('A') },
            { label: 'BBBBBBBBBB', onClick: () => console.log('B') },
            { label: 'CCCCCCCCCC', onClick: () => console.log('C') },
        ]
    },
    { label: 'Third (Primary)', onClick: () => console.log('Third'), severity: 'primary', isLink: true },
]
</script>