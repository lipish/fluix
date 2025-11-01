<script lang="ts">
	// Components page
</script>

<div class="page-container">
	<h1>Components</h1>
	<p>Fluix provides a comprehensive library of UI components for building desktop applications.</p>
	
	<div class="components-grid">
		<div class="component-card">
			<h2>Button</h2>
			<p>Interactive button component with multiple variants and sizes.</p>
			<a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="link">
				View API Docs →
			</a>
		</div>
		
		<div class="component-card">
			<h2>Icon</h2>
			<p>22 built-in icons with customizable sizes and colors.</p>
			<a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="link">
				View API Docs →
			</a>
		</div>
		
		<div class="component-card">
			<h2>Select</h2>
			<p>Dropdown selection component with single and multiple selection modes.</p>
			<a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="link">
				View API Docs →
			</a>
		</div>
		
		<div class="component-card">
			<h2>TextInput</h2>
			<p>Text input field with validation and event handling.</p>
			<a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="link">
				View API Docs →
			</a>
		</div>
		
		<div class="component-card">
			<h2>Checkbox</h2>
			<p>Checkbox component for boolean selections.</p>
			<a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="link">
				View API Docs →
			</a>
		</div>
	</div>
	
	<div class="section">
		<h2>Complete Component Reference</h2>
		<p>For detailed API documentation, see the <a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer">API documentation</a>.</p>
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
		margin-bottom: 0.5rem;
		color: var(--primary);
	}

	h2 {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--foreground);
	}

	.components-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
		margin: 2rem 0;
	}

	.component-card {
		padding: 1.5rem;
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		background: var(--card);
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.component-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
	}

	.component-card h2 {
		font-size: 1.25rem;
		margin-bottom: 0.5rem;
	}

	.component-card p {
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

	.section {
		margin-top: 3rem;
		padding-top: 2rem;
		border-top: 1px solid var(--border);
	}

	.section p {
		color: var(--muted-foreground);
		margin-top: 0.5rem;
	}

	.section a {
		color: var(--primary);
		text-decoration: none;
	}

	.section a:hover {
		text-decoration: underline;
	}
</style>

