<template>
    <article class="projects">
        <SlickList class="task-column-wrapper" axis="x" style="display: flex;" v-model:list="columns">
            <SlickItem class="task-column-card card" v-for="(column, i) in columns" :key="column.title" :index="i">
                <h1 class="task-column-title">
                    {{ column.title }}
                </h1>

                <SlickList class="card" axis="y" group="tasks" v-model:list="column.task">
                    <SlickItem v-for="(task, j) in column.task" :key="task" :index="j"> 
                        <article class="task-card card shadowed">
                            <section class="task-card-notches">
                                <a>Notch</a>
                            </section>

                            <h1 class="task-card-title">{{ task }}</h1>
                        </article>
                    </SlickItem>
                </SlickList>
            </SlickItem>
        </SlickList>
    </article>
</template>

<script lang="ts">
import { SlickItem, SlickList } from 'vue-slicksort';

export default {
    components: {
        SlickItem,
        SlickList,
    },
    data() {
        return {
            columns: [
                {
                    title: "To Do",
                    task: [
                        "Design Grove Nodes",
                        "B",
                        "C"
                    ]
                },
                {
                    title: "Doing",
                    task: [
                        "D",
                        "E",
                        "F"
                    ]
                },
                {
                    title: "Done",
                    task: [
                        "G",
                        "H",
                        "I"
                    ]
                }
            ],
        };
    },
}
</script>

<style scoped lang="scss">
.projects {
    overflow-x: scroll;
}

.task-column {
    &-wrapper {
        margin: 1em;
        display: flex;
        gap: 1em;
        flex-direction: row;
    
        > * {
            flex-grow: 1;
        }
    }

    &-card {
        max-height: 500em;
    }

    &-title {
        margin: 10px;
        font-weight: 600;
        font-size: large;
        text-transform: uppercase;
    }
}

.task-card {
    min-height: 10em;
    min-width: 20em;
    margin: 1em;
    background-color: var(--colour-background);
    transition: (background-color, margin) 0.25s ease;
    cursor: grab;
    padding: 1em;

    &-title {
        text-align: left;
        text-transform: uppercase;
        font-weight: 600;
    }

    &-notches {
        display: flex;

        > * {
            padding: 3px;
            border-radius: 0.5em;
        }
    }

    &:hover {
        background-color: var(--colour-mint);
        color: var(--colour-accent);
        margin: 0.5em;
    }
}

.ghost-card {
    background-color: var(--colour-medium-gray);
    margin: 0.4em;
}
</style>