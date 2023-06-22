<template lang="pug">
article.projects-section
    header.projects-section-header
        p Some Project Name
        p.time June, 18
    section.projects-section-line
        section.projects-status
            task-count(
                v-for='group in groups' 
                :key='group.name' 
                :group='group.name' 
                :count='group.count'
            )
        section.view-actions
            button.view-btn.list-view(
                title='List View'
                @click="switchToListView"
                :class="{ active: isListView }"
            )
                list-icon
            button.view-btn.grid-view(
                title='Grid View'
                @click="switchToGridView"
                :class="{ active: isGridView }"
            )
                grid-icon
    section.project-boxes.jsGridView(
        :class="projectsClass"
    )
        task(
            v-for='task in tasks' 
            :key='task.name' 
            :name='task.name' 
            :group='task.group' 
            :progress='task.progress' 
            :primary='task.primary' 
            :accent='task.accent' 
            :assignees="['Rabbitminers', 'Baxter']"
        )
updates
</template>

<script lang="ts">
import Count from '~/components/ui/project/tasks/count.vue';
import Task from '~/components/ui/project/tasks/task.vue';
import Updates from '~/components/ui/project/update/updates.vue';
import List from '~/components/icons/list.vue';
import Grid from '~/components/icons/grid.vue';

export default {
    name: 'about-project',
    components: {
        'task-count': Count,
        'task': Task,
        'updates': Updates,
        'list-icon': List,
        'grid-icon': Grid
    },
    data() {
        return {
            isListView: false,
            isGridView: true,
            groups: [
                {
                    name: "Unstarted",
                    count: 45
                },
                {
                    name: "In Progress",
                    count: 23
                },
                {
                    name: "Completed",
                    count: 30
                }
            ],
            tasks: [
                {
                    name: "Add file handling",
                    group: "Doing",
                    progress: 30,
                    primary: "var(--colour-orange)",
                    accent: "var(--colour-orange-accent)"
                },
                {
                    name: "Make a cup of tea",
                    group: "Doing",
                    progress: 70,
                    primary: "var(--colour-purple)",
                    accent: "var(--colour-purple-accent)"
                },
                {
                    name: "Have a nap",
                    group: "To Do",
                    progress: 50,
                    primary: "var(--colour-pink)",
                    accent: "var(--colour-pink-accent)"
                },
                {
                    name: "Walk Baxter",
                    group: "Done",
                    progress: 100,
                    primary: "var(--colour-blue)",
                    accent: "var(--colour-blue-accent)"
                },
                {
                    name: "Give Baxter biscuits",
                    group: "To Do",
                    progress: 80,
                    primary: "var(--colour-green)",
                    accent: "var(--colour-green-accent)"
                },
                {
                    name: "Fix middleware",
                    group: "Doing",
                    progress: 40,
                    primary: "var(--colour-darker-blue)",
                    accent: "var(--colour-darker-blue-accent)"
                },
            ]
        }
    },
    computed: {
        projectsClass() {
            return {
                jsListView: this.isListView,
                jsGridView: this.isGridView
            };
        }
    },
    methods: {
        switchToListView() {
            this.isListView = true;
            this.isGridView = false;
        },
        switchToGridView() {
            this.isListView = false;
            this.isGridView = true;
        }
    }
}
</script>