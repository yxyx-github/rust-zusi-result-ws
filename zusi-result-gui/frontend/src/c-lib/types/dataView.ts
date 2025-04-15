import type { Severity } from '@/c-lib/types/index.ts'

export type DataSetId = string
export type AttributeValue = any

export type DataSet = {
    id: DataSetId
    [key: string]: AttributeValue
}

export type Attribute<T extends AttributeType> = {
    label: string
    key: string
    get?: (dataSet: DataSet, key: string) => AttributeValue
    type: T
}

export interface AttributeType {
    name: string
    [option: string]: any
}

export interface StringAttributeType extends AttributeType {
    name: 'string'
    limit?: number,
}

export interface NumberAttributeType extends AttributeType {
    name: 'number'
    format?: string
}

export interface DateTimeAttributeType extends AttributeType {
    name: 'datetime'
    format?: string
}

export type Action = {
    onClick: (dataSets: DataSet[]) => void
    label: string
    severity?: Severity
    icon?: string
    enable?: (dataSet: DataSet) => boolean
    multiple?: boolean
}

export type MetaData = {
    attributes: Attribute<any>[]
    actions?: Action[]
    selectable?: boolean
}

export type AttributeViewProps<T extends AttributeType> = {
    attribute: Attribute<T>
    value: AttributeValue
}