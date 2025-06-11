<script setup lang="ts">
import { GameOfLifeAutomaton, GameOfLifeLattice, GameOfLifeRule, GameOfLifeState, Lattice2Point } from '@procedural/game_of_life';
import { BoundaryHandling } from '@procedural/toolkit'

const automatonSettings = reactive({
    populationSize: {
        columns: 50,
        rows: 50,
    },
    generationsCount: 500,
    boundaryHandling: BoundaryHandling.Wrap,
})

const renderSettings = reactive({
    cellSize: 10,
    deadColor: '#000000',
    aliveColor: '#FFFFFF',
    borderSize: 1,
    borderColor: '#888888',
})

const colorsMap = computed(() => new Map([
    [GameOfLifeState.Dead, renderSettings.deadColor],
    [GameOfLifeState.Alive, renderSettings.aliveColor],
]))

const population = ref<Array<GameOfLifeState>>([])

function evolve() {
    // const rule = new GameOfLifeRule(2, 4, 3)
    const rule = new GameOfLifeRule(2, 3, 3)
    const automaton = new GameOfLifeAutomaton(rule)
    const lattice = new GameOfLifeLattice()

    const width = automatonSettings.populationSize.columns
    const height = automatonSettings.populationSize.rows

    lattice.set_size(width, height)

    for (let y = 0; y < height; y++) {
        for (let x = 0; x < height; x++) {
            const point = new Lattice2Point(BigInt(x), BigInt(y))
            lattice.set_state(point, population.value[y * width + x])
        }
    }

    let generations = 0

    // TODO: fix performance
    const timer = setInterval(() => {
        generations += 1

        if (generations <= automatonSettings.generationsCount) {
            automaton.step(lattice)
            population.value = lattice.states

            console.log(population.value);
        } else {
            clearInterval(timer)
        }
    }, 100)
}

function generateRandomPopulation() {
    population.value = randomPopulation()
}

function fill(factory: () => GameOfLifeState) {
    const length = automatonSettings.populationSize.columns * automatonSettings.populationSize.rows

    return Array.from({ length }).map(factory)
}

function randomPopulation() {
    const randomFactory = () => Math.random() > 0.5 ? GameOfLifeState.Alive : GameOfLifeState.Dead
    return fill(randomFactory)
}

function clear() {
    population.value = []
}

function toggleState(x: number, y: number) {
    const idx = y * automatonSettings.populationSize.columns + x
    const state = population.value[idx]
    const newState = state === GameOfLifeState.Alive ? GameOfLifeState.Dead : GameOfLifeState.Alive

    population.value[idx] = newState
}

function fillDead() {
    return fill(() => GameOfLifeState.Dead)
}

function fillAlive() {
    return fill(() => GameOfLifeState.Alive)
}

const states = computed(() => {
    let _states: Array<Array<GameOfLifeState>> = []

    const width = automatonSettings.populationSize.columns
    const height = automatonSettings.populationSize.rows

    for (let y = 0; y < height; y++) {
        _states.push([])
        for (let x = 0; x < height; x++) {
            const state = population.value[y * width + x]
            _states[y][x] = state
        }
    }

    return _states
})

const hoveredX = ref()
const hoveredY = ref()
</script>

<template>
    <div>
        <button @click="generateRandomPopulation">random</button>
        <button @click="evolve">
            Запустить симуляцию
        </button>

        <hr>

        <Application
            :width="automatonSettings.populationSize.columns * renderSettings.cellSize"
            :height="automatonSettings.populationSize.rows * renderSettings.cellSize"
        >
            <Lattice2Renderer
                :states="states"
                :cell-size="renderSettings.cellSize"
                :colors="colorsMap"
            />

            <Lattice2Grid
                :border-color="renderSettings.borderColor"
                :border-size="renderSettings.borderSize"
                :cell-size="renderSettings.cellSize"
                :columns="automatonSettings.populationSize.columns"
                :rows="automatonSettings.populationSize.rows"
            />

            <Lattice2Pointer
                v-if="hoveredX !== null && hoveredY !== null"
                :cell-size="renderSettings.cellSize"
                :x="hoveredX * renderSettings.cellSize"
                :y="hoveredY * renderSettings.cellSize"
                :color="'#F5DA5199'"
            />

            <Lattice2Overlay
                :cell-size="renderSettings.cellSize"
                :columns="automatonSettings.populationSize.columns"
                :rows="automatonSettings.populationSize.rows"
                v-model:hovered-x="hoveredX"
                v-model:hovered-y="hoveredY"
                @click="toggleState"
            />
        </Application>
    </div>
</template>
