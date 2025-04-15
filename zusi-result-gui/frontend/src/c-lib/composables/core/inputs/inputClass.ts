import type { MaybeRefOrGetter } from 'vue'
import { computed, toValue } from 'vue'
import type { Severity } from '@/c-lib/types'

export function useInputClass(severity: MaybeRefOrGetter<Severity>) {
    const outlineSeverityClass = computed(() => {
        switch (toValue(severity)) {
            case 'success':
                return 'outline-success-400'
            case 'warning':
                return 'outline-warning-400'
            case 'error':
                return 'outline-error-400'
            case 'secondary':
                return 'outline-secondary-400'
            default:
            case 'primary':
                return 'outline-primary-400'
        }
    })

    return `input ${outlineSeverityClass.value}`
}