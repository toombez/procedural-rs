<script setup lang="ts">
import type { GraphicsInst } from 'vue3-pixi';
import { drawCellBorder } from '~/assets/typescript/utils';

const props = withDefaults(defineProps<{
    columns: number,
    cellSize: number,
    x?: number,
    y?: number,
    borderColor: number | string
    borderSize: number
}>(), {
    x: 0,
    y: 0,
})

</script>

<template>
    <Lattice2Grid
        :columns="columns"
        :cell-size="cellSize"
        :x="x"
        :y="y"
        :border-color="borderColor"
        :border-size="borderSize"
        :rows="1"
    />
</template>
