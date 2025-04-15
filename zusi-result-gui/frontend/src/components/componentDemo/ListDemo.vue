<template>
    <template v-for="compact in [true, false]" :key="compact">
        <List v-for="size in ['sm', 'md', 'lg'] as Size[]" :key="size" :compact="compact">
            <ListItem text="Item A" :size="size" :compact="compact"/>
            <ListItem text="Item B" :size="size" :compact="compact"/>
            <ListItem text="Item C" :size="size" :compact="compact"/>
            <SubList text="SubList" :size="size" :compact="compact">
                <ListItem text="Sub Item A" :size="size" :compact="compact"/>
                <ListItem text="Sub Item B" :size="size" :compact="compact"/>
                <ListItem text="Sub Item C" :size="size" :compact="compact"/>
                <SubList text="SubSubList" :size="size" :compact="compact">
                    <ListItem text="SubSub Item A" :size="size" :compact="compact"/>
                    <ListItem text="SubSub Item B" :size="size" :compact="compact"/>
                    <ListItem text="SubSub Item C" :size="size" :compact="compact"/>
                </SubList>
            </SubList>
        </List>
    </template>
</template>

<script setup lang="ts">
import List from '@/c-lib/components/core/lists/List.vue'
import ListItem from '@/c-lib/components/core/lists/ListItem.vue'
import SubList from '@/c-lib/components/core/lists/SubList.vue'
import type { Size } from '@/c-lib/types'
</script>

<style scoped>
</style>