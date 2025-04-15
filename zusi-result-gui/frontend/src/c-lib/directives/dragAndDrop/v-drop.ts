import type { DirectiveBinding } from 'vue'

export const vDrop = {
    mounted(el: Element, binding: DirectiveBinding<(identifier: any) => any>) {
        el.addEventListener('drop', (e: Event | DragEvent) => handleDrop(e as DragEvent, binding.arg ?? 'dragAndDrop', binding.value))
    },
    unmounted(el: Element, binding: DirectiveBinding<(identifier: any) => any>) {
        el.removeEventListener('drop', (e: Event | DragEvent) => handleDrop(e as DragEvent, binding.arg ?? 'dragAndDrop', binding.value))
    },
}

function handleDrop(e: DragEvent, name: string, onDrop: (identifier: any) => any) {
    onDrop(e.dataTransfer?.getData(name))
}