<script setup lang="ts">
import type { GraphicsInst } from 'vue3-pixi';
import { drawCellBorder } from '~/assets/typescript/utils';

const props = withDefaults(defineProps<{
    columns: number,
    rows: number,
    cellSize: number,
    x?: number,
    y?: number,
    borderColor: number | string
    borderSize: number
}>(), {
    x: 0,
    y: 0,
})

function render(graphics: GraphicsInst) {
    graphics.clear()
    for (let y = 0; y < props.rows; y++) {
        for (let x = 0; x < props.columns; x++) {
            drawCellBorder(graphics, x * props.cellSize, y * props.cellSize, props.borderSize, props.cellSize, props.borderColor)
        }
    }
}
</script>

<template>
    <Graphics
        @render="render"
        :x
        :y
    />
</template>
