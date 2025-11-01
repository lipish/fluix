<script lang="ts">
	import { Blocks, Zap, Palette, Layout, BarChart3, SquareCode } from 'lucide-svelte';
	import { base } from '$app/paths';
</script>

<div class="banner">
	<h1>Fluix Component</h1>
	<div class="banner-description">
		Rust UI components for building fantastic cross-platform desktop applications by using GPUI 0.2.
	</div>
	<div class="actions">
		<a href="{base}/docs/getting-started" class="btn-primary">Get Started</a>
		<a href="{base}/docs/components">
			<Blocks />
			Components
		</a>
	</div>
</div>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 mb-12">
	<div class="feature-card">
		<h3>
			<div class="icon bg-green-500 dark:bg-green-700">
				<Blocks />
			</div>
			<div>46+ Components</div>
		</h3>
		<div>
			Comprehensive library of cross-platform desktop UI components for building feature-rich applications.
		</div>
	</div>
	<div class="feature-card">
		<h3>
			<div class="icon bg-blue-500 dark:bg-blue-700">
				<Zap />
			</div>
			<div>High Performance</div>
		</h3>
		<div>
			GPU-accelerated rendering powered by GPUI for smooth user experience.
		</div>
	</div>

	<div class="feature-card">
		<h3>
			<div class="icon bg-red-500 dark:bg-red-700">
				<Palette />
			</div>
			<div>Themeable</div>
		</h3>
		<div>
			Built-in theme system with flexible theme and style customization support.
		</div>
	</div>

	<div class="feature-card">
		<h3>
			<div class="icon bg-yellow-500 dark:bg-yellow-700">
				<Layout />
			</div>
			<div>Type Safe</div>
		</h3>
		<div>
			Leverage Rust's type system for compile-time type safety guarantees.
		</div>
	</div>

	<div class="feature-card">
		<h3>
			<div class="icon bg-pink-500 dark:bg-pink-700">
				<BarChart3 />
			</div>
			<div>Easy to Use</div>
		</h3>
		<div>
			Simple and consistent API design for quick onboarding.
		</div>
	</div>

	<div class="feature-card">
		<h3>
			<div class="icon bg-cyan-500 dark:bg-cyan-700">
				<SquareCode />
			</div>
			<div>Well Documented</div>
		</h3>
		<div>
			Comprehensive API documentation, tutorials, and example code.
		</div>
	</div>
</div>

<style>
	.banner {
		margin-bottom: 2rem;
		padding: 3rem 0;
		text-align: center;
	}

	@media (min-width: 1280px) {
		.banner {
			padding: 7.5rem 0;
		}
	}

	.banner h1 {
		font-size: 1.875rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		color: var(--primary);
	}

	@media (min-width: 1280px) {
		.banner h1 {
			font-size: 3rem;
		}
	}

	.banner-description {
		font-size: 1.125rem;
		color: var(--muted-foreground);
		margin-top: 0.5rem;
	}

	@media (min-width: 1280px) {
		.banner-description {
			font-size: 1.5rem;
			margin-top: 1rem;
		}
	}

	.actions {
		margin-top: 1rem;
		display: flex;
		gap: 1rem;
		justify-content: center;
		font-size: 0.875rem;
	}

	.actions a {
		display: flex;
		align-items: center;
		height: 2rem;
		border-radius: 0.375rem;
		gap: 0.375rem;
		padding: 0 0.75rem;
		text-decoration: none;
		background: var(--secondary);
		color: var(--secondary-foreground);
		transition: background-color 0.2s;
	}

	.actions a:hover {
		background: oklch(from var(--secondary) l c h / 0.7);
	}

	.actions a.btn-primary {
		background: var(--primary);
		color: var(--primary-foreground);
	}

	.actions a.btn-primary:hover {
		background: oklch(from var(--primary) l c h / 0.9);
	}

	.actions a :global(svg) {
		width: 1rem;
		height: 1rem;
	}

	.feature-card {
		display: flex;
		flex-direction: column;
		font-size: 0.875rem;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		border: 1px solid var(--border);
		border-radius: 0.5rem;
	}

	.feature-card h3 {
		margin: 0;
		padding: 0;
		font-size: 1.125rem;
		color: var(--primary);
		display: flex;
		gap: 0.75rem;
		align-items: center;
	}

	.feature-card h3 .icon {
		display: flex;
		height: 2.25rem;
		width: 2.25rem;
		align-items: center;
		justify-content: center;
		border-radius: 0.375rem;
		color: white;
	}

	.feature-card h3 .icon :global(svg) {
		width: 1.25rem;
		height: 1.25rem;
	}
</style>
