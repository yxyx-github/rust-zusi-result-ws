import { ref } from 'vue'
import type { MenuItem, MenuItemPath } from '@/c-lib/types/menu.ts'

export function useMenuKeyboardManager(items: MenuItem[], getParentElement: () => HTMLElement, onLeave: () => void = () => {}) {
    function lastIndex(path: MenuItemPath, index: number = path.length - 1): number {
        let currentMenuItems = items
        for (let i = 0; i < index; i++) {
            const children = currentMenuItems[path[i]].children
            if (children === undefined) {
                return -1
            } else {
                currentMenuItems = children as MenuItem[]
            }
        }
        return currentMenuItems.length - 1
    }

    const selected = ref<MenuItemPath>([])

    function onUp() {
        if (selected.value.length === 0) {
            selected.value.push(lastIndex(selected.value))
        } else {
            const index = selected.value.length - 1
            let newValue = selected.value[index] - 1
            if (newValue < 0) {
                newValue = lastIndex(selected.value, index)
            }
            selected.value[index] = newValue
        }
    }

    function onRight() {
        if (selected.value.length === 0) {
            selected.value.push(0)
        } else {
            if (lastIndex(selected.value, selected.value.length) !== -1) {
                selected.value.push(0)
            }
        }
    }

    function onDown() {
        if (selected.value.length === 0) {
            selected.value.push(0)
        } else {
            const index = selected.value.length - 1
            let newValue = selected.value[index] + 1
            if (newValue > lastIndex(selected.value, index)) {
                newValue = 0
            }
            selected.value[index] = newValue
        }
    }

    function onLeft() {
        if (selected.value.length === 0) {
            selected.value.push(lastIndex(selected.value))
        } else {
            selected.value.pop()
            if (selected.value.length === 0) {
                onLeave()
            }
        }
    }

    function onEsc() {
        selected.value = []
        onLeave()
    }

    function onUpdateFocus({ path, value }: { path: MenuItemPath, value: boolean }) {
        if (value) {
            selected.value = path
        } else if (!getParentElement().contains(document.activeElement)) {
            selected.value = []
        }
    }

    return {
        selected,
        onUp,
        onRight,
        onDown,
        onLeft,
        onEsc,
        onUpdateFocus,
    }
}
