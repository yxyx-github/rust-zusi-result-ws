<template>
    <ContextMenu :items="actionMenuItems" v-slot:trigger="{ onContextMenu }">
        <TR @contextmenu.prevent="actionMenuItems.length > 0 ? $event.stopPropagation() || onContextMenu($event) : undefined">
            <TD v-if="isSelectable || hasActions">
                <Row :gap="2" :wrap="false" alignItems="center">
                    <Checkbox v-if="isSelectable" compact v-model="isSelected"/>
                    <MenuBar :items="actionMenuItems" v-if="actionMenuItems.length > 0" prependIcon="more-vertical" variant="ghost" compact :showLabel="null" buttonLabelThreshold="xl"/>
                </Row>
            </TD>
            <TD v-for="attribute in props.metaData.attributes" :key="attribute.key">
                <slot name="attributeValue" :attribute="attribute" :value="getAttributeValue(attribute, props.dataSet)">
                    <DefaultAttributeView :attribute="attribute" :value="getAttributeValue(attribute, props.dataSet)"/>
                </slot>
            </TD>
        </TR>
    </ContextMenu>
</template>

<script setup lang="ts">
import type { Action, Attribute, AttributeValue, DataSet, MetaData } from '@/c-lib/types/dataView.ts'
import TR from '@/c-lib/components/core/tables/TR.vue'
import DefaultAttributeView from '@/c-lib/components/widgets/dataViews/table/attributes/DefaultAttributeView.vue'
import TD from '@/c-lib/components/core/tables/TD.vue'
import Checkbox from '@/c-lib/components/core/forms/inputs/boolean/Checkbox.vue'
import { computed } from 'vue'
import ContextMenu from '@/c-lib/components/core/menus/context/ContextMenu.vue'
import type { MenuItem } from '@/c-lib/types/menu.ts'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import MenuBar from '@/c-lib/components/core/menus/bar/MenuBar.vue'
import Dropdown from '@/c-lib/components/core/menus/dropdown/Dropdown.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'

const props = defineProps<{
    metaData: MetaData
    dataSet: DataSet
}>()

const selected = defineModel<DataSet[]>('selected', { required: true })

const isSelected = computed<boolean>({
    get: () => selected.value.some(selected => selected.id === props.dataSet.id),
    set: value => {
        if (value && !selected.value.some(selected => selected.id === props.dataSet.id)) {
            selected.value = [ ...selected.value, props.dataSet ]
        } else if (!value) {
            selected.value = selected.value.filter(selected => selected.id !== props.dataSet.id)
        }
    },
})

const isSelectable = computed<boolean>(() => props.metaData.selectable ?? false)

const hasActions = computed<boolean>(() => (props.metaData.actions?.length ?? 0) > 0)

const actionMenuItems = computed<MenuItem[]>(() =>
    props.metaData.actions?.filter((action: Action) =>
        action.enable === undefined ? true : action.enable(props.dataSet)
    ).map((action: Action) => ({
        label: action.label,
        title: action.label,
        severity: action.severity,
        icon: action.icon,
        onClick: () => action.onClick([props.dataSet])
    } as MenuItem)) ?? []
)

function getAttributeValue(attribute: Attribute<any>, dataSet: DataSet): AttributeValue {
    return attribute.get === undefined
            ? dataSet[attribute.key]
            : attribute.get(dataSet, attribute.key)
}
</script>