import type { GraphicsInst } from "vue3-pixi";

function drawSquareCell(graphics: GraphicsInst, x: number, y: number, cellSize: number, color: number | string) {
    graphics.beginFill(color)
    graphics.drawRect(x, y, cellSize, cellSize)
    graphics.endFill()
}

function drawCellBorder(graphics: GraphicsInst, x: number, y: number, borderSize: number, cellSize: number, color: number | string) {
    drawSquareCell(graphics, x, y, cellSize, color)

    graphics.beginHole()
    graphics.drawRect(x + borderSize, y + borderSize, cellSize - borderSize * 2, cellSize - borderSize * 2)
    graphics.endHole()
}

export { drawSquareCell, drawCellBorder }
