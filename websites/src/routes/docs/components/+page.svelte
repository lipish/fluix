<script lang="ts">
	import { 
		Check, 
		X, 
		Search, 
		Settings, 
		Home, 
		Star,
		ChevronDown,
		ChevronRight,
		User,
		Mail,
		Lock,
		Eye,
		EyeOff
	} from 'lucide-svelte';
	
	let selectedComponent = $state('button');
	let selectedCategory = $state('variants');
	let textInputValue = $state('');
	let passwordVisible = $state(false);
	let checkboxChecked = $state(false);
	let checkboxChecked2 = $state(false);
	let checkboxChecked3 = $state(false);
	let selectedValue = $state('');
	let isDropdownOpen = $state(false);
	let selectOpen = $state(false);
	
	const selectOptions = [
		{ value: 'react', label: 'React' },
		{ value: 'vue', label: 'Vue.js' },
		{ value: 'angular', label: 'Angular' },
		{ value: 'svelte', label: 'Svelte' },
		{ value: 'solid', label: 'SolidJS' }
	];
	
	const components = {
		button: {
			name: 'Button',
			categories: {
				variants: { name: 'Variants', icon: Check },
				sizes: { name: 'Sizes', icon: Settings },
				states: { name: 'States', icon: Lock },
				withIcons: { name: 'With Icons', icon: Star }
			}
		},
		icon: {
			name: 'Icon',
			categories: {
				gallery: { name: 'Icon Gallery', icon: Home },
				sizes: { name: 'Sizes', icon: Settings },
				colors: { name: 'Colors', icon: Settings }
			}
		},
		textinput: {
			name: 'TextInput',
			categories: {
				basic: { name: 'Basic', icon: Search },
				states: { name: 'States', icon: Lock },
				validation: { name: 'Validation', icon: Check }
			}
		},
		checkbox: {
			name: 'Checkbox',
			categories: {
				basic: { name: 'Basic', icon: Check },
				states: { name: 'States', icon: Lock },
				group: { name: 'Group', icon: Settings }
			}
		},
		select: {
			name: 'Select',
			categories: {
				basic: { name: 'Basic', icon: ChevronDown },
				states: { name: 'States', icon: Lock },
				multiple: { name: 'Multiple', icon: Settings }
			}
		}
	};
	
	function selectComponent(component: string) {
		selectedComponent = component;
		// 自动选择第一个分类
		const cats = Object.keys(components[component].categories);
		if (cats.length > 0) {
			selectedCategory = cats[0];
		}
	}
	
	function selectCategory(category: string) {
		selectedCategory = category;
	}
</script>

<div class="components-layout">
	<!-- Sidebar Navigation -->
	<aside class="sidebar">
		<h2>Components</h2>
		<nav class="component-nav">
			{#each Object.entries(components) as [key, component]}
				<div class="nav-section">
					<button 
						class="nav-item nav-component"
						class:active={selectedComponent === key}
						onclick={() => selectComponent(key)}
					>
						<span>{component.name}</span>
						<ChevronRight 
							size={16} 
							class={selectedComponent === key ? 'rotate' : ''}
						/>
					</button>
					
					{#if selectedComponent === key}
						<div class="nav-submenu">
							{#each Object.entries(component.categories) as [catKey, category]}
								<button
									class="nav-item nav-category"
									class:active={selectedCategory === catKey}
									onclick={() => selectCategory(catKey)}
								>
									<category.icon size={14} />
									<span>{category.name}</span>
								</button>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		</nav>
	</aside>
	
	<!-- Main Content -->
	<main class="content">
		<div class="content-header">
			<h1>{components[selectedComponent].name}</h1>
			<p class="component-description">
				{#if selectedComponent === 'button'}
					Interactive button component with multiple variants and sizes.
				{:else if selectedComponent === 'icon'}
					22 built-in icons with customizable sizes and colors.
				{:else if selectedComponent === 'textinput'}
					Text input field with validation and event handling.
				{:else if selectedComponent === 'checkbox'}
					Checkbox component for boolean selections.
				{:else if selectedComponent === 'select'}
					Dropdown selection component with single and multiple selection modes.
				{/if}
			</p>
		</div>
		
		<div class="demo-content">
			<!-- Button Demos -->
			{#if selectedComponent === 'button'}
				{#if selectedCategory === 'variants'}
					<section class="demo-section">
						<h2>Variants</h2>
						<div class="demo-row">
							<button class="btn btn-primary">Primary</button>
							<button class="btn btn-secondary">Secondary</button>
							<button class="btn btn-outline">Outline</button>
							<button class="btn btn-ghost">Ghost</button>
							<button class="btn btn-danger">Danger</button>
						</div>
					</section>
				{:else if selectedCategory === 'sizes'}
					<section class="demo-section">
						<h2>Sizes</h2>
						<div class="demo-row">
							<button class="btn btn-primary btn-xs">Extra Small</button>
							<button class="btn btn-primary btn-sm">Small</button>
							<button class="btn btn-primary btn-md">Medium</button>
							<button class="btn btn-primary btn-lg">Large</button>
							<button class="btn btn-primary btn-xl">Extra Large</button>
						</div>
					</section>
				{:else if selectedCategory === 'states'}
					<section class="demo-section">
						<h2>States</h2>
						<div class="demo-row">
							<button class="btn btn-primary">Normal</button>
							<button class="btn btn-primary" disabled>Disabled</button>
							<button class="btn btn-primary loading">Loading...</button>
						</div>
					</section>
				{:else if selectedCategory === 'withIcons'}
					<section class="demo-section">
						<h2>With Icons</h2>
						<div class="demo-row">
							<button class="btn btn-primary">
								<Check size={16} />
								Save
							</button>
							<button class="btn btn-secondary">
								<X size={16} />
								Cancel
							</button>
							<button class="btn btn-outline">
								<Search size={16} />
								Search
							</button>
						</div>
					</section>
				{/if}
			
			<!-- Icon Demos -->
			{:else if selectedComponent === 'icon'}
				{#if selectedCategory === 'gallery'}
					<section class="demo-section">
						<h2>Icon Gallery</h2>
						<div class="icon-grid">
							<div class="icon-item"><Home size={24} /><span>Home</span></div>
							<div class="icon-item"><User size={24} /><span>User</span></div>
							<div class="icon-item"><Settings size={24} /><span>Settings</span></div>
							<div class="icon-item"><Search size={24} /><span>Search</span></div>
							<div class="icon-item"><Star size={24} /><span>Star</span></div>
							<div class="icon-item"><Mail size={24} /><span>Mail</span></div>
							<div class="icon-item"><Lock size={24} /><span>Lock</span></div>
							<div class="icon-item"><Check size={24} /><span>Check</span></div>
						</div>
					</section>
				{:else if selectedCategory === 'sizes'}
					<section class="demo-section">
						<h2>Sizes</h2>
						<div class="demo-row icon-sizes">
							<div class="icon-demo"><Home size={12} /><span>12px</span></div>
							<div class="icon-demo"><Home size={16} /><span>16px</span></div>
							<div class="icon-demo"><Home size={20} /><span>20px</span></div>
							<div class="icon-demo"><Home size={24} /><span>24px</span></div>
							<div class="icon-demo"><Home size={32} /><span>32px</span></div>
						</div>
					</section>
				{:else if selectedCategory === 'colors'}
					<section class="demo-section">
						<h2>Colors</h2>
						<div class="demo-row icon-sizes">
							<div class="icon-demo"><Home size={24} style="color: var(--primary);" /><span>Primary</span></div>
							<div class="icon-demo"><Home size={24} style="color: var(--muted-foreground);" /><span>Muted</span></div>
							<div class="icon-demo"><Home size={24} style="color: #ef4444;" /><span>Red</span></div>
							<div class="icon-demo"><Home size={24} style="color: #22c55e;" /><span>Green</span></div>
							<div class="icon-demo"><Home size={24} style="color: #3b82f6;" /><span>Blue</span></div>
						</div>
					</section>
				{/if}
			
			<!-- TextInput Demos -->
			{:else if selectedComponent === 'textinput'}
				{#if selectedCategory === 'basic'}
					<section class="demo-section">
						<h2>Basic Input</h2>
						<div class="input-demo">
							<input type="text" class="text-input" placeholder="Enter text..." bind:value={textInputValue} />
						</div>
					</section>
					<section class="demo-section">
						<h2>With Icon</h2>
						<div class="input-demo">
							<div class="input-wrapper">
								<Search size={16} class="input-icon" />
								<input type="text" class="text-input with-icon" placeholder="Search..." />
							</div>
						</div>
					</section>
				{:else if selectedCategory === 'states'}
					<section class="demo-section">
						<h2>States</h2>
						<div class="input-demo">
							<input type="text" class="text-input" placeholder="Normal input" />
							<input type="text" class="text-input" placeholder="Disabled input" disabled />
						</div>
					</section>
					<section class="demo-section">
						<h2>Password Input</h2>
						<div class="input-demo">
							<div class="input-wrapper">
								<input 
									type={passwordVisible ? "text" : "password"} 
									class="text-input with-icon" 
									placeholder="Enter password..." 
								/>
								<button 
									class="password-toggle"
									onclick={() => passwordVisible = !passwordVisible}
								>
									{#if passwordVisible}
										<EyeOff size={16} />
									{:else}
										<Eye size={16} />
									{/if}
								</button>
							</div>
						</div>
					</section>
				{:else if selectedCategory === 'validation'}
					<section class="demo-section">
						<h2>Max Length</h2>
						<div class="input-demo">
							<input type="text" class="text-input" placeholder="Max 10 characters" maxlength="10" />
						</div>
					</section>
					<section class="demo-section">
						<h2>Number Only</h2>
						<div class="input-demo">
							<input type="text" class="text-input" placeholder="Numbers only..." pattern="[0-9]*" />
						</div>
					</section>
				{/if}
			
			<!-- Checkbox Demos -->
			{:else if selectedComponent === 'checkbox'}
				{#if selectedCategory === 'basic'}
					<section class="demo-section">
						<h2>Basic Checkbox</h2>
						<div class="checkbox-demo">
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" bind:checked={checkboxChecked} />
								<span>Accept terms and conditions</span>
							</label>
						</div>
					</section>
				{:else if selectedCategory === 'states'}
					<section class="demo-section">
						<h2>States</h2>
						<div class="checkbox-demo">
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" />
								<span>Unchecked</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" checked />
								<span>Checked</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" disabled />
								<span>Disabled</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" checked disabled />
								<span>Checked & Disabled</span>
							</label>
						</div>
					</section>
				{:else if selectedCategory === 'group'}
					<section class="demo-section">
						<h2>Checkbox Group</h2>
						<div class="checkbox-demo">
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" bind:checked={checkboxChecked} />
								<span>Option 1</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" bind:checked={checkboxChecked2} />
								<span>Option 2</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" class="checkbox" bind:checked={checkboxChecked3} />
								<span>Option 3</span>
							</label>
						</div>
					</section>
				{/if}
			
			<!-- Select Demos -->
			{:else if selectedComponent === 'select'}
				{#if selectedCategory === 'basic'}
					<section class="demo-section">
						<h2>Basic Select</h2>
						<div class="select-wrapper">
							<button class="select-button" onclick={() => selectOpen = !selectOpen}>
								<span>{selectedValue || 'Select an option...'}</span>
								<ChevronDown size={16} class={selectOpen ? 'rotate' : ''} />
							</button>
							{#if selectOpen}
								<div class="select-dropdown">
									{#each selectOptions as option}
										<button 
											class="select-option"
											onclick={() => {
												selectedValue = option.label;
												selectOpen = false;
											}}
										>
											{option.label}
										</button>
									{/each}
								</div>
							{/if}
						</div>
					</section>
				{:else if selectedCategory === 'states'}
					<section class="demo-section">
						<h2>States</h2>
						<div class="select-wrapper">
							<button class="select-button">
								<span>Normal Select</span>
								<ChevronDown size={16} />
							</button>
						</div>
						<div class="select-wrapper">
							<button class="select-button" disabled>
								<span>Disabled Select</span>
								<ChevronDown size={16} />
							</button>
						</div>
					</section>
				{:else if selectedCategory === 'multiple'}
					<section class="demo-section">
						<h2>Multiple Selection</h2>
						<p class="hint">Select multiple options:</p>
						<div class="checkbox-demo">
							{#each selectOptions as option}
								<label class="checkbox-label">
									<input type="checkbox" class="checkbox" />
									<span>{option.label}</span>
								</label>
							{/each}
						</div>
					</section>
				{/if}
			{/if}
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

	.nav-section {
		display: flex;
		flex-direction: column;
	}

	.nav-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 0.75rem;
		border: none;
		background: transparent;
		color: var(--foreground);
		font-size: 0.875rem;
		cursor: pointer;
		border-radius: 0.375rem;
		transition: background-color 0.2s;
		text-align: left;
		gap: 0.5rem;
	}

	.nav-component {
		font-weight: 500;
	}

	.nav-component:hover {
		background: var(--secondary);
	}

	.nav-component.active {
		background: var(--secondary);
		color: var(--primary);
	}

	.nav-component :global(svg) {
		transition: transform 0.2s;
	}

	.nav-component.active :global(svg.rotate) {
		transform: rotate(90deg);
	}

	.nav-submenu {
		margin-left: 1rem;
		margin-top: 0.25rem;
		margin-bottom: 0.5rem;
		padding-left: 0.5rem;
		border-left: 2px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.nav-category {
		font-weight: 400;
		font-size: 0.8125rem;
		padding-left: 0.5rem;
	}

	.nav-category:hover {
		background: var(--secondary);
	}

	.nav-category.active {
		background: var(--primary);
		color: var(--primary-foreground);
	}

	.nav-category :global(svg) {
		opacity: 0.7;
	}

	.nav-category.active :global(svg) {
		opacity: 1;
	}

	/* Content */
	.content {
		flex: 1;
		padding: 2rem;
		max-width: 1200px;
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

	.demo-section {
		padding: 1.5rem;
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		background: var(--card);
	}

	.demo-section h2 {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: var(--foreground);
	}

	.demo-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		align-items: center;
	}

	/* Button Styles */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 0.375rem;
		border: 1px solid transparent;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		background: var(--secondary);
		color: var(--secondary-foreground);
	}

	.btn:hover:not(:disabled) {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		background: var(--primary);
		color: var(--primary-foreground);
	}

	.btn-secondary {
		background: var(--secondary);
		color: var(--secondary-foreground);
	}

	.btn-outline {
		background: transparent;
		border-color: var(--border);
		color: var(--foreground);
	}

	.btn-ghost {
		background: transparent;
		color: var(--foreground);
	}

	.btn-ghost:hover:not(:disabled) {
		background: var(--secondary);
	}

	.btn-danger {
		background: #ef4444;
		color: white;
	}

	.btn-xs {
		padding: 0.25rem 0.5rem;
		font-size: 0.6875rem;
		height: 1.25rem;
	}

	.btn-sm {
		padding: 0.375rem 0.75rem;
		font-size: 0.8125rem;
		height: 1.75rem;
	}

	.btn-md {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		height: 2.25rem;
	}

	.btn-lg {
		padding: 0.625rem 1.25rem;
		font-size: 1rem;
		height: 2.75rem;
	}

	.btn-xl {
		padding: 0.75rem 1.5rem;
		font-size: 1.125rem;
		height: 3.25rem;
	}

	.btn.loading {
		position: relative;
		color: transparent;
	}

	.btn.loading::after {
		content: '';
		position: absolute;
		width: 1rem;
		height: 1rem;
		border: 2px solid currentColor;
		border-top-color: transparent;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Icon Styles */
	.icon-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
		gap: 1rem;
	}

	.icon-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		background: var(--background);
		color: var(--foreground);
	}

	.icon-item span {
		font-size: 0.75rem;
		color: var(--muted-foreground);
	}

	.icon-sizes {
		gap: 1.5rem;
	}

	.icon-demo {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.icon-demo span {
		font-size: 0.75rem;
		color: var(--muted-foreground);
	}

	/* Input Styles */
	.input-demo {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-width: 400px;
	}

	.input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.input-icon {
		position: absolute;
		left: 0.75rem;
		color: var(--muted-foreground);
		pointer-events: none;
	}

	.text-input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid var(--border);
		border-radius: 0.375rem;
		background: var(--background);
		color: var(--foreground);
		font-size: 0.875rem;
		transition: border-color 0.2s;
	}

	.text-input:focus {
		outline: none;
		border-color: var(--primary);
	}

	.text-input.with-icon {
		padding-left: 2.5rem;
		padding-right: 2.5rem;
	}

	.text-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.password-toggle {
		position: absolute;
		right: 0.75rem;
		background: none;
		border: none;
		color: var(--muted-foreground);
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
	}

	.password-toggle:hover {
		color: var(--foreground);
	}

	/* Checkbox Styles */
	.checkbox-demo {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
		color: var(--foreground);
	}

	.checkbox {
		width: 1rem;
		height: 1rem;
		cursor: pointer;
		accent-color: var(--primary);
	}

	/* Select Styles */
	.select-wrapper {
		position: relative;
		max-width: 300px;
	}

	.select-button {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 0.75rem;
		border: 1px solid var(--border);
		border-radius: 0.375rem;
		background: var(--background);
		color: var(--foreground);
		font-size: 0.875rem;
		cursor: pointer;
		transition: border-color 0.2s;
	}

	.select-button:hover:not(:disabled) {
		border-color: var(--primary);
	}

	.select-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.select-button :global(svg) {
		transition: transform 0.2s;
	}

	.select-button :global(svg.rotate) {
		transform: rotate(180deg);
	}

	.select-dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		right: 0;
		margin-top: 0.25rem;
		border: 1px solid var(--border);
		border-radius: 0.375rem;
		background: var(--popover);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
		overflow: hidden;
		z-index: 10;
	}

	.select-option {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: none;
		background: transparent;
		color: var(--foreground);
		font-size: 0.875rem;
		text-align: left;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.select-option:hover {
		background: var(--secondary);
	}

	.hint {
		color: var(--muted-foreground);
		margin-bottom: 0.75rem;
		font-size: 0.875rem;
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
	}
</style>
