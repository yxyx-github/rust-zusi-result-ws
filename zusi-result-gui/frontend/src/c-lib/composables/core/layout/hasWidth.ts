import { computed, type MaybeRefOrGetter, toValue } from 'vue'

export function useHasWidth(className: MaybeRefOrGetter<string>) {
    const hasWidth = computed<boolean>(() => toValue(className).match(/^(.* )?w-[0-9\\.]+( .*)?$/) !== null)

    return hasWidth
}