import { defineStore } from 'pinia'
import { ref } from 'vue'
import { v4 as uuidv4 } from 'uuid'
import type { LoadingIndicatorHandler } from '@/c-lib/types/loadingIndicators.ts'

export const useLoadingIncicatorsStore = defineStore('loadingIncicators', () => {
    const loadingIndicators = ref<LoadingIndicatorHandler[]>([])

    function start(): LoadingIndicatorHandler {
        const newLIH = {
            id: uuidv4(),
            stop: () => loadingIndicators.value = loadingIndicators.value.filter(l => l.id !== newLIH.id),
        }

        loadingIndicators.value.push(newLIH)

        return newLIH
    }

    function has(): boolean {
        return loadingIndicators.value.length > 0
    }

    return {
        start,
        has,
    }
})
