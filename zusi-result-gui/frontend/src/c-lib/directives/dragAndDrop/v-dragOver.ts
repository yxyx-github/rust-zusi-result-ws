import type { DirectiveBinding } from 'vue'

export const vDragOver = {
    mounted(el: Element, binding: DirectiveBinding) {
        el.addEventListener('dragenter', () => setAttribute(el, binding.arg ?? 'class', binding.value))
        el.addEventListener('dragleave', () => removeAttribute(el, binding.arg ?? 'class', binding.value))
        el.addEventListener('drop', () => removeAttribute(el, binding.arg ?? 'class', binding.value))
        el.addEventListener('dragover', e => handleDragOver(e, el, binding.arg ?? 'class', binding.value))
    },
    updated(el: Element, binding: DirectiveBinding) {
        if (binding.value !== binding.oldValue) {
            removeAttribute(el, binding.arg ?? 'class', binding.oldValue)
        }
    },
    unmounted(el: Element, binding: DirectiveBinding) {
        el.removeEventListener('dragenter', () => setAttribute(el, binding.arg ?? 'class', binding.value))
        el.removeEventListener('dragleave', () => removeAttribute(el, binding.arg ?? 'class', binding.value))
        el.removeEventListener('drop', () => removeAttribute(el, binding.arg ?? 'class', binding.value))
        el.removeEventListener('dragover', e => handleDragOver(e, el, binding.arg ?? 'class', binding.value))
    },
}

function setAttribute(el: Element, attribute: string, value: any) {
    if (!el.getAttribute(attribute)?.includes(value)) {
        el.setAttribute(attribute, `${el.getAttribute(attribute)} ${value}`)
    }
}

function removeAttribute(el: Element, attribute: string, value: any) {
    const oldAttributeValue = el.getAttribute(attribute) ?? ''
    el.setAttribute(attribute, oldAttributeValue.replace(value, '').replace(/  +/g, ' ').trim())
}

function handleDragOver(e: Event, el: Element, attribute: string, value: any) {
    e.preventDefault()
    setAttribute(el, attribute, value)
}