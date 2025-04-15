<template>
    <Column>
        <Row>
            <Card>This is a card.</Card>
            <Card>This is a card.</Card>
            <Card>This is a card.</Card>
        </Row>
        <Row>
            <Card>This is a card.</Card>
            <Card>This is a card.</Card>
            <Card>This is a card.</Card>
        </Row>
        <Row>
            <Card>This is a card.</Card>
            <Card>This is a card.</Card>
            <Card>This is a card.</Card>
        </Row>
    </Column>
</template>

<script setup lang="ts">
import Column from '@/c-lib/components/core/layout/Column.vue'
import Row from '@/c-lib/components/core/layout/Row.vue'
import ContentBox from '@/c-lib/components/core/layout/ContentBox.vue'
import Card from '@/c-lib/components/core/containers/Card.vue'
</script>