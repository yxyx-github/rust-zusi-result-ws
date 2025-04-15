import type { MaybeRefOrGetter } from 'vue'
import type { Size } from '@/c-lib/types'
import { computed, ref, toValue, watchEffect } from 'vue'

export function useSize(size: MaybeRefOrGetter<Size>, compact: MaybeRefOrGetter<boolean>) {
    const gapSize = ref(0)
    const equalAspectRatioSizeClass = ref('')
    const xSizeClass = ref('')
    const ySizeClass = ref('')
    const fontSizeClass = ref('')

    watchEffect(() => {
        const isCompact = toValue(compact)
        switch (toValue(size)) {
            case 'sm':
                gapSize.value = isCompact ? 0.5 : 2
                equalAspectRatioSizeClass.value = isCompact ? 'p-0.5' : 'p-2'
                xSizeClass.value = isCompact ? 'px-0.5' : 'px-2'
                ySizeClass.value = isCompact ? 'py-0' : 'py-1'
                fontSizeClass.value = 'text-sm'
                break
            case 'md':
                gapSize.value = isCompact ? 1 : 3
                equalAspectRatioSizeClass.value = isCompact ? 'p-1' : 'p-3'
                xSizeClass.value = isCompact ? 'px-1' : 'px-3'
                ySizeClass.value = isCompact ? 'py-0' : 'py-1.5'
                fontSizeClass.value = 'text-base'
                break
            case 'lg':
                gapSize.value = isCompact ? 1.5 : 4
                equalAspectRatioSizeClass.value = isCompact ? 'p-1.5' : 'p-4'
                xSizeClass.value = isCompact ? 'px-1.5' : 'px-4'
                ySizeClass.value = isCompact ? 'py-0' : 'py-2'
                fontSizeClass.value = 'text-lg'
                break
        }
    })

    const sizeClass = computed(() => `${xSizeClass.value} ${ySizeClass.value} ${fontSizeClass.value}`)

    return {
        gapSize,
        equalAspectRatioSizeClass,
        xSizeClass,
        ySizeClass,
        fontSizeClass,
        sizeClass,
    }
}