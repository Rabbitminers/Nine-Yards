<script>
	import Grazing from "../../components/icons/grazing.svelte";
</script>

<article class="card lg:card-side w-full h-max lg:h-[70vh] bg-neutral shadow-xl">
    <figure class="h-64 lg:h-full bg-base-200 p-6">
        <Grazing shadow="fill-base-100"/>
    </figure>
    <slot/>
</article>