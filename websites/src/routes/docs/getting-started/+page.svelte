<script lang="ts">
	// Getting Started page
</script>

<div class="page-container">
	<h1>Getting Started</h1>
	
	<div class="section">
		<h2>Installation</h2>
		<p>Add Fluix to your <code>Cargo.toml</code>:</p>
		<pre><code>fluix = "0.1"</code></pre>
	</div>
	
	<div class="section">
		<h2>Your First App</h2>
		<p>Create a simple app with a button:</p>
		<pre><code>use fluix::*;
use gpui::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        Window::new(cx)
            .with_assets(fluix::Assets)
            .with_view(|cx| {
                Button::new("Click me")
                    .on_click(|_| {
                        println!("Button clicked!");
                    })
                    .build(cx)
            })
    });
}</code></pre>
	</div>
	
	<div class="section">
		<h2>Next Steps</h2>
		<ul>
			<li><a href="/docs/components">Learn about Components</a></li>
			<li><a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer">View API Documentation</a></li>
			<li><a href="https://github.com/lipish/fluix" target="_blank" rel="noopener noreferrer">Check out Examples</a></li>
		</ul>
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
		margin-bottom: 1rem;
		color: var(--primary);
	}

	h2 {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--foreground);
		margin-top: 2rem;
	}

	.section {
		margin-top: 2rem;
	}

	.section p {
		color: var(--muted-foreground);
		margin-bottom: 0.5rem;
	}

	code {
		background: var(--secondary);
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.875rem;
		color: var(--foreground);
	}

	pre {
		background: var(--secondary);
		padding: 1rem;
		border-radius: 0.5rem;
		overflow-x: auto;
		margin-top: 0.5rem;
		border: 1px solid var(--border);
	}

	pre code {
		background: none;
		padding: 0;
		color: var(--foreground);
	}

	ul {
		list-style: none;
		padding: 0;
		margin-top: 0.5rem;
	}

	li {
		margin-bottom: 0.5rem;
	}

	a {
		color: var(--primary);
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}
</style>

