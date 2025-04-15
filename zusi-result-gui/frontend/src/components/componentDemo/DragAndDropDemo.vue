<template>
    <Column :gap="1">
        <Card v-for="item in items" :key="item" v-drag:item="item" v-drag-over:class="'border-b-fg'" v-drop:item="(identifier: any) => onDrop(item, identifier)">
            {{ item }}
        </Card>
    </Column>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import Card from '@/c-lib/components/core/containers/Card.vue'
import { vDrag } from '@/c-lib/directives/dragAndDrop/v-drag'
import { vDragOver } from '@/c-lib/directives/dragAndDrop/v-dragOver'
import { vDrop } from '@/c-lib/directives/dragAndDrop/v-drop'

const items = ref([
    'ItemA',
    'ItemB',
    'ItemC',
    'ItemD',
])

function onDrop(item: string, dragged: string) {
    let newItems: string[] = []
    items.value.forEach(currentItem => {
        if (currentItem !== dragged) {
            newItems.push(currentItem)
        }
        if (currentItem === item) {
            newItems.push(dragged)
        }
    })
    items.value = newItems
}
</script>