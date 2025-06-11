<script setup lang="ts">
import initToolkit from '@procedural/toolkit'
import initWolfram from '@procedural/wolfram_code'
import initGameOfLife from '@procedural/game_of_life'

onBeforeMount(async () => {
    await initToolkit()
    await initWolfram()
    await initGameOfLife()
})
</script>

<template>
    <div>
        <NuxtLink to="/">
            Index
        </NuxtLink>
        <NuxtLink to="/wolfram-code">
            Wolfram
        </NuxtLink>
        <NuxtLink to="/game-of-life">
            Game of life
        </NuxtLink>
        <NuxtPage />
    </div>
</template>
