<template>
    <span v-if="props.name !== ''" class="font-inherit w-[1em] h-[1em]">
        <SvgIconLoader v-if="props.name !== ' '" :name="props.name"/>
    </span>
</template>

<script setup lang="ts">
import SvgIconLoader from '@/c-lib/components/core/icons/SvgIconLoader.vue'

const props = defineProps<{
    name: string,
}>()
</script>