<script setup lang="ts">
import type { GraphicsInst } from 'vue3-pixi';
import { drawSquareCell } from '~/assets/typescript/utils';

const props = withDefaults(defineProps<{
    cellSize: number,
    color: number | string,
    x: number,
    y: number,
}>(), {})

function render(graphics: GraphicsInst) {
    drawSquareCell(graphics, 0, 0, props.cellSize, props.color)
}
</script>

<template>
    <Graphics
        @render="render"
        :x
        :y
    />
</template>
