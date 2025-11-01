<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from '$lib/components/Navbar.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Fluix Component - Rust UI Component Library</title>
</svelte:head>

<Navbar />

<main class="main-content">
	{@render children()}
</main>

<style>
	.main-content {
		min-height: calc(100vh - 4rem);
		padding: 2rem 1.5rem;
		max-width: 1280px;
		margin: 0 auto;
		width: 100%;
	}

	@media (max-width: 768px) {
		.main-content {
			padding: 1.5rem 1rem;
		}
	}
</style>
