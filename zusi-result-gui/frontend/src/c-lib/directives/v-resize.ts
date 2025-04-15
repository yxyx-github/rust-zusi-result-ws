import type { DirectiveBinding } from 'vue'

export const vResize = {
    mounted(el: Element, binding: DirectiveBinding) {
        window.addEventListener('resize', binding.value)
    },
    unmounted(el: Element, binding: DirectiveBinding) {
        window.removeEventListener('resize', binding.value)
    },
}
