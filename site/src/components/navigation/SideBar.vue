<template>
    <article class="side-bar card shadowed">
        <router-link 
            class="side-bar-element" 
            v-for="link in links" 
            :key="link.destintation" :to="link.destintation"
        >
            <font-awesome-icon 
                class="side-bar-icon shadowed" 
                :icon="link.icon"
            />
        </router-link>
    </article>
</template>

<script lang="ts">
import { RouterLink } from 'vue-router';

export default {
    components: {
        "router-link": RouterLink
    },
    data() {
        return {
            links: [
                {
                    destintation: "/",
                    icon: "fa-house"
                },
                {
                    destintation: "/projects",
                    icon: "fa-square-check"
                },
                {
                    destintation: "/calendar",
                    icon: "fa-calendar-days"
                },
                {
                    destintation: "/settings",
                    icon: "fa-gear"
                }
            ]
        }
    }
}
</script>

<style lang="scss">
@import url(http://weloveiconfonts.com/api/?family=entypo);

.side-bar {
    margin: 1em;
    display: flex;
    gap: 1em;
    height: fit-content;
    flex-direction: column;
    justify-content: space-around;
    padding: 1em;

    &-icon {
        color: var(--colour-mint);
        background-color: var(--colour-background);
        padding: 0.5em;
        font-size: 1.45em;
        border-radius: 0.5em;

        transition: background-color 0.25s ease;

        &:hover {
            background-color: var(--colour-mint);
            color: var(--colour-accent);
        }
    }
}
</style>