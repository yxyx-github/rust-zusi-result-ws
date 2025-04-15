import type { ButtonProps, ButtonStyleImplementorProps } from '@/c-lib/types/button.ts'

export const defaultButtonStyleImplementorProps: ButtonStyleImplementorProps = {
    type: 'button',
    disabled: false,
    focusable: true,
    initialFocus: false,
}

export const defaultButtonProps: ButtonProps = {
    ...defaultButtonStyleImplementorProps,
    variant: 'normal',
    label: '',
    textAlign: 'center',
    prependIcon: '',
    appendIcon: '',
    showLabel: true,
    threshold: 'sm',
    overrideFocus: null,
}