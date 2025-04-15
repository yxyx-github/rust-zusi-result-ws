import type { MaybeRefOrGetter } from 'vue'
import { computed, toValue } from 'vue'

export function useCardClass(isCard: MaybeRefOrGetter<boolean>) {
    return computed(() => toValue(isCard) ? 'shadow-md border border-secondary-200' : '')
}