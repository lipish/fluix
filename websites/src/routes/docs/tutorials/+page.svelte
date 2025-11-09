<script lang="ts">
	// Tutorials page
	import { base } from '$app/paths';
</script>

<div class="page-container">
	<h1>Tutorials</h1>
	
	<div class="tutorials-grid">
		<div class="tutorial-card">
			<h2>Components Overview</h2>
			<p>Complete overview of all Fluix components with examples and installation guides.</p>
			<a href="{base}/docs/components" class="link">View Components →</a>
		</div>
		
		<div class="tutorial-card">
			<h2>Working with Components</h2>
			<p>Learn how to use all available components.</p>
			<a href="{base}/docs/components" class="link">Read Tutorial →</a>
		</div>
		
		<div class="tutorial-card">
			<h2>Styling and Theming</h2>
			<p>Customize the appearance of your application.</p>
			<a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="link">View Docs →</a>
		</div>
	</div>
</div>

<style>
	.page-container {
		max-width: 1280px;
		margin: 0 auto;
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		margin-bottom: 2rem;
		color: var(--primary);
	}

	h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--foreground);
	}

	.tutorials-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.tutorial-card {
		padding: 1.5rem;
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		background: var(--card);
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.tutorial-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
	}

	.tutorial-card p {
		color: var(--muted-foreground);
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.link {
		color: var(--primary);
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 500;
		transition: opacity 0.2s;
	}

	.link:hover {
		opacity: 0.8;
	}
</style>

