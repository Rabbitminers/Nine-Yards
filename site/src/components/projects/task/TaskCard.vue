<template>
    <SlickItem class="task-card card shadowed">
        <a class="task-card-title">{{title}}</a>
        <slot/>
    </SlickItem>
</template>

<script lang="ts">
import { SlickItem } from 'vue-slicksort';

export default {
    name: "TaskCard",
    components: {
        SlickItem
    },
    props: {
        title: {
            type: String,
            default: "Missing title"
        }
    }
}
</script>

<style lang="scss">
.task-card {
    padding: 2em;
    min-width: 20em;
    margin: 1em;
    background-color: var(--colour-background);
    transition: all 0.25s ease;

    &-title {
        text-align: left;
        font-weight: 500;
    }

    &:hover {
        background-color: var(--colour-mint);
        margin: 0.5em;
    }
}
</style>