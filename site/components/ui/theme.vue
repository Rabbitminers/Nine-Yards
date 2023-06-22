<template lang="pug">
button(
    class="mode-switch" 
    title="Switch Theme"
    @click="toggle_dark()"
)
    icons-moon(:class="{ active: is_dark }")
</template>

<script setup lang="ts">
import {toggle_dark, is_dark} from '~/api/dark'
</script>