<template lang="pug">
article.projects-section
    header.projects-section-header
        p Some Project Name
        p.time June, 18
    section.projects-groups.rows
        task-group(
            v-for="group in groups"
            :key="group.name"
            :title="group.name"
        )
            task(
                v-for='task in group.tasks' 
                :key='task.name' 
                :name='task.name' 
                :group='group.name' 
                :progress='task.progress' 
                :primary='task.primary' 
                :accent='task.accent' 
                :assignees="['Rabbitminers', 'Baxter']"
            )
</template>

<script lang="ts">
import Task from '~/components/ui/project/tasks/task.vue';
import Group from '~/components/ui/project/tasks/group.vue';

export default {
    name: 'task-board',
    components: {
        'task': Task,
        'task-group': Group
    },
    data() {
        return {
            groups: [
                {
                    name: "To-Do",
                    tasks: [
                        {
                            name: "Add file handling",
                            group: "Doing",
                            progress: 30,
                            primary: "var(--colour-orange)",
                            accent: "var(--colour-orange-accent)"
                        },
                        {
                            name: "Add file handling",
                            group: "Doing",
                            progress: 30,
                            primary: "var(--colour-green)",
                            accent: "var(--colour-green-accent)"
                        },
                    ]
                },
                {
                    name: "Doing",
                    tasks: [
                        {
                            name: "Add file handling",
                            group: "Doing",
                            progress: 30,
                            primary: "var(--colour-blue)",
                            accent: "var(--colour-blue-accent)"
                        },
                    ]
                },
                {
                    name: "Done",
                    tasks: [
                        {
                            name: "Add file handling",
                            group: "Doing",
                            progress: 30,
                            primary: "var(--colour-pink)",
                            accent: "var(--colour-pink-accent)"
                        },
                    ]
                },
            ]
        }
    }
}
</script>

<style lang="scss">
.projects-groups {
    justify-content: space-around;
}
</style>