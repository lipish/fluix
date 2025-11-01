<script lang="ts">
	const components = [
		{ id: 'button', name: 'Button', demo: 'button_demo' },
		{ id: 'icon', name: 'Icon', demo: 'icon_demo' },
		{ id: 'textinput', name: 'TextInput', demo: 'text_input_demo' },
		{ id: 'checkbox', name: 'Checkbox', demo: 'checkbox_demo' },
		{ id: 'select', name: 'Select', demo: 'select_demo' },
		{ id: 'combobox', name: 'Combobox', demo: 'combobox_demo' }
	];
	
	let selectedComponent = $state('button');
	
	const componentDescriptions: Record<string, string> = {
		button: 'Interactive button component with multiple variants and sizes.',
		icon: '22 built-in icons with customizable sizes and colors.',
		textinput: 'Text input field with validation and event handling.',
		checkbox: 'Checkbox component for boolean selections.',
		select: 'Dropdown selection component with single and multiple selection modes.',
		combobox: 'Combobox component with search and selection capabilities.'
	};
	
	function getScreenshotPath(componentId: string): string {
		return `/screenshots/components/${componentId}-demo.png`;
	}
</script>

<div class="components-layout">
	<!-- Sidebar Navigation -->
	<aside class="sidebar">
		<h2>Components</h2>
		<nav class="component-nav">
			{#each components as component}
				<button 
					class="nav-item"
					class:active={selectedComponent === component.id}
					onclick={() => selectedComponent = component.id}
				>
					{component.name}
				</button>
			{/each}
		</nav>
	</aside>
	
	<!-- Main Content -->
	<main class="content">
		<div class="content-header">
			<h1>{components.find(c => c.id === selectedComponent)?.name}</h1>
			<p class="component-description">
				{componentDescriptions[selectedComponent]}
			</p>
		</div>
		
		<div class="demo-content">
			<div class="screenshot-container">
				<img 
					src={getScreenshotPath(selectedComponent)}
					alt="{components.find(c => c.id === selectedComponent)?.name} Demo"
					class="component-screenshot"
					onerror={(e) => {
						const img = e.target as HTMLImageElement;
						img.style.display = 'none';
						const placeholder = img.nextElementSibling as HTMLElement;
						if (placeholder) placeholder.style.display = 'flex';
					}}
				/>
				<div class="screenshot-placeholder" style="display: none;">
					<div class="placeholder-content">
						<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<rect x="3" y="3" width="18" height="18" rx="2" stroke-dasharray="4 4"/>
							<path d="M8 12h8M12 8v8"/>
						</svg>
						<h3>截图未找到</h3>
						<p>请运行以下命令并截图：</p>
						<code class="code-block">
							cargo run --example {components.find(c => c.id === selectedComponent)?.demo}
						</code>
						<p class="hint">截图保存位置：<code>websites/static/screenshots/components/{selectedComponent}-demo.png</code></p>
					</div>
				</div>
			</div>
		</div>
	</main>
</div>

<style>
	.components-layout {
		display: flex;
		gap: 2rem;
		min-height: calc(100vh - 4rem);
	}

	/* Sidebar */
	.sidebar {
		width: 280px;
		flex-shrink: 0;
		padding: 1.5rem;
		border-right: 1px solid var(--border);
		background: var(--card);
		position: sticky;
		top: 4rem;
		height: fit-content;
		max-height: calc(100vh - 4rem);
		overflow-y: auto;
	}

	.sidebar h2 {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: var(--foreground);
	}

	.component-nav {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.nav-item {
		display: flex;
		align-items: center;
		padding: 0.75rem 1rem;
		border: none;
		background: transparent;
		color: var(--foreground);
		font-size: 0.875rem;
		cursor: pointer;
		border-radius: 0.375rem;
		transition: background-color 0.2s;
		text-align: left;
		font-weight: 500;
	}

	.nav-item:hover {
		background: var(--secondary);
	}

	.nav-item.active {
		background: var(--primary);
		color: var(--primary-foreground);
	}

	/* Content */
	.content {
		flex: 1;
		padding: 2rem;
		max-width: 1400px;
	}

	.content-header h1 {
		font-size: 2rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		color: var(--primary);
	}

	.component-description {
		color: var(--muted-foreground);
		margin-bottom: 2rem;
		font-size: 1rem;
	}

	.demo-content {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.screenshot-container {
		position: relative;
		width: 100%;
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		background: var(--card);
		overflow: hidden;
		min-height: 400px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.component-screenshot {
		width: 100%;
		height: auto;
		display: block;
		max-width: 100%;
	}

	.screenshot-placeholder {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--background);
	}

	.placeholder-content {
		text-align: center;
		padding: 2rem;
		max-width: 500px;
	}

	.placeholder-content svg {
		color: var(--muted-foreground);
		margin-bottom: 1rem;
	}

	.placeholder-content h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--foreground);
	}

	.placeholder-content p {
		color: var(--muted-foreground);
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.code-block {
		display: block;
		padding: 1rem;
		background: var(--secondary);
		border-radius: 0.375rem;
		border: 1px solid var(--border);
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 0.875rem;
		color: var(--foreground);
		overflow-x: auto;
		margin: 0.75rem 0;
	}

	code {
		background: var(--secondary);
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.875rem;
		color: var(--foreground);
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
	}

	.hint {
		color: var(--muted-foreground);
		font-size: 0.8125rem;
		margin-top: 0.5rem;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.components-layout {
			flex-direction: column;
		}

		.sidebar {
			width: 100%;
			position: relative;
			top: 0;
			border-right: none;
			border-bottom: 1px solid var(--border);
		}

		.content {
			padding: 1.5rem 1rem;
		}

		.screenshot-container {
			min-height: 300px;
		}
	}
</style>
