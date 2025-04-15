import type { Severity, Size } from '@/c-lib/types/index.ts'

export type InputProps = {
    initialFocus?: boolean
    disabled?: boolean
    severity?: Severity
}

export type TextInputProps = {
    type?: 'text' | 'password'
}

export type NumberInputProps = {
    min?: number
    max?: number
    step?: number
}

export type DateTimeInputProps = {
    type?: 'date' | 'time' | 'datetime'
}

export type TextAreaProps = {
    rows?: number
    cols?: number
    autoResize?: boolean
}

export type CheckboxProps = {
    default?: boolean | null
    trueIcon?: string
    falseIcon?: string
    nullIcon?: string
    grayScaleFalse?: boolean
}

export type SelectItem = {
    label?: string
    value: string
    [x: string]: any
}

export type SelectProps = {
    items: SelectItem[]
    default?: SelectItem | null
}