<template lang="pug">
.project-box-wrapper(:class='{expanded: expanded}')
  .project-box(:style='background_colour')
    .project-box-header
      span {{ date }}
      .more-wrapper(@click='expand_view()')
        button.project-btn-more
          more-icon
    .project-box-content-header
      p.box-content-header.ellipsis {{ name }}
      p.box-content-subheader {{ group }}
    .box-progress-wrapper
      p.box-progress-header Progress
      .box-progress-bar
        span.box-progress(:style='bar_style')
      p.box-progress-percentage {{progress}}%
    .project-box-footer
      .participants
        img(v-for='assignee in assignees' :key='assignee' :src='icon' alt='participant')
        button.add-participant(:style='primary_colour')
          plus-icon
      .days-left(:style='primary_colour')
        | 2 Days Left
.overlay.full(v-if='expanded')
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { to_full_date } from '~/api/helpers/dates';
import More from '~/components/icons/more.vue'
import Plus from '~/components/icons/plus.vue';

export default defineComponent({
    name: "task",
    components: {
        'more-icon': More,
        'plus-icon': Plus
    },
    props: {
        name: {
            type: String,
            default: "Missing Title"
        },
        group: {
            String,
            default: "Missing Task Group"
        },
        timestamp: {
            type: Number,
            default: 0
        },
        assignees: {
            type: Array as () => string[],
            default: () => []
        },
        progress: {
            type: Number,
            default: 30,
            validator: (value: number ) => {
                return value >= 0 && value <= 100
            }
        },
        primary: {
            type: String,
            default: "var(--colour-orange)"
        },
        accent: {
            type: String,
            default: "var(--colour-orange-accent)"
        }
    },
    methods: {
        expand_view() {
            this.expanded = !this.expanded;
        }
    },
    data() {
        return {
            icon: "https://avatars.githubusercontent.com/u/79579164?s=400&u=2d32d51075ebfd7e1ca5489b27952a7de15cdb49&v=4",
            expanded: false,
            date: to_full_date(this.timestamp),
            bar_style: `width: ${this.progress}%; background-color: ${this.accent};`,
            primary_colour: `color: ${this.accent};`,
            background_colour: `background-color: ${this.primary};`, 
        }
    },
})
</script>

<style lang="scss">
@import '@/styles/layout.scss';

.expanded {
    @extend .fixed-center;
    z-index: 10;
}

.overlay {
    @extend .fixed-center;
    z-index: 5;
    background-color: #232323aa;
}
</style>