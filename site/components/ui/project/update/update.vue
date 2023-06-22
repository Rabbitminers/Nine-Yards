<template lang="pug">
article.update-box
    img(
        :src="icon" 
        alt='profile image'
    )
    section.update-content
      header.update-header
        span.name {{ username }}
        section.star-checkbox
          input#star-1(type='checkbox')
          label(for='star-1')
            star-icon
      p.update-line
        | {{ description }}
      p.update-line.time
        | {{ date }}
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import star from '~/components/icons/star.vue'

export default defineComponent({
    name: "update",
    components: {
        "star-icon": star, 
    },
    props: {
        user: {
            type: String,
            default: "Invalid user"
        },
        description: {
            type: String,
            default: "Missing description"
        },
        date: {
            type: String,
            default: "Some timestamp"
        }
    },
    data() {
        return {
            username: this.user, // Temp should be a user id
            icon: "https://avatars.githubusercontent.com/u/79579164?s=400&u=2d32d51075ebfd7e1ca5489b27952a7de15cdb49&v=4"
        }
    }
})
</script>

<style scoped lang="scss">

.update-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;

    .name {
        font-size: 16px;
        line-height: 24px;
        font-weight: 700;
        color: var(--main-color);
        margin: 0;
    }
}

.update-box {
    border-top: 1px solid var(--update-box-border);
    padding: 16px;
    display: flex;
    align-items: flex-start;
    width: 100%;

    &:hover {
        background-color: var(--update-box-hover);
        border-top-color: var(--link-color-hover);

        +.update-box {
            border-top-color: var(--link-color-hover);
        }
    }

    img {
        border-radius: 50%;
        object-fit: cover;
        width: 40px;
        height: 40px;
    }
}

.update-content {
    padding-left: 16px;
    width: 100%;
}

.update-line {
    font-size: 14px;
    line-height: 20px;
    margin: 8px 0;
    color: var(--secondary-color);
    opacity: 0.7;

    &.time {
        text-align: right;
        margin-bottom: 0;
    }
}

.star-checkbox {
    input {
        opacity: 0;
        position: absolute;
        width: 0;
        height: 0;
    }

    label {
        width: 24px;
        height: 24px;
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: pointer;
    }

    .dark & {
        color: var(--secondary-color);

        input:checked+label {
            color: var(--star);
        }
    }

    input:checked+label svg {
        fill: var(--star);
        transition: 0.2s;
    }
}
</style>