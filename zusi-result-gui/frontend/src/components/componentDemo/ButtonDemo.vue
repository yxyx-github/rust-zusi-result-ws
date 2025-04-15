<template>
    <Button ref="testButton" @click="test" class="w-full">test</Button>
    <Button class="w-full" prependIcon="clipboard" appendIcon="clipboard">test</Button>
    <ButtonBar>
        <Button size="sm" compact title="circle" prependIcon="circle"/>
        <Button size="sm" compact label="circle" prependIcon="circle"/>
    </ButtonBar>
    <ButtonBar>
        <Button size="md" compact title="circle" prependIcon="circle"/>
        <Button size="md" compact label="circle" prependIcon="circle"/>
    </ButtonBar>
    <ButtonBar>
        <Button size="lg" compact title="circle" prependIcon="circle"/>
        <Button size="lg" compact label="circle" prependIcon="circle"/>
    </ButtonBar>
    <ButtonBar>
        <Button size="sm" title="circle" prependIcon="circle"/>
        <Button size="sm" label="circle" prependIcon="circle"/>
    </ButtonBar>
    <ButtonBar>
        <Button size="md" title="circle" prependIcon="circle"/>
        <Button size="md" label="circle" prependIcon="circle"/>
    </ButtonBar>
    <ButtonBar>
        <Button size="lg" title="circle" prependIcon="circle"/>
        <Button size="lg" label="circle" prependIcon="circle"/>
    </ButtonBar>
    <Row alignItems="start" :gap="1">
        <Button size="sm" title="circle" prependIcon="circle"/>
        <Button size="sm" label="circle" prependIcon="circle"/>
    </Row>
    <Row alignItems="start" :gap="1">
        <Button size="md" title="circle" prependIcon="circle"/>
        <Button size="md" label="circle" prependIcon="circle"/>
    </Row>
    <Row alignItems="start" :gap="1">
        <Button size="lg" title="circle" prependIcon="circle"/>
        <Button size="lg" label="circle" prependIcon="circle"/>
    </Row>
    <ButtonBar>
        <Button title="different title" prependIcon="circle">test<br/>test</Button>
        <Button severity="secondary" title="test" label="label"/>
        <Button severity="success" title="test" label="label"/>
        <Button severity="warning" title="test" label="label"/>
        <Button severity="error" title="test" label="label"/>
    </ButtonBar>
    <ButtonBar>
        <Button @click="toggleCompact" :compact="isCompact" title="different title" prependIcon="circle">test</Button>
        <Button @click="toggleCompact" :compact="isCompact" severity="secondary" title="test" label="label"/>
        <Button @click="toggleCompact" :compact="isCompact" severity="success" title="test" label="label"/>
        <Button @click="toggleCompact" :compact="isCompact" severity="warning" title="test" label="label"/>
        <Button @click="toggleCompact" :compact="isCompact" severity="error" title="test" label="label"/>
    </ButtonBar>
    <ButtonBar>
        <Button variant="ghost" prependIcon="circle">test</Button>
        <Button variant="ghost" severity="secondary">test</Button>
        <Button variant="ghost" severity="success">test</Button>
        <Button variant="ghost" severity="warning">test</Button>
        <Button variant="ghost" severity="error">test</Button>
    </ButtonBar>
    <ButtonBar>
        <Button @click="toggleCompact" :compact="isCompact" variant="ghost" prependIcon="circle">test</Button>
        <Button @click="toggleCompact" :compact="isCompact" variant="ghost" severity="secondary">test</Button>
        <Button @click="toggleCompact" :compact="isCompact" variant="ghost" severity="success">test</Button>
        <Button @click="toggleCompact" :compact="isCompact" variant="ghost" severity="warning">test</Button>
        <Button @click="toggleCompact" :compact="isCompact" variant="ghost" severity="error">test</Button>
    </ButtonBar>
    <ButtonBar>
        <Button compact variant="link" prependIcon="circle">test</Button>
        <Button compact variant="link" severity="secondary">test</Button>
        <Button compact variant="link" severity="success">test</Button>
        <Button compact variant="link" severity="warning">test</Button>
        <Button compact variant="link" severity="error">test</Button>
    </ButtonBar>
    <ButtonBar>
        <Button variant="link" prependIcon="circle">test</Button>
        <Button variant="link" severity="secondary">test</Button>
        <Button variant="link" severity="success">test</Button>
        <Button variant="link" severity="warning">test</Button>
        <Button variant="link" severity="error">test</Button>
    </ButtonBar>
    <ButtonGroup>
        <Button prependIcon="circle">test<br/>test</Button>
        <Button>test</Button>
        <Button>test</Button>
        <Button>test</Button>
        <Button severity="secondary">test</Button>
        <Button severity="success">test</Button>
        <Button severity="warning">test</Button>
        <Button severity="error">test</Button>
    </ButtonGroup>
    <ButtonGroup>
        <Button variant="ghost" prependIcon="circle">test</Button>
        <Button variant="ghost" severity="secondary">test</Button>
        <Button variant="ghost" severity="success">test</Button>
        <Button variant="ghost" severity="warning">test</Button>
        <Button variant="ghost" severity="error">test</Button>
    </ButtonGroup>
</template>

<script setup lang="ts">
import Button from '@/c-lib/components/core/controls/buttons/Button.vue'
import ButtonBar from '@/c-lib/components/core/controls/buttons/ButtonBar.vue'
import ButtonGroup from '@/c-lib/components/core/controls/buttons/ButtonGroup.vue'
import { ref } from 'vue'
import Row from '@/c-lib/components/core/layout/Row.vue'

const testButton = ref(null)

const isCompact = ref(true)
function toggleCompact() {
    isCompact.value = !isCompact.value
}

function test() {
    // @ts-ignore
    console.log(testButton.value?.element())
}
</script>