<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
</script>

<div class="logo">
	<img src={favicon} alt="Fluix" class="logo-icon" />
	<span class="logo-text">Fluix Component</span>
</div>

<style>
	.logo {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		text-decoration: none;
		color: inherit;
	}

	.logo-icon {
		width: 2rem;
		height: 2rem;
		flex-shrink: 0;
	}

	.logo-text {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--foreground);
	}

	@media (max-width: 768px) {
		.logo-text {
			display: none;
		}
	}
</style>

