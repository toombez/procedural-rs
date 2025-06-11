<script setup lang="ts">
import { BoundaryHandling } from '@procedural/toolkit'
import { WolframCodeAutomaton, WolframCodeLattice, WolframCodeRule, WolframCodeState } from '@procedural/wolfram_code'

const automatonSettings = reactive({
    populationSize: 50,
    generationsCount: 50,
    ruleNumber: 99,
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
    [WolframCodeState.Dead, renderSettings.deadColor],
    [WolframCodeState.Alive, renderSettings.aliveColor],
]))

const initialPopulation = ref<Array<WolframCodeState>>([])
const history = ref<Array<Array<WolframCodeState>>>([])
const hoveredX = ref()
const hoveredY = ref()

function evolve() {
    const rule = new WolframCodeRule(automatonSettings.ruleNumber)
    const automaton = new WolframCodeAutomaton(rule)

    const lattice = new WolframCodeLattice(initialPopulation.value)

    lattice.set_boundary_handing(automatonSettings.boundaryHandling)

    history.value = []

    for (let generation = 0; generation < automatonSettings.generationsCount; generation++) {
        automaton.step(lattice)
        history.value.push(lattice.states)
    }
}

function randomPopulation() {
    return Array
        .from({ length: automatonSettings.populationSize })
        .map(() => Math.random() > 0.5 ? WolframCodeState.Alive : WolframCodeState.Dead)
}

watch(initialPopulation, () => {
    history.value = []
})

function generateRandomPopulation() {
    initialPopulation.value = randomPopulation()
}

function clear() {
    history.value = []
}

function toggleState(x: number, y: number) {
    if (y === 0) {
        const state = initialPopulation.value[x]
        const newState = state === WolframCodeState.Alive ? WolframCodeState.Dead : WolframCodeState.Alive

        initialPopulation.value[x] = newState
    }
}

function fillDead() {
    initialPopulation.value = Array
        .from({ length: automatonSettings.populationSize })
        .map(() => WolframCodeState.Dead)
}

function fillAlive() {
    initialPopulation.value = Array
        .from({ length: automatonSettings.populationSize })
        .map(() => WolframCodeState.Alive)
}
</script>

<template>
    <div>
        <form @submit.prevent="evolve">
            <fieldset>
                <legend>
                    Настройки автомата
                </legend>

                <label for="rule-field">
                    Правило
                </label>

                <input
                    id="rule-field"
                    min="0"
                    max="255"
                    type="number"
                    v-model.number="automatonSettings.ruleNumber"
                >

                <label for="population-size-field">
                    Размер популяции
                </label>
                <input
                    id="population-size-field"
                    min="0"
                    type="number"
                    v-model.number="automatonSettings.populationSize"
                >

                <label for="generations-count-field">
                    Поличество поколений
                </label>

                <input
                    id="generations-count-field"
                    min="0"
                    type="number"
                    v-model.number="automatonSettings.generationsCount"
                >

                <label for="boundary-handling-field">
                    Обработка границ
                </label>

                <select v-model="automatonSettings.boundaryHandling" id="boundary-handling-field">
                    <option :value="BoundaryHandling.Default">
                        Default
                    </option>
                    <option :value="BoundaryHandling.Clamp">
                        Clamp
                    </option>
                    <option :value="BoundaryHandling.Wrap">
                        Wrap
                    </option>
                </select>
            </fieldset>

            <fieldset>
                <legend>
                    Настройки отображения
                </legend>

                <label for="cell-size-field">
                    Размер клетки
                </label>

                <input
                    id="cell-size-field"
                    min="0"
                    type="number"
                    v-model.number="renderSettings.cellSize"
                >

                <label for="hovered-cell-border-field">
                    Размер обводки для наведении
                </label>

                <input
                    id="hovered-cell-border-field"
                    min="0"
                    type="number"
                    v-model.number="renderSettings.borderSize"
                >

                <label for="alive-color-field">
                    Цвет сетки
                </label>

                <input
                    id="alive-color-field"
                    type="color"
                    v-model="renderSettings.borderColor"
                >

                <label for="alive-color-field">
                    Цвет живой клетки
                </label>

                <input
                    id="alive-color-field"
                    type="color"
                    v-model="renderSettings.aliveColor"
                >

                <label for="dead-color-field">
                    Цвет мертвой клетки
                </label>

                <input
                    id="dead-color-field"
                    type="color"
                    v-model="renderSettings.deadColor"
                >
            </fieldset>

            <fieldset>
                <legend>
                    Управление
                </legend>

                <button type="button" @click="generateRandomPopulation">
                    Случайная популяция
                </button>

                <button type="submit">
                    Запуск симуляции
                </button>

                <button type="button" @click="clear">
                    Очистка
                </button>

                <button type="button" @click="fillDead">
                    Заполнить мертвыми клетками
                </button>

                <button type="button" @click="fillAlive">
                    Заполнить живыми клетками
                </button>
            </fieldset>
        </form>

        <hr>

        <Application
            :width="automatonSettings.populationSize * renderSettings.cellSize"
            :height="automatonSettings.generationsCount * renderSettings.cellSize + renderSettings.cellSize"
        >
            <Lattice2Renderer
                :states="[initialPopulation]"
                :cell-size="renderSettings.cellSize"
                :colors="colorsMap"
            />
            <Lattice2Renderer
                :y="renderSettings.cellSize"
                :states="history"
                :cell-size="renderSettings.cellSize"
                :colors="colorsMap"
            />

            <Lattice2Pointer
                v-if="hoveredX !== null && hoveredY !== null"
                :cell-size="renderSettings.cellSize"
                :color="'#F5DA5199'"
                :x="hoveredX * renderSettings.cellSize"
                :y="hoveredY * renderSettings.cellSize"
            />

            <Lattice2Overlay
                :cell-size="renderSettings.cellSize"
                :columns="automatonSettings.populationSize"
                :rows="automatonSettings.generationsCount + 1"
                v-model:hovered-x="hoveredX"
                v-model:hovered-y="hoveredY"
                @click="toggleState"
            />

            <Lattice2Grid
                :columns="automatonSettings.populationSize"
                :rows="automatonSettings.generationsCount + 1"
                :border-color="renderSettings.borderColor"
                :border-size="renderSettings.borderSize"
                :cell-size="renderSettings.cellSize"
            />
        </Application>
    </div>
</template>
