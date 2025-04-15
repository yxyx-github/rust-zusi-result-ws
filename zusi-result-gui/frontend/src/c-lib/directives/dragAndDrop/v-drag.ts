import type { DirectiveBinding } from 'vue'

export const vDrag = {
    mounted(el: Element, binding: DirectiveBinding) {
        setIdentifier(el, binding.value)
        el.setAttribute('draggable', 'true')
        el.addEventListener('dragstart', (e: Event | DragEvent) => handleDrag(e as DragEvent, binding.arg ?? 'dragAndDrop', (el as any).dataset['identifier']))
    },
    updated(el: Element, binding: DirectiveBinding) {
        if (binding.value !== binding.oldValue) {
            el.removeEventListener('dragstart', (e: Event | DragEvent) => handleDrag(e as DragEvent, binding.arg ?? 'dragAndDrop', (el as any).dataset['identifier']))
            setIdentifier(el, binding.value)
            el.addEventListener('dragstart', (e: Event | DragEvent) => handleDrag(e as DragEvent, binding.arg ?? 'dragAndDrop', (el as any).dataset['identifier']))
        }
    },
    unmounted(el: Element, binding: DirectiveBinding) {
        el.removeAttribute('draggable')
        el.removeEventListener('dragstart', (e: Event | DragEvent) => handleDrag(e as DragEvent, binding.arg ?? 'dragAndDrop', (el as any).dataset['identifier']))
    },
}

function handleDrag(e: DragEvent, name: string, identifier: any) {
    if (e.dataTransfer !== null) {
        e.dataTransfer.dropEffect = 'move'
        e.dataTransfer.effectAllowed = 'move'
        e.dataTransfer.setData(name, identifier)
    }
}

function setIdentifier(el: Element, identifier: any) {
    (el as any).dataset['identifier'] = identifier
}