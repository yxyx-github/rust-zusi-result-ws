import type { DropdownProps } from '@/c-lib/types/menu.ts'

export const defaultDropdownProps: Omit<DropdownProps, 'items'> = {
    align: 'vertical',
    preferredXDirection: 'right',
    preferredYDirection: 'bottom',
    trigger: 'click',
    position: 'absolute',
    limitByOverflowParent: false,
}