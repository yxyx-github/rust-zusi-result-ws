<template>
    <Button prependIcon="chevron-left"
            :showLabel="false"
            :title="$t('c.backwards')"
            @click="router.go(-1)"
    />
    <Button v-if="enableForwards"
            prependIcon="chevron-right"
            :showLabel="false"
            :title="$t('c.forwards')"
            @click="router.go(1)"
    />
    <Button
            prependIcon="home"
            :showLabel="false"
            :title="$t('c.home')"
            :disabled="props.homeRouteName === router.currentRoute.value.name"
            @click="router.push({ name: props.homeRouteName })"
    />
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'

const router = useRouter()

const props = withDefaults(defineProps<{
    homeRouteName: string
    enableForwards?: boolean
}>(), {
    enableForwards: false,
})
</script>