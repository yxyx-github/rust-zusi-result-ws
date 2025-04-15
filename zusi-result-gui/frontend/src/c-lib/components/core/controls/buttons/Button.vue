<template>
    <component
            ref="buttonComponentElement"
            :is="buttonComponent"
            :severity="props.severity"
            :size="props.size"
            :compact="props.compact"
            :type="props.type"
            :disabled="props.disabled"
            :focusable="props.focusable"
            :initialFocus="props.initialFocus"
            :="$attrs">
        <span class="pointer-events-none flex flex-row flex-nowrap gap-0 font-inherit">
            <span :class="`pointer-events-none flex flex-row flex-nowrap grow items-center font-inherit ${gapClass} ${iconAlginClass}`">
                <Icon :name="props.prependIcon" class="grow-0"/>
                <ExtraInfo :class="`grow ${textAlignClass}`" :show="showContent" @show="setTextIsShown($event)" :threshold="props.threshold">
                    <slot>{{ props.label }}</slot>
                </ExtraInfo>
                <Icon :name="props.appendIcon" class="grow-0"/>
            </span>
            <span class="invisible w-0 pointer-events-none font-inherit">.</span>
        </span>
    </component>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, useSlots, watch } from 'vue'
import ExtraInfo from '@/c-lib/components/core/text/ExtraInfo.vue'
import GhostButton from '@/c-lib/components/core/controls/buttons/styles/GhostButton.vue'
import LinkButton from '@/c-lib/components/core/controls/buttons/styles/LinkButton.vue'
import NormalButton from '@/c-lib/components/core/controls/buttons/styles/NormalButton.vue'
import MenuButton from '@/c-lib/components/core/controls/buttons/styles/MenuButton.vue'
import Icon from '@/c-lib/components/core/icons/Icon.vue'
import type { ButtonProps } from '@/c-lib/types/button.ts'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

defineOptions({
    inheritAttrs: false,
})

const slots = useSlots()

const props = withDefaults(defineProps<ButtonProps & SizeProps>(), {
    ...defaultButtonProps,
    ...defaultSizeProps,
})

const buttonComponentElement = ref<any>(null)

function element(): any {
    return buttonComponentElement.value.element()
}

watch(() => props.overrideFocus, updateFocus)

onMounted(updateFocus)

function updateFocus() {
    if (props.overrideFocus === true) {
        element().focus()
    } else if (props.overrideFocus === false) {
        element().blur()
    }
}

const textIsShown = ref<boolean>(false)

function setTextIsShown(newValue: boolean) {
    // console.log('sTIS: ', newValue)
    textIsShown.value = newValue
}

const buttonComponent = computed(() => {
    switch (props.variant) {
        case 'menu':
            return MenuButton
        case 'ghost':
            return GhostButton
        case 'link':
            return LinkButton
        default:
        case 'normal':
            return NormalButton
    }
})

const hasIcon = computed(() => hasPrependIcon.value || hasAppendIcon.value)
const hasPrependIcon = computed(() => props.prependIcon !== "")
const hasAppendIcon = computed(() => props.appendIcon !== "")
const hasContent = computed(() => slots.default || props.label !== '')
const showContent = computed(() => hasContent.value ? (hasIcon.value ? props.showLabel : true) : false)
const textAlignClass = computed(() =>
    props.textAlign === 'left' ? 'text-left' : (
        props.textAlign === 'right' ? 'text-right' : 'text-center'
    )
)
const iconAlginClass = computed(() =>
    props.textAlign === 'left' ? 'justify-start' : (
        props.textAlign === 'right' ? 'justify-end' : 'justify-center'
    )
)
const gapClass = computed(() => {
    switch (props.size) {
        case 'sm':
            return 'gap-1.5'
        case 'md':
            return 'gap-2.5'
        case 'lg':
            return 'gap-3.5'
    }
})

defineExpose({
    element,
})
</script>