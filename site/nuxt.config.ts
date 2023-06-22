// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  app: {
    head: {
      title: 'Nine Yards'
    }
  },
  devtools: { 
    enabled: true 
  },
  css: ['@/styles/global.scss'],
  modules: ['@nuxtjs/color-mode']
})
