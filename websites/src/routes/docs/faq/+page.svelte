<script lang="ts">
	// FAQ page
	import { base } from '$app/paths';
</script>

<div class="page-container">
	<h1>Frequently Asked Questions</h1>
	
	<div class="faq-section">
		<h2>General</h2>
		
		<div class="faq-item">
			<h3>What is Fluix?</h3>
			<p>Fluix is a Rust UI component library for building cross-platform desktop applications using GPUI 0.2.</p>
		</div>
		
		<div class="faq-item">
			<h3>How do I get started?</h3>
			<p>Check out our <a href="{base}/docs/components">Components documentation</a>.</p>
		</div>
		
		<div class="faq-item">
			<h3>Where can I find the API documentation?</h3>
			<p>View the complete API documentation at <a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer">docs.rs/fluix</a>.</p>
		</div>
	</div>
	
	<div class="faq-section">
		<h2>Components</h2>
		
		<div class="faq-item">
			<h3>What components are available?</h3>
			<p>Fluix includes Button, Icon, Select, TextInput, and Checkbox components. See the <a href="{base}/docs/components">Components page</a> for details.</p>
		</div>
		
		<div class="faq-item">
			<h3>How do I handle events?</h3>
			<p>Components provide event handlers like <code>on_click</code> for buttons. See the <a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer">API documentation</a> for details.</p>
		</div>
	</div>
	
	<div class="faq-section">
		<h2>Installation</h2>
		
		<div class="faq-item">
			<h3>How do I install Fluix?</h3>
			<p>Add <code>fluix = "0.1.22"</code> to your <code>Cargo.toml</code> dependencies.</p>
		</div>
		
		<div class="faq-item">
			<h3>What are the requirements?</h3>
			<p>Fluix requires Rust and GPUI 0.2. See the <a href="{base}/docs/components">Components documentation</a> for details.</p>
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
		font-size: 1.5rem;
		font-weight: 600;
		margin-top: 2rem;
		margin-bottom: 1rem;
		color: var(--foreground);
	}

	h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--foreground);
	}

	.faq-section {
		margin-bottom: 2rem;
	}

	.faq-item {
		margin-bottom: 1.5rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid var(--border);
	}

	.faq-item:last-child {
		border-bottom: none;
	}

	.faq-item p {
		color: var(--muted-foreground);
		line-height: 1.6;
	}

	code {
		background: var(--secondary);
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.875rem;
		color: var(--foreground);
	}

	a {
		color: var(--primary);
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}
</style>

