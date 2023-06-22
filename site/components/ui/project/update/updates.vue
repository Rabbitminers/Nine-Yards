<template lang="pug">
article.updates-section 
    button.updates-close 
        svg(
            xmlns="http://www.w3.org/2000/svg" 
            width="24" height="24" 
            viewBox="0 0 24 24" 
            fill="none" 
            stroke="currentColor" 
            stroke-width="2" 
            stroke-linecap="round" 
            stroke-linejoin="round" 
            class="feather feather-x-circle"
        )
            circle (cx="12" cy="12" r="10")
            line (x1="15" y1="9" x2="9" y2="15")
            line (x1="9" y1="9" x2="15" y2="15")
    header.projects-section-header
        p Recent Updates
    section.updates
        update(
            v-for="update in updates"
            :key="update.description"
            :user="update.username"
            :date="update.date"
            :description="update.description"
        )
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import update from './update.vue'

export default defineComponent({
    name: "updates-tab",
    components: {
        "update": update
    },
    data() {
        return {
            updates: [
                {
                    "username": "Rabbitminers",
                    "description": "Completed task 'Add file handling to database' from group 'Doing'. ",
                    "date": "18, Jun"
                },
                {
                    "username": "Baxter",
                    "description": "Moved task 'nag for biscuits' to group 'Doing' from group 'To-Do' ",
                    "date": "18, Jun"
                },
                {
                    "username": "Rabbitminers",
                    "description": "Added new task 'Add file handling to database' to group 'Doing'",
                    "date": "17, Jun"
                },
                {
                    "username": "Sam",
                    "description": "Sam joined the project 'Nine-Yards' welcome!",
                    "date": "16, Jun"
                },
            ]
        }
    }
})
</script>

<style lang="scss">
.updates-section {
    flex-shrink: 0;
    padding-bottom: 32px;
    background-color: var(--projects-section);
    margin-left: 24px;
    flex: 1;
    width: 100%;
    border-radius: 30px;
    position: relative;
    overflow: auto;
    transition: all 300ms cubic-bezier(0.19, 1, 0.56, 1);

    .updates-close {
        position: absolute;
        top: 12px;
        right: 12px;
        z-index: 3;
        border: none;
        background-color: transparent;
        color: var(--main-color);
        display: none;
    }

    &.show {
        transform: translateX(0);
        opacity: 1;
        margin-left: 0;
    }

    .projects-section-header {
        position: sticky;
        top: 0;
        z-index: 1;
        padding: 32px 24px 0 24px;
        background-color: var(--projects-section);
    }
}

.updates-btn {
    border-radius: 4px 0 0 4px;
    position: absolute;
    right: 0;
    top: 58px;
    background-color: var(--update-btn);
    border: none;
    color: var(--main-color);
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 4px;
    display: none;
}

@media screen and (max-width: 880px) {
    .updates-section {
        transform: translateX(100%);
        position: absolute;
        opacity: 0;
        top: 0;
        z-index: 2;
        height: 100%;
        width: 100%;

        .updates-close {
            display: block;
        }
    }

    .updates-btn {
        display: flex;
    }
}

@media screen and (max-width: 520px) {
    .updates-btn {
        top: 48px;
    }
}
</style>