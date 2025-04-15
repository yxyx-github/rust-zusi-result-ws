<template>
    <component :is="svgIcon" class="font-inherit w-(1em) h-(1em)"/>
</template>

<script setup lang="ts">
import { h, ref, watch } from 'vue'

const props = defineProps<{
    name: string,
}>()

const svgIcon = ref<any>(h('div'))

watch(() => props.name, loadIcon)

loadIcon()

function loadIcon() {
    import(`../../../../assets/icons/${props.name}.svg?component`).catch(() =>
        import(`../../../assets/icons/${props.name}.svg?component`)
    ).catch(() => {
        console.warn('Icon not found:', props.name)
        return h('div')
    }).then(component => svgIcon.value = component)
}
</script>