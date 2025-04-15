<template>
    <slot name="trigger" v-if="props.trigger">
        <Button :="props" :label="props.label === '' ? props.title : props.label" @click="show = true"/>
    </slot>
    <teleport to="#modals" v-if="show">
        <ScreenOverlay/>
        <div class="fixed top-0 left-0 w-full h-full" @keydown.esc="props.closable && (show = false)">
            <div class="relative w-full h-full flex flex-row flex-nowrap justify-center items-center">
                <div :class="`relative max-w-full max-h-full flex flex-col justify-start
                            ${props.threshold === 'mb' && !fullScreen ? '' : 'mb:w-full mb:h-full'}
                            ${fullScreen ? '' : `${props.threshold}:w-auto ${props.threshold}:h-auto`}
                            bg-bg shadow-md border border-secondary-200 rounded-sm
                    `" :="$attrs">
                    <Row alignItems="center" justifyItems="between" :grow="false" class="bg-bg rounded-tr-sm rounded-tl-sm z-10">
                        <Row :gap="0">
                            <Button v-if="props.resizable" severity="secondary" variant="ghost" prependIcon="x" class="invisible" inert/>
                            <Button v-if="props.closable" severity="secondary" variant="ghost" prependIcon="x" class="invisible" inert/>
                        </Row>
                        <div class="py-1.5 flex-grow text-center">{{ props.title }}</div>
                        <Row :gap="0">
                            <Button
                                    v-if="props.resizable"
                                    severity="secondary"
                                    variant="ghost"
                                    :title="$t(fullScreen ? 'c.minimize' : 'c.maximize')"
                                    :prependIcon="fullScreen ? 'minimize-2' : 'maximize-2'"
                                    @click="fullScreen = !fullScreen"
                            />
                            <Button
                                    v-if="props.closable"
                                    severity="secondary"
                                    variant="ghost"
                                    :title="$t('c.close')"
                                    prependIcon="x"
                                    @click="show = false"
                            />
                        </Row>
                    </Row>
                    <Column :is="ContentBox" :grow="true" class="rounded-br-sm rounded-bl-sm overflow-auto z-0">
                        <slot :close="() => show = false"/>
                    </Column>
                </div>
            </div>
        </div>
    </teleport>
</template>

<script setup lang="ts">
import ScreenOverlay from '@/c-lib/components/core/containers/ScreenOverlay.vue'
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import Column from '@/c-lib/components/core/layout/Column.vue'
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import type { ButtonProps } from '@/c-lib/types/button.ts'
import { defaultButtonProps } from '@/c-lib/types/defaults/button'
import { defaultModalProps } from '@/c-lib/types/defaults/modal'
import type { ModalProps } from '@/c-lib/types/modal.ts'
import type { SizeProps } from '@/c-lib/types/size.ts'
import { defaultSizeProps } from '@/c-lib/types/defaults/size'

defineOptions({
    inheritAttrs: false,
})

const emit = defineEmits(['close'])

const props = withDefaults(defineProps<ButtonProps & SizeProps & ModalProps>(), {
    ...defaultButtonProps,
    ...defaultSizeProps,
    ...defaultModalProps,
})

const show = defineModel<boolean>('show', {
    default: false,
    set: value => {
        if (!value) {
            emit('close')
        }
        return value
    },
})

const fullScreen = defineModel<boolean>('fullScreen', { default: false })
</script>