<template>
    <TableDataView :metaData="metaData" :dataSets="dataSets" v-model:selected="selected" v-slot:attributeValue="{ attribute, value }">
        <CustomAttributeView v-if="attribute.key === 'name'" :attribute="attribute" :value="value"/>
        <DefaultAttributeView v-else :attribute="attribute" :value="value"/>
    </TableDataView>
    <TableDataView :metaData="{ ...metaData, selectable: false, actions: [] }" :dataSets="selected"/>
</template>

<script setup lang="ts">
import TableDataView from '@/c-lib/components/widgets/dataViews/table/TableDataView.vue'
import type { DataSet, MetaData } from '@/c-lib/types/dataView.ts'
import { ref } from 'vue'
import DefaultAttributeView from '@/c-lib/components/widgets/dataViews/table/attributes/DefaultAttributeView.vue'
import CustomAttributeView from '@/components/componentDemo/widgetDemo/dataViewDemo/CustomAttributeView.vue'

const metaData: MetaData = {
    attributes: [
        { label: 'Name', key: 'name', type: { name: 'string' } },
        { label: 'Description', key: 'description', type: { name: 'string', limit: 10 } },
        { label: 'Category', key: 'category', get: (dataSet: DataSet, key: string) => dataSet[key].toUpperCase(), type: { name: 'string' } },
        { label: 'Amount', key: 'amount', type: { name: 'number', format: 'currency' } },
        { label: 'Date', key: 'date', type: { name: 'datetime', format: 'date' } },
    ],
    actions: [
        { label: 'Global', icon: 'clipboard', onClick: dataSets => console.log('Global:', JSON.stringify(dataSets)) },
        { label: 'Test', icon: 'clipboard', onClick: dataSets => console.log('Test:', JSON.stringify(dataSets)), enable: dataSet => dataSet.id !== 'a' },
        { label: 'Delete', icon: 'clipboard', severity: 'error', onClick: dataSets => console.log('delete:', JSON.stringify(dataSets)), enable: dataSet => dataSet.id !== 'a' },
    ],
    selectable: true,
}

const dataSets = ref<DataSet[]>([
    { id: 'a', name: 'Name A', description: 'Description A', category: 'first', amount: 40, date: new Date(Date.now())},
    { id: 'b', name: 'Name B', description: 'Description B', category: 'first', amount: 30, date: new Date(Date.now())},
    { id: 'c', name: 'Name C', description: 'Description C', category: 'second', amount: 80, date: new Date(Date.now())},
    { id: 'd', name: 'Name D', description: 'Description D', category: 'second', amount: 90, date: new Date(Date.now())},
])

const selected = ref<DataSet[]>([])
</script>