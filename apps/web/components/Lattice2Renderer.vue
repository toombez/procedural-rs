<script setup lang="ts" generic="S">
import type { GraphicsInst } from 'vue3-pixi';
import { drawSquareCell } from '~/assets/typescript/utils';

const props = withDefaults(defineProps<{
    states: Array<Array<S>>,
    cellSize?: number,
    x?: number,
    y?: number,
    colors: Map<S, number>
}>(), {
    cellSize: 1,
    x: 0,
    y: 0,
})

function render(graphics: GraphicsInst) {
    const colors = props.colors
    const cellSize = props.cellSize

    props
        .states
        .forEach((statesRow, y) => statesRow.forEach((state, x) => {
            drawSquareCell(graphics, x, y, cellSize, colors.get(state)!)
        }))
}
</script>

<template>
    <Graphics
        @render="render"
        :x
        :y
    />
</template>
