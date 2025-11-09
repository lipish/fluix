<script lang="ts">
	import { ExternalLink, BookOpen, Code, Search, FileText, Zap } from 'lucide-svelte';
	import { base } from '$app/paths';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	const quickStartCode = `use fluix::*;
use gpui::*;

struct MyApp {
    button: Entity<Button>,
}

impl MyApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let button = cx.new(|_| {
            Button::new("Click me")
                .variant(ButtonVariant::Primary)
        });

        cx.subscribe_in(&button, window, Self::on_click).detach();

        Self { button }
    }

    fn on_click(
        &mut self,
        _: &Entity<Button>,
        _: &ButtonEvent,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        println!("Button clicked!");
    }
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(self.button.clone())
    }
}`;

	const createComponentCode = `let button = cx.new(|cx| {
    Button::new("Click me")
        .variant(ButtonVariant::Primary)
        .size(ComponentSize::Large)
});`;

	const handleEventsCode = `cx.subscribe_in(&button, window, |_, _, event, _, _| {
    match event {
        ButtonEvent::Click => {
            println!("Button clicked!");
        }
    }
}).detach();`;

	const validationCode = `let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter email")
        .validator(|value| value.contains('@'))
        .max_length(100)
});`;

	const passwordCode = `use fluix::components::form::text_input::PasswordMaskMode;

let password = cx.new(|cx| {
    TextInput::new(cx)
        .password(true)
        .password_mask_mode(PasswordMaskMode::Partial {
            prefix_len: 2,
            suffix_len: 2,
        })
});`;
</script>

<div class="page-container">
	<div class="header">
		<h1>API Documentation</h1>
		<p class="subtitle">Complete reference guide for Fluix components and APIs</p>
	</div>

	<div class="content-grid">
		<!-- Quick Start Section -->
		<section class="section">
			<div class="section-header">
				<Zap class="section-icon" />
				<h2>Quick Start</h2>
			</div>
			<div class="section-content">
				<p>Get started with Fluix in minutes! Here's a simple example:</p>
				<div class="code-block">
					<CodeBlock code={quickStartCode} language="rust" />
				</div>
			</div>
		</section>

		<!-- Component Overview -->
		<section class="section">
			<div class="section-header">
				<Code class="section-icon" />
				<h2>Component Overview</h2>
			</div>
			<div class="section-content">
				<p>Fluix provides a comprehensive set of UI components:</p>
				<div class="component-grid">
					<div class="component-card">
						<h3>Basic Components</h3>
						<ul>
							<li><a href="{base}/docs/components">Button</a> - Interactive buttons with variants</li>
							<li><a href="{base}/docs/components">Icon</a> - SVG icons with 31 built-in icons</li>
						</ul>
					</div>
					<div class="component-card">
						<h3>Form Components</h3>
						<ul>
							<li><a href="{base}/docs/components">TextInput</a> - Single-line text input</li>
							<li><a href="{base}/docs/components">TextArea</a> - Multi-line text area</li>
							<li><a href="{base}/docs/components">Checkbox</a> - Boolean checkbox</li>
							<li><a href="{base}/docs/components">Select</a> - Dropdown selection</li>
							<li><a href="{base}/docs/components">Combobox</a> - Searchable dropdown</li>
						</ul>
					</div>
					<div class="component-card">
						<h3>Layout Components</h3>
						<ul>
							<li><a href="{base}/docs/components">Tabs</a> - Tab navigation</li>
							<li><a href="{base}/docs/components">Breadcrumb</a> - Breadcrumb navigation</li>
						</ul>
					</div>
				</div>
			</div>
		</section>

		<!-- Documentation Links -->
		<section class="section">
			<div class="section-header">
				<BookOpen class="section-icon" />
				<h2>Documentation Resources</h2>
			</div>
			<div class="section-content">
				<div class="doc-links">
					<a href="https://docs.rs/fluix/latest/fluix/index.html" target="_blank" rel="noopener noreferrer" class="doc-link primary">
						<div class="doc-link-content">
							<div class="doc-link-header">
								<h3>Complete API Reference</h3>
								<ExternalLink class="external-icon" />
							</div>
							<p>Browse the complete API documentation on docs.rs with detailed method signatures, examples, and type definitions.</p>
							<div class="doc-link-footer">
								<span class="doc-link-url">docs.rs/fluix</span>
							</div>
						</div>
					</a>

					<a href="https://github.com/lipish/fluix/blob/main/docs/COMPONENT-REFERENCE.md" target="_blank" rel="noopener noreferrer" class="doc-link">
						<div class="doc-link-content">
							<div class="doc-link-header">
								<h3>Component Reference</h3>
								<ExternalLink class="external-icon" />
							</div>
							<p>Comprehensive guide with examples, method descriptions, and usage patterns for all components.</p>
						</div>
					</a>

					<a href="{base}/docs/components" class="doc-link">
						<div class="doc-link-content">
							<div class="doc-link-header">
								<h3>Components Documentation</h3>
							</div>
							<p>Complete overview of all Fluix components with examples, installation guides, and usage patterns.</p>
						</div>
					</a>

					<a href="{base}/docs/tutorials" class="doc-link">
						<div class="doc-link-content">
							<div class="doc-link-header">
								<h3>Tutorials</h3>
							</div>
							<p>Learn how to use components, handle events, style your UI, and build complete applications.</p>
						</div>
					</a>
				</div>
			</div>
		</section>

		<!-- Common Patterns -->
		<section class="section">
			<div class="section-header">
				<FileText class="section-icon" />
				<h2>Common Patterns</h2>
			</div>
			<div class="section-content">
				<div class="pattern-list">
					<div class="pattern-item">
						<h3>Creating a Component</h3>
						<div class="code-block">
							<CodeBlock code={createComponentCode} language="rust" />
						</div>
					</div>

					<div class="pattern-item">
						<h3>Handling Events</h3>
						<div class="code-block">
							<CodeBlock code={handleEventsCode} language="rust" />
						</div>
					</div>

					<div class="pattern-item">
						<h3>Text Input with Validation</h3>
						<div class="code-block">
							<CodeBlock code={validationCode} language="rust" />
						</div>
					</div>

					<div class="pattern-item">
						<h3>Password Input</h3>
						<div class="code-block">
							<CodeBlock code={passwordCode} language="rust" />
						</div>
					</div>
				</div>
			</div>
		</section>

		<!-- Search & Navigation -->
		<section class="section">
			<div class="section-header">
				<Search class="section-icon" />
				<h2>Finding What You Need</h2>
			</div>
			<div class="section-content">
				<div class="search-hints">
					<div class="hint-item">
						<h3>Looking for a component?</h3>
						<p>Check out the <a href="{base}/docs/components">Components page</a> to see all available components with examples.</p>
					</div>
					<div class="hint-item">
						<h3>Need method details?</h3>
						<p>Visit the <a href="https://docs.rs/fluix/latest/fluix/index.html" target="_blank" rel="noopener noreferrer">complete API reference</a> for detailed method signatures and documentation.</p>
					</div>
					<div class="hint-item">
						<h3>Stuck on something?</h3>
						<p>Read our <a href="{base}/docs/faq">FAQ</a> or browse the <a href="{base}/docs/tutorials">tutorials</a> for step-by-step guides.</p>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	.page-container {
		max-width: 1280px;
		margin: 0 auto;
		padding: 2rem 1.5rem;
	}

	.header {
		margin-bottom: 3rem;
		text-align: center;
	}

	.header h1 {
		font-size: 2.5rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		color: var(--foreground);
	}

	.subtitle {
		font-size: 1.125rem;
		color: var(--muted-foreground);
		margin: 0;
	}

	.content-grid {
		display: flex;
		flex-direction: column;
		gap: 3rem;
	}

	.section {
		background: var(--card);
		border: 1px solid var(--border);
		border-radius: 0.75rem;
		padding: 2rem;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.section-icon {
		width: 1.5rem;
		height: 1.5rem;
		color: var(--primary);
	}

	.section-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		margin: 0;
		color: var(--foreground);
	}

	.section-content {
		color: var(--foreground);
	}

	.section-content p {
		margin-bottom: 1rem;
		line-height: 1.6;
	}

	.code-block {
		background: var(--muted);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 1rem;
		overflow-x: auto;
		margin: 1rem 0;
	}

	.code-block pre {
		margin: 0;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 0.75rem;
		line-height: 1.6;
		color: var(--foreground);
	}

	.code-block code {
		color: var(--foreground);
		display: block;
		padding-left: 1rem;
		text-indent: 0;
		font-family: inherit;
	}

	.component-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
		margin-top: 1rem;
	}

	.component-card {
		background: var(--secondary);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 1.25rem;
	}

	.component-card h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0 0 0.75rem 0;
		color: var(--foreground);
	}

	.component-card ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.component-card li {
		padding: 0.5rem 0;
		border-bottom: 1px solid var(--border);
	}

	.component-card li:last-child {
		border-bottom: none;
	}

	.component-card a {
		color: var(--foreground);
		text-decoration: none;
		transition: color 0.2s;
	}

	.component-card a:hover {
		color: var(--primary);
	}

	.doc-links {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
		margin-top: 1rem;
	}

	.doc-link {
		display: block;
		background: var(--secondary);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 1.5rem;
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
	}

	.doc-link:hover {
		background: var(--accent);
		border-color: var(--primary);
		transform: translateY(-2px);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
	}

	.doc-link.primary {
		background: linear-gradient(135deg, oklch(0.4 0.15 250) 0%, oklch(0.35 0.15 250) 100%);
		border-color: var(--primary);
		color: white;
	}

	.doc-link.primary:hover {
		background: linear-gradient(135deg, oklch(0.35 0.15 250) 0%, oklch(0.4 0.15 250) 100%);
		transform: translateY(-2px);
		box-shadow: 0 8px 12px -2px rgba(0, 0, 0, 0.2);
	}

	.doc-link-content {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.doc-link-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
	}

	.doc-link-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.doc-link.primary .doc-link-header h3 {
		color: white;
	}

	.doc-link p {
		margin: 0;
		line-height: 1.6;
		font-size: 0.875rem;
		opacity: 0.9;
	}

	.doc-link.primary p {
		color: rgba(255, 255, 255, 0.9);
	}

	.doc-link-footer {
		margin-top: 0.5rem;
	}

	.doc-link-url {
		font-size: 0.75rem;
		opacity: 0.7;
		font-family: monospace;
	}

	.doc-link.primary .doc-link-url {
		color: rgba(255, 255, 255, 0.8);
	}

	.external-icon {
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
	}

	.pattern-list {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		margin-top: 1rem;
	}

	.pattern-item {
		padding: 1.5rem;
		background: var(--secondary);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
	}

	.pattern-item h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0 0 1rem 0;
		color: var(--foreground);
	}

	.search-hints {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		margin-top: 1rem;
	}

	.hint-item {
		padding: 1.25rem;
		background: var(--secondary);
		border-left: 3px solid var(--primary);
		border-radius: 0.375rem;
	}

	.hint-item h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 0.5rem 0;
		color: var(--foreground);
	}

	.hint-item p {
		margin: 0;
		line-height: 1.6;
		color: var(--muted-foreground);
	}

	.hint-item a {
		color: var(--primary);
		text-decoration: none;
		font-weight: 500;
	}

	.hint-item a:hover {
		text-decoration: underline;
	}

	@media (max-width: 768px) {
		.page-container {
			padding: 1.5rem 1rem;
		}

		.header h1 {
			font-size: 2rem;
		}

		.section {
			padding: 1.5rem;
		}

		.component-grid,
		.doc-links {
			grid-template-columns: 1fr;
		}
	}
</style>

