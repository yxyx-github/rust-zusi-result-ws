import type { Severity } from '@/c-lib/types/index.ts'
import type { Threshold } from '@/c-lib/types/screens.ts'

export type ButtonVariant = 'normal' | 'link' | 'ghost' | 'menu'

export type ButtonType = 'button' | 'submit' | 'reset'

export type ButtonStyleImplementorProps = {
    severity?: Severity
    type?: ButtonType
    disabled?: boolean
    focusable?: boolean
    initialFocus?: boolean
}

export type ButtonProps = {
    variant?: ButtonVariant
    label?: string
    textAlign?: 'left' | 'right' | 'center'
    prependIcon?: string
    appendIcon?: string
    showLabel?: boolean | null
    threshold?: Threshold
    overrideFocus?: boolean | null
} & ButtonStyleImplementorProps