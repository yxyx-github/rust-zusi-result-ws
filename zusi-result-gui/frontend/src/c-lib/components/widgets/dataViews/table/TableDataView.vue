<template>
    <ContextMenu :items="actionMenuItems" v-slot:trigger="{ onContextMenu }">
        <Table @contextmenu.capture.prevent="actionMenuItems.length > 0 ? $event.stopPropagation() || onContextMenu($event) : undefined">
            <THead>
                <TR :highlightOnFocus="false" :highlightOnHover="false">
                    <TH v-if="isSelectable || hasActions">
                        <Row :gap="2" :wrap="false" alignItems="center" class="font-normal">
                            <Checkbox v-if="isSelectable" compact v-model="allSelected"/>
                            <MenuBar :items="actionMenuItems" v-if="actionMenuItems.length > 0" prependIcon="more-vertical" variant="ghost" compact :showLabel="null" buttonLabelThreshold="xl"/>
                        </Row>
                    </TH>
                    <TH v-for="attribute in props.metaData.attributes" :key="attribute.key">{{ attribute.label }}</TH>
                </TR>
            </THead>
            <TBody>
                <TableDataViewRow v-for="dataSet in props.dataSets" :key="dataSet.id" :metaData="props.metaData" :dataSet="dataSet" v-model:selected="selected">
                    <template #attributeValue="data" v-if="$slots.attributeValue">
                        <slot name="attributeValue" :="data"/>
                    </template>
                </TableDataViewRow>
            </TBody>
        </Table>
    </ContextMenu>
</template>

<script setup lang="ts">
import type { Action, DataSet, MetaData } from '@/c-lib/types/dataView.ts'
import Table from '@/c-lib/components/core/tables/Table.vue'
import THead from '@/c-lib/components/core/tables/THead.vue'
import TR from '@/c-lib/components/core/tables/TR.vue'
import TH from '@/c-lib/components/core/tables/TH.vue'
import TBody from '@/c-lib/components/core/tables/TBody.vue'
import Checkbox from '@/c-lib/components/core/forms/inputs/boolean/Checkbox.vue'
import TableDataViewRow from '@/c-lib/components/widgets/dataViews/table/TableDataViewRow.vue'
import { computed } from 'vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import MenuBar from '@/c-lib/components/core/menus/bar/MenuBar.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import ContextMenu from '@/c-lib/components/core/menus/context/ContextMenu.vue'

const props = withDefaults(defineProps<{
    metaData: MetaData
    dataSets: DataSet[]
    sticky?: boolean
}>(), {
    sticky: false,
})

const hasActions = computed<boolean>(() => (props.metaData.actions?.length ?? 0) > 0)

const isSelectable = computed<boolean>(() => props.metaData.selectable ?? false)

const selected = defineModel<DataSet[]>('selected', { default: [] })

const allSelected = computed<boolean |  null>({
    get: () => props.dataSets.every(dataSet => selected.value.some(selected => selected.id === dataSet.id)) ? true : (selected.value.length > 0 ? null : false),
    set: value => value ? selected.value = props.dataSets : selected.value = [],
})

const actionMenuItems = computed<MenuItem[]>(() =>
    props.metaData.actions?.map((action: Action) => ({
        label: action.label,
        title: action.label,
        severity: action.severity,
        icon: action.icon,
        disabled: !(
            (action.enable === undefined ? true : selected.value.every(dataSet => action.enable?.(dataSet) ?? true))
            && selected.value.length > 0
        ),
        onClick: () => action.onClick(selected.value)
    } as MenuItem)) ?? []
)
</script>