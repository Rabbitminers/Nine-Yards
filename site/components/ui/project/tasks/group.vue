<template lang="pug">
.article.task-group.columns
    span.task-group-title {{ title }}
    slot
</template>

<script lang="ts">
export default {
    name: "task-group",
    props: {
        title: {
            type: String,
            default: "Missing title"
        }
    }
}
</script>

<style lang="scss">
</style>