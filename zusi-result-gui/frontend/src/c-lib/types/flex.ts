import type { Threshold } from '@/c-lib/types/screens.ts'

export type Direction = 'row' | 'col'
export type AlignItems = 'start' | 'center' | 'end' | 'stretch' | 'baseline'
export type JustifyItems = 'start' | 'center' | 'end' | 'between' | 'around' | 'evenly'

export type FlexProps = {
    direction?: { [key in Threshold]?: Direction } | Direction
    alignItems?: { [key in Threshold]?: AlignItems } | AlignItems
    justifyItems?: { [key in Threshold]?: JustifyItems } | JustifyItems
    gap?: { [key in Threshold]?: number } | number
    wrap?: { [key in Threshold]?: boolean } | boolean
    grow?: { [key in Threshold]?: boolean } | boolean
    shrink?: { [key in Threshold]?: boolean } | boolean
    is?: any
}