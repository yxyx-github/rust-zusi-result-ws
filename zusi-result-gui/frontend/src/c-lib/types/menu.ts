import type { Severity } from '@/c-lib/types/index.ts'

export type MenuItem = {
    onClick?: (e: Event) => void
    label: string
    showLabel?: boolean | null
    title?: string
    severity?: Severity
    icon?: string
    disabled?: boolean
    children?: MenuItem[]
    [x: string]: any
}

export type MenuItemData = {
    item: MenuItem
    hasIcon: boolean
    hasSubMenu: boolean
    enableFocus: boolean // TODO: choose better name (true means set focus, false means do nothing with focus)
    updateFocus: (value: boolean) => void
}

export type SubMenuAlign = 'horizontal' | 'vertical'

export type SubMenuXAlignDirection = 'left' | 'right'
export type SubMenuYAlignDirection = 'top' | 'bottom'

export type MenuItemPath = number[]

export type MenuTrigger = 'click' | 'hover'

export type MenuPosition = 'fixed' | 'absolute'

export type DropdownProps = {
    items: MenuItem[]
    align?: SubMenuAlign
    preferredXDirection?: SubMenuXAlignDirection
    preferredYDirection?: SubMenuYAlignDirection
    trigger?: MenuTrigger
    position?: MenuPosition
    limitByOverflowParent?: boolean
}