<script setup lang="ts" generic="S">
const props = withDefaults(defineProps<{
    states: Array<S>,
    cellSize?: number,
    x?: number,
    y?: number,
    colors: Map<S, number>
}>(), {
    cellSize: 1,
    x: 0,
    y: 0,
})
</script>

<template>
    <Lattice2Renderer
        :states="[states]"
        :cell-size
        :colors
        :x
        :y
    />
</template>
