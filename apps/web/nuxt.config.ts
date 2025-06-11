// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2025-05-15',
    devtools: { enabled: true },
    modules: ['vue3-pixi-nuxt'],
    components: [
        {
            path: '~/components',
            pathPrefix: false,
        }
    ]
})
