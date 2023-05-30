<template>
    <article class="task-column card shadowed">
        <section class="task-column-title">
            {{ title }}
        </section>

        <SlickList class="task-column-tasks">
            <slot/>
        </SlickList>
    </article>
</template>

<script lang="ts">
import { SlickList } from 'vue-slicksort';

export default {
    components: {
        SlickList
    },
    props: {
        title: {
            type: String,
            default: "Missing Title"
        }
    }
}
</script>

<style lang="scss" scoped>
.task-column {
    height: min-content;
    transition: all 2s ease;

    &-title {
        text-transform: uppercase;
        text-align: center;
        font-weight: 600;
    }

    &-tasks {
        
    }
}
</style>