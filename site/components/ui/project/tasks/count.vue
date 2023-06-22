<template lang="pug">
article.item-status
    span.status-number {{  count }}
    span.status-type {{ group }}
</template>

<script lang="ts">
export default {
    name: "task-count",
    props: {
        group: {
            type: String,
            default: "Missing Name"
        },
        count: {
            type: Number,
            default: 0
        }
    }
}
</script>

<style scoped lang="scss">
.item-status {
    display: flex;
    flex-direction: column;
    margin-right: 16px;

    &:not(:last-child) .status-type:after {
        content: '';
        position: absolute;
        right: 8px;
        top: 50%;
        transform: translatey(-50%);
        width: 6px;
        height: 6px;
        border-radius: 50%;
        border: 1px solid var(--secondary-color);
    }
}

.status-number {
    font-size: 24px;
    line-height: 32px;
    font-weight: 700;
    color: var(--main-color);
}

.status-type {
    position: relative;
    padding-right: 24px;
    color: var(--secondary-color);
}

@media screen and (max-width: 980px) {
    .status-number, .status-type {
        font-size: 14px;
    }
    
    .status-type:after {
        width: 4px;
        height: 4px;
    }
    
    .item-status { 
        margin-right: 0; 
    }
}

@media screen and (max-width: 520px) {
    .status-number, .status-type {
        font-size: 10px;
    }

    .status-type {
        padding-right: 4px;
        
        &:after {
            display: none;
        }
    }
}

</style>