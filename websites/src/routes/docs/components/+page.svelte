<script lang="ts">
	import { 
		Check, 
		X, 
		Search, 
		Settings, 
		Home, 
		Star,
		ChevronDown,
		User,
		Mail,
		Lock
	} from 'lucide-svelte';
	
	let selectedValue = $state('');
	let checkboxChecked = $state(false);
	let textInputValue = $state('');
	let isDropdownOpen = $state(false);
	
	const selectOptions = [
		{ value: 'option1', label: 'Option 1' },
		{ value: 'option2', label: 'Option 2' },
		{ value: 'option3', label: 'Option 3' }
	];
</script>

<div class="page-container">
	<h1>Components</h1>
	<p class="intro">Fluix provides a comprehensive library of UI components for building desktop applications. Explore each component below:</p>
	
	<!-- Button Section -->
	<section class="component-section">
		<h2>Button</h2>
		<p class="section-description">Interactive button component with multiple variants and sizes.</p>
		
		<div class="demo-group">
			<h3>Variants</h3>
			<div class="demo-row">
				<button class="btn btn-primary">Primary</button>
				<button class="btn btn-secondary">Secondary</button>
				<button class="btn btn-outline">Outline</button>
				<button class="btn btn-ghost">Ghost</button>
				<button class="btn btn-danger">Danger</button>
			</div>
		</div>
		
		<div class="demo-group">
			<h3>Sizes</h3>
			<div class="demo-row">
				<button class="btn btn-primary btn-xs">Extra Small</button>
				<button class="btn btn-primary btn-sm">Small</button>
				<button class="btn btn-primary btn-md">Medium</button>
				<button class="btn btn-primary btn-lg">Large</button>
				<button class="btn btn-primary btn-xl">Extra Large</button>
			</div>
		</div>
		
		<div class="demo-group">
			<h3>States</h3>
			<div class="demo-row">
				<button class="btn btn-primary">Normal</button>
				<button class="btn btn-primary" disabled>Disabled</button>
				<button class="btn btn-primary loading">Loading...</button>
			</div>
		</div>
		
		<div class="demo-group">
			<h3>With Icons</h3>
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
		</div>
	</section>
	
	<!-- Icon Section -->
	<section class="component-section">
		<h2>Icon</h2>
		<p class="section-description">22 built-in icons with customizable sizes and colors.</p>
		
		<div class="demo-group">
			<h3>Icons</h3>
			<div class="icon-grid">
				<div class="icon-item">
					<Home size={24} />
					<span>Home</span>
				</div>
				<div class="icon-item">
					<User size={24} />
					<span>User</span>
				</div>
				<div class="icon-item">
					<Settings size={24} />
					<span>Settings</span>
				</div>
				<div class="icon-item">
					<Search size={24} />
					<span>Search</span>
				</div>
				<div class="icon-item">
					<Star size={24} />
					<span>Star</span>
				</div>
				<div class="icon-item">
					<Mail size={24} />
					<span>Mail</span>
				</div>
				<div class="icon-item">
					<Lock size={24} />
					<span>Lock</span>
				</div>
				<div class="icon-item">
					<Check size={24} />
					<span>Check</span>
				</div>
			</div>
		</div>
		
		<div class="demo-group">
			<h3>Sizes</h3>
			<div class="demo-row icon-sizes">
				<div class="icon-demo">
					<Home size={12} />
					<span>12px</span>
				</div>
				<div class="icon-demo">
					<Home size={16} />
					<span>16px</span>
				</div>
				<div class="icon-demo">
					<Home size={20} />
					<span>20px</span>
				</div>
				<div class="icon-demo">
					<Home size={24} />
					<span>24px</span>
				</div>
				<div class="icon-demo">
					<Home size={32} />
					<span>32px</span>
				</div>
			</div>
		</div>
	</section>
	
	<!-- TextInput Section -->
	<section class="component-section">
		<h2>TextInput</h2>
		<p class="section-description">Text input field with validation and event handling.</p>
		
		<div class="demo-group">
			<h3>Basic Input</h3>
			<div class="input-demo">
				<input 
					type="text" 
					class="text-input" 
					placeholder="Enter text..."
					bind:value={textInputValue}
				/>
			</div>
		</div>
		
		<div class="demo-group">
			<h3>With Icon</h3>
			<div class="input-demo">
				<div class="input-wrapper">
					<Search size={16} class="input-icon" />
					<input 
						type="text" 
						class="text-input with-icon" 
						placeholder="Search..."
					/>
				</div>
			</div>
		</div>
		
		<div class="demo-group">
			<h3>States</h3>
			<div class="input-demo">
				<input 
					type="text" 
					class="text-input" 
					placeholder="Normal input"
				/>
				<input 
					type="text" 
					class="text-input" 
					placeholder="Disabled input"
					disabled
				/>
			</div>
		</div>
	</section>
	
	<!-- Checkbox Section -->
	<section class="component-section">
		<h2>Checkbox</h2>
		<p class="section-description">Checkbox component for boolean selections.</p>
		
		<div class="demo-group">
			<h3>Basic Checkbox</h3>
			<div class="checkbox-demo">
				<label class="checkbox-label">
					<input 
						type="checkbox" 
						class="checkbox"
						bind:checked={checkboxChecked}
					/>
					<span>Accept terms and conditions</span>
				</label>
			</div>
		</div>
		
		<div class="demo-group">
			<h3>Multiple Checkboxes</h3>
			<div class="checkbox-demo">
				<label class="checkbox-label">
					<input type="checkbox" class="checkbox" />
					<span>Option 1</span>
				</label>
				<label class="checkbox-label">
					<input type="checkbox" class="checkbox" />
					<span>Option 2</span>
				</label>
				<label class="checkbox-label">
					<input type="checkbox" class="checkbox" />
					<span>Option 3</span>
				</label>
			</div>
		</div>
	</section>
	
	<!-- Select Section -->
	<section class="component-section">
		<h2>Select</h2>
		<p class="section-description">Dropdown selection component with single and multiple selection modes.</p>
		
		<div class="demo-group">
			<h3>Basic Select</h3>
			<div class="select-wrapper">
				<button 
					class="select-button"
					onclick={() => isDropdownOpen = !isDropdownOpen}
				>
					<span>{selectedValue || 'Select an option...'}</span>
					<ChevronDown size={16} class={isDropdownOpen ? 'rotate' : ''} />
				</button>
				{#if isDropdownOpen}
					<div class="select-dropdown">
						{#each selectOptions as option}
							<button 
								class="select-option"
								onclick={() => {
									selectedValue = option.label;
									isDropdownOpen = false;
								}}
							>
								{option.label}
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</section>
	
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
		margin-top: 2rem;
		color: var(--foreground);
	}

	h3 {
		font-size: 1rem;
		font-weight: 600;
		margin-bottom: 0.75rem;
		color: var(--foreground);
	}

	.intro {
		color: var(--muted-foreground);
		margin-bottom: 2rem;
		font-size: 1rem;
	}

	.component-section {
		margin-bottom: 3rem;
		padding-bottom: 2rem;
		border-bottom: 1px solid var(--border);
	}

	.component-section:last-of-type {
		border-bottom: none;
	}

	.section-description {
		color: var(--muted-foreground);
		margin-bottom: 1.5rem;
		font-size: 0.875rem;
	}

	.demo-group {
		margin-bottom: 2rem;
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
		background: var(--card);
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
	}

	.text-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	.select-button:hover {
		border-color: var(--primary);
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
