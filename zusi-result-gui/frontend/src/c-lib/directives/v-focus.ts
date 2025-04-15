import type { DirectiveBinding } from 'vue'

export const vFocus = {
    mounted(el: any, binding: DirectiveBinding<boolean>) {
        if (binding.value) {
            el.focus()
        }
    },
}
