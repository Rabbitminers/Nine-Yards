// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { 
    enabled: true 
  },
  css: ['@/styles/global.scss'],
  modules: ['@nuxtjs/color-mode']
})
