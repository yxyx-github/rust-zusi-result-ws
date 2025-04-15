<template>
    <PageRoot>
        <Column :gap="0" grow class="h-full">
            <TitleBar class="z-10">
                <template #left v-if="pageName !== 'home'">
                    <WebHistoryNavigation homeRouteName="home"/>
                </template>
                <PageTitle appName="UI-Component-Lib" :pageName="pageName"/>
                <template #right>
                    <MenuBar :items="menuItems" preferredXDirection="left" severity="warning"/>
                </template>
            </TitleBar>
            <ContentBox direction="col" grow shrink class="z-0 overflow-auto">
                <MessageLocationViewer location=""/>
                <RouterView/>
            </ContentBox>
        </Column>
    </PageRoot>
</template>

<script setup lang="ts">
import PageRoot from '@/c-lib/components/core/containers/PageRoot.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import TitleBar from '@/c-lib/components/widgets/navigation/TitleBar.vue'
import PageTitle from '@/c-lib/components/widgets/navigation/PageTitle.vue'
import MenuBar from '@/c-lib/components/core/menus/bar/MenuBar.vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import { useRouter } from 'vue-router'
import WebHistoryNavigation from '@/c-lib/components/widgets/navigation/WebHistoryNavigation.vue'
import { computed } from 'vue'
import MessageLocationViewer from '@/c-lib/components/core/messages/messageViewers/MessageLocationViewer.vue'

const router = useRouter()

const menuItems: MenuItem[] = [
    { label: 'Go to', icon: 'chevron-down', showLabel: null, severity: 'primary', children: [
            { label: 'TestItemA', onClick: () => router.push({ name: 'testa' }) },
            { label: 'TestItemB', onClick: () => router.push({ name: 'testb' }) },
            { label: 'TestItemC', onClick: () => router.push({ name: 'testc' }) },
        ] },
    { label: 'TestA ...........................', icon: 'home', showLabel: null, onClick: () => router.push({ name: 'home' }) },
    { label: 'TestB', icon: 'home', showLabel: false, onClick: () => router.push({ name: 'home' }) },
]

const pageName = computed<string>(() => (router.currentRoute.value.name ?? '') as string)
</script>