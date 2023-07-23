<script lang="ts">
    export let title: string = "";
</script>

<article class="card w-full h-full grow-1 bg-neutral shadow-xl">
    <span class="card-body">
        {#if title}
            <section class="card-title w-full">
                <h1>{title}</h1>
            </section>
        {/if}
        <slot/>
    </span>
</article>