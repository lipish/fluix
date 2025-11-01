<script lang="ts">
	import { Search, ChevronDown, ExternalLink, Github, MoreHorizontal } from 'lucide-svelte';
	import { onMount } from 'svelte';
	
	let searchValue = $state('');
	let resourcesOpen = $state(false);
	let githubStars = $state<number | null>(null);
	let searchInput: HTMLInputElement;

	onMount(async () => {
		// Fetch GitHub stars (with fallback)
		try {
			const response = await fetch('https://api.github.com/repos/lipish/fluix');
			if (response.ok) {
				const data = await response.json();
				githubStars = data.stargazers_count;
			}
		} catch (error) {
			console.error('Failed to fetch GitHub stars:', error);
			githubStars = null;
		}
	});

	function handleSearch(e: KeyboardEvent) {
		if (e.key === 'Enter' && searchValue.trim()) {
			// Handle search navigation
			console.log('Search:', searchValue);
			// You can implement search navigation here
		}
	}

	function handleCmdK(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			searchInput?.focus();
		}
	}
</script>

<svelte:window onkeydown={handleCmdK} />

<nav class="navbar">
	<div class="nav-container">
		<!-- Logo -->
		<a href="/" class="logo-link">
			<div class="logo-icon-wrapper">
				<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
					<rect x="3" y="3" width="8" height="8" fill="currentColor" stroke="none"/>
					<rect x="13" y="3" width="8" height="8" stroke="currentColor" fill="none"/>
					<rect x="3" y="13" width="8" height="8" stroke="currentColor" fill="none"/>
					<rect x="13" y="13" width="8" height="8" fill="currentColor" stroke="none"/>
				</svg>
			</div>
			<span class="logo-text">Fluix Component</span>
		</a>

		<!-- Search Bar -->
		<div class="search-container">
			<div class="search-input-wrapper">
				<Search class="search-icon" />
				<input
					type="text"
					class="search-input"
					placeholder="Search"
					bind:value={searchValue}
					onkeydown={handleSearch}
					bind:this={searchInput}
				/>
				<div class="search-shortcut">
					<span class="shortcut-key">⌘</span>
					<span class="shortcut-key">K</span>
				</div>
			</div>
		</div>

		<!-- Navigation Links -->
		<div class="nav-links">
			<a href="/" class="nav-link">Home</a>
			<a href="/docs/getting-started" class="nav-link">Getting Started</a>
			<a href="/docs/components" class="nav-link">Components</a>
			<a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="nav-link">
				API Doc
				<ExternalLink class="external-icon" />
			</a>
			<div class="nav-link resources-dropdown" onmouseenter={() => resourcesOpen = true} onmouseleave={() => resourcesOpen = false}>
				<span>Resources</span>
				<ChevronDown class="chevron-icon" />
				{#if resourcesOpen}
					<div class="dropdown-menu">
						<a href="/docs/tutorials" class="dropdown-item">Tutorials</a>
						<a href="/docs/faq" class="dropdown-item">FAQ</a>
						<a href="https://github.com/lipish/fluix/releases" target="_blank" rel="noopener noreferrer" class="dropdown-item">
							Releases
							<ExternalLink class="external-icon-small" />
						</a>
					</div>
				{/if}
			</div>
		</div>

		<!-- GitHub Link -->
		<a
			href="https://github.com/lipish/fluix"
			target="_blank"
			rel="noopener noreferrer"
			class="github-link"
		>
			<Github class="github-icon" />
			{#if githubStars !== null}
				<span class="github-stars">{githubStars >= 1000 ? (githubStars / 1000).toFixed(1) + 'k' : githubStars}</span>
			{:else}
				<span class="github-stars">GitHub</span>
			{/if}
		</a>

		<!-- More Options -->
		<button class="more-button" aria-label="More options">
			<MoreHorizontal />
		</button>
	</div>
</nav>

<style>
	.navbar {
		position: sticky;
		top: 0;
		z-index: 50;
		background: var(--background);
		border-bottom: 1px solid var(--border);
		padding: 0;
		backdrop-filter: blur(8px);
		background: oklch(from var(--background) l c h / 0.95);
	}

	.nav-container {
		max-width: 1280px;
		margin: 0 auto;
		padding: 0 1.5rem;
		display: flex;
		align-items: center;
		gap: 1.5rem;
		height: 4rem;
	}

	.logo-link {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		text-decoration: none;
		color: inherit;
		flex-shrink: 0;
	}

	.logo-icon-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2rem;
		height: 2rem;
		flex-shrink: 0;
		color: var(--foreground);
	}

	.logo-icon-wrapper :global(svg) {
		width: 100%;
		height: 100%;
	}

	.logo-icon {
		width: 2rem;
		height: 2rem;
		flex-shrink: 0;
	}

	.logo-text {
		font-size: 1rem;
		font-weight: 700;
		color: var(--foreground);
		white-space: nowrap;
	}

	.search-container {
		flex: 1;
		max-width: 500px;
		margin: 0 auto;
		min-width: 200px;
	}

	.search-input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
		background: var(--secondary);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 0.5rem 0.75rem;
		gap: 0.5rem;
		transition: border-color 0.2s, box-shadow 0.2s;
	}

	.search-input-wrapper:focus-within {
		border-color: var(--ring);
		outline: 2px solid transparent;
		outline-offset: 2px;
		box-shadow: 0 0 0 2px oklch(from var(--ring) l c h / 0.2);
	}

	.search-icon {
		width: 1rem;
		height: 1rem;
		color: var(--muted-foreground);
		flex-shrink: 0;
	}

	.search-input {
		flex: 1;
		border: none;
		background: transparent;
		color: var(--foreground);
		font-size: 0.875rem;
		outline: none;
	}

	.search-input::placeholder {
		color: var(--muted-foreground);
	}

	.search-shortcut {
		display: flex;
		gap: 0.125rem;
		flex-shrink: 0;
	}

	.shortcut-key {
		padding: 0.125rem 0.375rem;
		font-size: 0.6875rem;
		background: var(--background);
		border: 1px solid var(--border);
		border-radius: 0.25rem;
		color: var(--muted-foreground);
		font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace;
		font-weight: 500;
		line-height: 1.2;
	}

	.nav-links {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		margin-left: auto;
		flex-shrink: 0;
	}

	.nav-link {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		text-decoration: none;
		color: var(--foreground);
		font-size: 0.875rem;
		font-weight: 400;
		padding: 0.5rem 0.75rem;
		border-radius: 0.375rem;
		transition: background-color 0.2s;
		white-space: nowrap;
	}

	.nav-link:hover {
		background: var(--secondary);
	}

	.external-icon {
		width: 0.625rem;
		height: 0.625rem;
		color: var(--muted-foreground);
		opacity: 0.5;
	}

	.resources-dropdown {
		position: relative;
		cursor: pointer;
	}

	.chevron-icon {
		width: 0.625rem;
		height: 0.625rem;
		color: var(--muted-foreground);
		opacity: 0.5;
		transition: transform 0.2s;
	}

	.resources-dropdown:hover .chevron-icon {
		transform: rotate(180deg);
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 0.5rem;
		background: var(--popover);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 0.5rem;
		min-width: 12rem;
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		border-radius: 0.375rem;
		text-decoration: none;
		color: var(--foreground);
		font-size: 0.875rem;
		transition: background-color 0.2s;
	}

	.dropdown-item:hover {
		background: var(--secondary);
	}

	.external-icon-small {
		width: 0.75rem;
		height: 0.75rem;
		color: var(--muted-foreground);
	}

	.github-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		text-decoration: none;
		color: var(--foreground);
		padding: 0.5rem 0.75rem;
		border-radius: 0.375rem;
		transition: background-color 0.2s;
		flex-shrink: 0;
	}

	.github-link:hover {
		background: var(--secondary);
	}

	.github-icon {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--foreground);
	}

	.github-stars {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--foreground);
	}

	.more-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2rem;
		height: 2rem;
		border: none;
		background: transparent;
		color: var(--foreground);
		border-radius: 0.375rem;
		cursor: pointer;
		transition: background-color 0.2s;
		flex-shrink: 0;
	}

	.more-button:hover {
		background: var(--secondary);
	}

	.more-button :global(svg) {
		width: 1.25rem;
		height: 1.25rem;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.search-container {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.logo-text {
			display: none;
		}

		.nav-links {
			gap: 0.75rem;
		}

		.nav-link {
			padding: 0.5rem;
			font-size: 0.8125rem;
		}

		.github-stars {
			display: none;
		}
	}

	@media (max-width: 640px) {
		.nav-container {
			padding: 0 1rem;
			gap: 0.75rem;
		}

		.nav-links {
			gap: 0.5rem;
		}

		.nav-link {
			padding: 0.375rem 0.5rem;
			font-size: 0.75rem;
		}
	}
</style>
