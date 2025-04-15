import type { MaybeRefOrGetter } from 'vue'
import { computed, toValue } from 'vue'

export function useThresholdClass(threshold: MaybeRefOrGetter<string>, hide: MaybeRefOrGetter<string>, show: MaybeRefOrGetter<string>) {
    const thresholdVal = toValue(threshold)
    const hideVal = toValue(hide)
    const showVal = toValue(show)
    
    const smallClass = computed(() => `${thresholdVal === 'mb' ? '' : `mb:${showVal}`} ${thresholdVal}:${hideVal}`)
    const largeClass = computed(() => `${thresholdVal === 'mb' ? '' : `mb:${hideVal}`} ${thresholdVal}:${showVal}`)

    return {
        smallClass,
        largeClass,
    }
}