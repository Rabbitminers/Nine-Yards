<template>
    <article class="account-icon-wrapper">
        <section class="account-icon-image">

        </section>

        <section class="account-icon-name">
            {{ name }}
        </section>
    </article>
</template>

<script lang="ts">
export default {
    name: "AccountIcon",
    props: {
        size: {
            type: String,
            default: "m",
            validator(value: string) {
                return ["xs", "s", "m", "l", "xl"].includes(value);
            },
        },
        name: {
            type: String
        }
    }
}
</script>

<style lang="scss">
.account-icon {
    &-wrapper {
        background-color: var(--color-background);
        border-radius: 10em;
    }
}
</style>