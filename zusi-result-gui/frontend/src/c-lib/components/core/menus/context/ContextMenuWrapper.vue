<template>
    <div ref="menuWrapper" :class="`${props.position === 'fixed' ? 'fixed' : 'absolute'} z-1000 flex flex-col`" :style="positionStyle">
        <slot/>
    </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, reactive, ref } from 'vue'
import type { MenuPosition, SubMenuXAlignDirection, SubMenuYAlignDirection } from '@/c-lib/types/menu.ts'
import { useElementBounding, useWindowSize } from '@vueuse/core'

type FrameBounding = {
    useFrame: boolean
    frameWidth: number
    frameHeight: number
    frameXOffset: number
    frameYOffset: number
}

const props = withDefaults(defineProps<{
    x1: number
    y1: number
    x2?: number
    y2?: number
    absX?: number
    absY?: number
    preferredXDirection?: SubMenuXAlignDirection
    preferredYDirection?: SubMenuYAlignDirection
    position?: MenuPosition
    limitByOverflowParent?: boolean
}>(), {
    absX: 0,
    absY: 0,
    preferredXDirection: 'right',
    preferredYDirection: 'bottom',
    position: 'fixed',
    limitByOverflowParent: false,
})

const menuWrapper = ref(null)

const menuSize = reactive<{
    menuWidth: number
    menuHeight: number
}>({
    menuWidth: 0,
    menuHeight: 0,
})

const { width: windowWidth, height: windowHeight } = useWindowSize()

onMounted(async () => {
    await nextTick()
    // @ts-ignore
    menuSize.menuWidth = parseFloat(window.getComputedStyle(menuWrapper.value).width)
    // @ts-ignore
    menuSize.menuHeight = parseFloat(window.getComputedStyle(menuWrapper.value).height)
})

const positionStyle = computed(() => {
    const frameWidth = frameBounding.value.useFrame ? frameBounding.value.frameWidth : windowWidth.value
    const frameHeight = frameBounding.value.useFrame ? frameBounding.value.frameHeight : windowHeight.value
    const frameXOffset = frameBounding.value.frameXOffset
    const frameYOffset = frameBounding.value.frameYOffset
    const menuContainerWidth = menuSize.menuWidth
    const menuContainerHeight = menuSize.menuHeight

    const x1 = props.x1
    const x2 = (props.x2 ?? props.x1)
    const y1 = props.y1
    const y2 = (props.y2 ?? props.y1)

    const wX1 = x1 - frameXOffset
    const wX2 = x2 - frameXOffset
    const wY1 = y1 - frameYOffset
    const wY2 = y2 - frameYOffset

    const absX = (props.position === 'absolute' ? props.absX : 0)
    const absY = (props.position === 'absolute' ? props.absY : 0)

    const xStyleLeft = `left: ${(x1 - menuContainerWidth < 0 ? frameXOffset : x1 - menuContainerWidth) - absX}px;
                        ${wX1 < menuContainerWidth ? `width: ${wX1}px;` : ''}`
    const xStyleRight = `left: ${x2 - absX}px;
                        ${frameWidth - wX2 < menuContainerWidth ? `width: ${frameWidth - wX2}px;` : ''}`
    const yStyleTop = `top: ${(y1 - menuContainerHeight < 0 ? frameYOffset : y1 - menuContainerHeight) - absY}px;
                        ${wY1 < menuContainerHeight ? `height: ${wY1}px;` : ''}`
    const yStyleBottom = `top: ${y2 - absY}px;
                        ${frameHeight - wY2 < menuContainerHeight ? `height: ${frameHeight - wY2}px;` : ''}`

    let xStyle = ''
    let yStyle = ''

    if (props.preferredXDirection === 'right') {
        xStyle =
            frameWidth - wX2 < menuContainerWidth &&
            frameWidth - wX2 < wX1
                ? xStyleLeft
                : xStyleRight
    } else {
        xStyle =
            wX1 < menuContainerWidth &&
            wX1 < frameWidth - wX2
                ? xStyleRight
                : xStyleLeft
    }
    if (props.preferredYDirection === 'bottom') {
        yStyle =
            frameHeight - wY2 < menuContainerHeight &&
            frameHeight - wY2 < wY1
                ? yStyleTop
                : yStyleBottom
    } else {
        yStyle =
            wY1 < menuContainerHeight &&
            wY1 < frameHeight - wY2
                ? yStyleBottom
                : yStyleTop
    }

    return `${xStyle} ${yStyle}`
})

const overflowParent = ref<HTMLElement | null>(null)

onMounted(() => overflowParent.value = props.limitByOverflowParent ? getOverflowParent(menuWrapper.value as unknown as HTMLElement) : null)

const { x: overflowParentX, y: overflowParentY, width: overflowParentWidth, height: overflowParentHeight } = useElementBounding(overflowParent)

const frameBounding = computed<FrameBounding>(() => {
    const bounding: FrameBounding = {
        useFrame: false,
        frameWidth: 0,
        frameHeight: 0,
        frameXOffset: 0,
        frameYOffset: 0,
    }

    if (props.limitByOverflowParent) {
        if (overflowParent.value !== null) {
            bounding.useFrame = true

            bounding.frameWidth = overflowParentWidth.value
            bounding.frameHeight = overflowParentHeight.value
            bounding.frameXOffset = overflowParentX.value
            bounding.frameYOffset = overflowParentY.value

            if (bounding.frameXOffset < 0) {
                bounding.frameWidth += bounding.frameXOffset
                bounding.frameXOffset = 0
            }
            if (bounding.frameYOffset < 0) {
                bounding.frameHeight += bounding.frameYOffset
                bounding.frameYOffset = 0
            }

            const widthDiff = windowWidth.value - bounding.frameXOffset - bounding.frameWidth
            if (widthDiff < 0) {
                bounding.frameWidth += widthDiff
            }
            const heightDiff = windowHeight.value - bounding.frameYOffset - bounding.frameHeight
            if (heightDiff < 0) {
                bounding.frameHeight += heightDiff
            }
        }
    }

    return bounding
})

function getOverflowParent(element: HTMLElement): HTMLElement | null {
    if (['auto', 'scroll'].includes(getComputedStyle(element).overflow)) {
        return element
    } else if (element.tagName.toLowerCase() === 'body' || element.parentElement === null) {
        return null
    } else {
        return getOverflowParent(element.parentElement)
    }
}

function element() {
    return menuWrapper.value
}

defineExpose({
    element,
})
</script>