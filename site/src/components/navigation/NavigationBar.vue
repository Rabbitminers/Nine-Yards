<template>
    <article class="navigation-bar full-width shadowed">
        <section class="navigation-bar-title"> 
            Nine yards
        </section>
    </article>
</template>

<script lang="ts">
export default {
    name: "NavigationBar"
}
</script>

<style lang="scss">
.navigation-bar {
    background-color: var(--colour-accent);
    padding: 2em;

    &-title {
        color: var(--colour-mint-dimmed);
        font-size: xx-large;
        text-transform: uppercase;
        font-weight: 800;
    }
}
</style>
