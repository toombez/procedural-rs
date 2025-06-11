<script setup lang="ts">
import type { GraphicsInst } from 'vue3-pixi';
import { drawCellBorder } from '~/assets/typescript/utils';

const props = withDefaults(defineProps<{
    columns: number,
    rows: number,
    cellSize: number,
    x?: number,
    y?: number,
}>(), {
    x: 0,
    y: 0,
})

const emit = defineEmits<{
    click: [number, number]
}>()

const hoveredX = defineModel<number | null, number>('hoveredX', {
    default: null
})
const hoveredY = defineModel<number | null, number>('hoveredY', {
    default: null
})

const graphics = useTemplateRef('graphics')

onMounted(() => {
    const _graphics = graphics.value as unknown as GraphicsInst

    _graphics.on('mousemove', (event) => {
        const { x, y } = event.getLocalPosition(_graphics)
        const cellX = Math.floor(x / props.cellSize)
        const cellY = Math.floor(y / props.cellSize)

        hoveredX.value = cellX
        hoveredY.value = cellY
    })

    _graphics.on('mouseleave', () => {
        hoveredX.value = null
    })

    _graphics.on('click', (event) => {
        const { x, y } = event.getLocalPosition(_graphics)
        const cellX = Math.floor(x / props.cellSize)
        const cellY = Math.floor(y / props.cellSize)

        emit('click', cellX, cellY)
    })
})

function render(graphics: GraphicsInst) {
    // TODO: change how to create trigger area
    graphics.beginFill('white')
    graphics.drawRect(0, 0, props.columns * props.cellSize, props.rows * props.cellSize)
    graphics.endFill()
    graphics.alpha = 0
}
</script>

<template>
    <Graphics
        @render="render"
        ref="graphics"
    />
</template>
