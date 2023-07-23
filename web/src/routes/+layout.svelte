<script>
    import '../app.css'
    
	import Header from '../components/layout/header.svelte';
</script>

<html lang="html" class="max-w-7xl max-h-screen m-auto" data-theme="nineyardslight">

<Header/>

<main class="p-2">
    <slot />
</main>

</html>