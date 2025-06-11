import type { GraphicsInst } from "vue3-pixi";

function drawSquareCell(graphics: GraphicsInst, x: number, y: number, cellSize: number, color: number) {
    graphics.beginFill(color)
    graphics.drawRect(x * cellSize, y * cellSize, cellSize, cellSize)
    graphics.endFill()
}

export { drawSquareCell }
