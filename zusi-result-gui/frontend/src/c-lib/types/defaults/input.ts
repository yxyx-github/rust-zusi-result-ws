import type { CheckboxProps, DateTimeInputProps, InputProps, NumberInputProps, SelectProps, TextAreaProps, TextInputProps } from '@/c-lib/types/input.ts'

export const defaultInputProps: InputProps = {
    initialFocus: false,
    disabled: false,
    severity: 'primary',
}

export const defaultTextInputProps: TextInputProps = {
    type: 'text',
}

export const defaultNumberInputProps: NumberInputProps = {}

export const defaultDateTimeInputProps: DateTimeInputProps = {
    type: 'datetime',
}

export const defaultTextAreaProps: TextAreaProps = {
    autoResize: false,
    rows: 3,
}

export const defaultCheckboxProps: CheckboxProps = {
    default: undefined,
    trueIcon: 'check',
    falseIcon: ' ',
    nullIcon: 'minus',
    grayScaleFalse: true,
}

export const defaultSelectProps: Partial<SelectProps> = {
    default: null,
}