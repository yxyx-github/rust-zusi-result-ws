import { h } from 'vue'
import type { FlexProps } from '@/c-lib/types/flex.ts'

export const defaultFlexProps: FlexProps = {
    direction: 'row',
    alignItems: 'stretch',
    justifyItems: 'start',
    gap: 4,
    wrap: false,
    grow: false,
    shrink: false,
    is: h('div'),
}