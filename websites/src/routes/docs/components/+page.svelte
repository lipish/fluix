<script lang="ts">
	import { base } from '$app/paths';
	import { Copy, Check } from 'lucide-svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	
	const components = [
		{ id: 'button', name: 'Button', demo: 'button_demo', category: 'ui' },
		{ id: 'icon', name: 'Icon', demo: 'icon_demo', category: 'ui' },
		{ id: 'textinput', name: 'TextInput', demo: 'text_input_demo', category: 'ui' },
		{ id: 'checkbox', name: 'Checkbox', demo: 'checkbox_demo', category: 'ui' },
		{ id: 'select', name: 'Select', demo: 'select_demo', category: 'ui' },
		{ id: 'combobox', name: 'Combobox', demo: 'combobox_demo', category: 'ui' },
		{ id: 'model_selector', name: 'Model Selector', demo: 'model_selector_demo', category: 'ai' }
	];
	
	let selectedComponent = $state('components-overview');
	let selectedDesignSystemSection = $state('overview');
	let activeTab = $state<Record<string, 'preview' | 'code'>>({});
	let copiedCode = $state<Record<string, boolean>>({});
	
	const componentData: Record<string, {
		description: string;
		installation: string;
		usage: string;
		usageCode: string;
		examples: Array<{ title: string; description?: string; code: string; screenshot?: string }>;
	}> = {
		button: {
			description: 'Interactive button component with multiple variants and sizes.',
			installation: 'fluix = "0.1.24"',
			usage: 'Import and use the Button component:',
			usageCode: `use fluix::{Button, ButtonVariant, ComponentSize};
use gpui::*;

let button = cx.new(|cx| {
    Button::new("Click me")
        .variant(ButtonVariant::Primary)
        .size(ComponentSize::Medium)
});`,
			examples: [
				{
					title: 'Default',
					description: 'Primary button variant',
					code: `let button = cx.new(|cx| {
    Button::new("Button")
        .variant(ButtonVariant::Primary)
});`,
					screenshot: 'button-default.png'
				},
				{
					title: 'Outline',
					code: `let button = cx.new(|cx| {
    Button::new("Outline")
        .variant(ButtonVariant::Outline)
});`,
					screenshot: 'button-outline.png'
				},
				{
					title: 'Secondary',
					code: `let button = cx.new(|cx| {
    Button::new("Secondary")
        .variant(ButtonVariant::Secondary)
});`,
					screenshot: 'button-secondary.png'
				},
				{
					title: 'Danger',
					code: `let button = cx.new(|cx| {
    Button::new("Delete")
        .variant(ButtonVariant::Danger)
});`,
					screenshot: 'button-danger.png'
				},
				{
					title: 'Text',
					code: `let button = cx.new(|cx| {
    Button::new("Text")
        .variant(ButtonVariant::Text)
});`,
					screenshot: 'button-text.png'
				},
				{
					title: 'Small Size',
					code: `let button = cx.new(|cx| {
    Button::new("Button")
        .variant(ButtonVariant::Primary)
        .size(ComponentSize::Small)
});`,
					screenshot: 'button-smallsize.png'
				},
				{
					title: 'Large Size',
					code: `let button = cx.new(|cx| {
    Button::new("Button")
        .variant(ButtonVariant::Primary)
        .size(ComponentSize::Large)
});`,
					screenshot: 'button-largesize.png'
				},
				{
					title: 'Disabled',
					code: `let button = cx.new(|cx| {
    Button::new("Button")
        .variant(ButtonVariant::Primary)
        .disabled(true)
});`,
					screenshot: 'button-disable.png'
				},
				{
					title: 'With Icon',
					code: `use fluix::IconName;

let button = cx.new(|cx| {
    Button::new("Download")
        .variant(ButtonVariant::Primary)
        .icon(IconName::Download)
});`,
					screenshot: 'button-withicon.png'
				},
				{
					title: 'Icon Button',
					code: `use fluix::{Icon, IconName};

// Custom icon button (only icon, no text)
div()
    .flex()
    .items_center()
    .justify_center()
    .size(px(36.))
    .rounded(px(8.))
    .bg(rgb(0x696FC7))
    .child(
        Icon::new(IconName::Attachment)
            .size(fluix::IconSize::Small)
            .color(rgb(0xFFFFFF))
    )`,
					screenshot: 'button-icon.png'
				}
			]
		},
		icon: {
			description: '31 built-in icons with customizable sizes and colors.',
			installation: 'fluix = "0.1.24"',
			usage: 'Import and use the Icon component:',
			usageCode: `use fluix::{Icon, IconName, IconSize};
use gpui::*;

let icon = cx.new(|cx| {
    Icon::new(IconName::Check)
        .size(IconSize::Medium)
});`,
			examples: [
				{
					title: 'Default',
					code: `let icon = cx.new(|cx| {
    Icon::new(IconName::Bell)
});`,
					screenshot: 'icon-default.png'
				},
				{
					title: 'Sizes',
					code: `let icon_xsmall = cx.new(|cx| {
    Icon::new(IconName::Star).xsmall()
});

let icon_small = cx.new(|cx| {
    Icon::new(IconName::Star).small()
});

let icon_medium = cx.new(|cx| {
    Icon::new(IconName::Star).medium()
});

let icon_large = cx.new(|cx| {
    Icon::new(IconName::Star).large()
});

let icon_xlarge = cx.new(|cx| {
    Icon::new(IconName::Star).xlarge()
});`,
					screenshot: 'icon-sizes.png'
				},
				{
					title: 'Colors',
					code: `use gpui::*;

let icon_red = cx.new(|cx| {
    Icon::new(IconName::Heart).color(rgb(0xEF4444))
});

let icon_green = cx.new(|cx| {
    Icon::new(IconName::Success).color(rgb(0x22C55E))
});

let icon_blue = cx.new(|cx| {
    Icon::new(IconName::Info).color(rgb(0x3B82F6))
});

let icon_orange = cx.new(|cx| {
    Icon::new(IconName::Warning).color(rgb(0xF59E0B))
});`,
					screenshot: 'icon-colors.png'
				},
				{
					title: 'All Icons',
					code: `// All 31 built-in icons
Icon::new(IconName::ArrowLeft)
Icon::new(IconName::ArrowRight)
Icon::new(IconName::ArrowUp)
Icon::new(IconName::ArrowDown)
Icon::new(IconName::Check)
Icon::new(IconName::ChevronUpDown)
Icon::new(IconName::ChevronUp)
Icon::new(IconName::ChevronDown)
Icon::new(IconName::Close)
Icon::new(IconName::Plus)
Icon::new(IconName::Minus)
Icon::new(IconName::Search)
Icon::new(IconName::Settings)
Icon::new(IconName::Home)
Icon::new(IconName::User)
Icon::new(IconName::UserPlus)
Icon::new(IconName::Bell)
Icon::new(IconName::Star)
Icon::new(IconName::Heart)
Icon::new(IconName::Menu)
Icon::new(IconName::Info)
Icon::new(IconName::Warning)
Icon::new(IconName::Error)
Icon::new(IconName::Success)
Icon::new(IconName::AlertCircle)
Icon::new(IconName::AlertTriangle)
Icon::new(IconName::UnfoldMore)
Icon::new(IconName::Send)
Icon::new(IconName::Attachment)
Icon::new(IconName::Image)
Icon::new(IconName::LogIn)
Icon::new(IconName::Task)`,
					screenshot: 'icon-all.png'
				}
			]
		},
		textinput: {
			description: 'Text input field with validation and event handling.',
			installation: 'fluix = "0.1.24"',
			usage: 'Import and use the TextInput component:',
			usageCode: `use fluix::TextInput;
use gpui::*;

let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter text...")
});`,
			examples: [
				{
					title: 'Default',
					code: `let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter text...")
});`,
					screenshot: 'textinput-default.png'
				},
				{
					title: 'Password',
					code: `let password_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter password...")
        .password(true)
});`,
					screenshot: 'textinput-password.png'
				},
				{
					title: 'Disabled',
					code: `let disabled_input = cx.new(|cx| {
    TextInput::new(cx)
        .value("This is disabled")
        .disabled(true)
});`,
					screenshot: 'textinput-disabled.png'
				},
				{
					title: 'Max Length',
					code: `let limited_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Max 10 characters")
        .max_length(10)
});`,
					screenshot: 'textinput-maxlength.png'
				},
				{
					title: 'Validator',
					code: `let validated_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Numbers only...")
        .validator(|text| text.chars().all(|c| c.is_numeric()))
});`,
					screenshot: 'textinput-validator.png'
				}
			]
		},
		checkbox: {
			description: 'Checkbox component for boolean selections.',
			installation: 'fluix = "0.1.24"',
			usage: 'Import and use the Checkbox component:',
			usageCode: `use fluix::Checkbox;
use gpui::*;

let checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Accept terms")
});`,
			examples: [
				{
					title: 'Default',
					code: `let checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Accept terms")
});`,
					screenshot: 'checkbox-default.png'
				},
				{
					title: 'Checked',
					code: `let checked_checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Newsletter subscription")
        .checked(true)
});`,
					screenshot: 'checkbox-checked.png'
				},
				{
					title: 'Disabled',
					code: `let disabled_checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Disabled checkbox")
        .disabled(true)
});`,
					screenshot: 'checkbox-disabled.png'
				},
				{
					title: 'Sizes',
					code: `use fluix::ComponentSize;

let xsmall = cx.new(|cx| {
    Checkbox::new(cx)
        .label("XSmall checkbox")
        .size(ComponentSize::XSmall)
});

let small = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Small checkbox")
        .size(ComponentSize::Small)
});

let medium = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Medium checkbox")
        .size(ComponentSize::Medium)
});

let large = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Large checkbox")
        .size(ComponentSize::Large)
});

let xlarge = cx.new(|cx| {
    Checkbox::new(cx)
        .label("XLarge checkbox")
        .size(ComponentSize::XLarge)
});`,
					screenshot: 'checkbox-sizes.png'
				}
			]
		},
		select: {
			description: 'Dropdown selection component with single and multiple selection modes.',
			installation: 'fluix = "0.1.24"',
			usage: 'Import and use the Select component:',
			usageCode: `use fluix::{Select, SelectOption};
use gpui::*;

let select = cx.new(|cx| {
    Select::new(cx)
        .options(vec![
            SelectOption::new("Option 1", "1"),
            SelectOption::new("Option 2", "2"),
        ])
});`,
			examples: [
				{
					title: 'Default',
					code: `let select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Select framework...")
        .options(vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue.js"),
            SelectOption::new("angular", "Angular"),
        ])
});`,
					screenshot: 'select-default.png'
				},
				{
					title: 'Disabled',
					code: `let disabled_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Disabled select...")
        .disabled(true)
        .options(vec![
            SelectOption::new("option1", "Option 1"),
            SelectOption::new("option2", "Option 2"),
        ])
});`,
					screenshot: 'select-disabled.png'
				},
				{
					title: 'Multiple',
					code: `let multi_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Select languages...")
        .multiple(true)
        .options(vec![
            SelectOption::new("rust", "Rust"),
            SelectOption::new("typescript", "TypeScript"),
            SelectOption::new("python", "Python"),
        ])
});`,
					screenshot: 'select-multiple.png'
				},
				{
					title: 'Grouped',
					code: `use fluix::SelectOptionGroup;

let grouped_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Select country...")
        .option_groups(vec![
            SelectOptionGroup::new("North America", vec![
                SelectOption::new("us", "United States"),
                SelectOption::new("ca", "Canada"),
            ]),
            SelectOptionGroup::new("Europe", vec![
                SelectOption::new("uk", "United Kingdom"),
                SelectOption::new("de", "Germany"),
            ]),
        ])
});`,
					screenshot: 'select-grouped.png'
				}
			]
		},
		combobox: {
			description: 'Combobox component with search and selection capabilities.',
			installation: 'fluix = "0.1.24"',
			usage: 'Import and use the Combobox component:',
			usageCode: `use fluix::{Combobox, SelectOption};
use gpui::*;

let combobox = cx.new(|cx| {
    Combobox::new(cx)
        .options(vec![
            SelectOption::new("Option 1", "1"),
            SelectOption::new("Option 2", "2"),
        ])
});`,
			examples: [
				{
					title: 'Default',
					code: `let combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Search or select a framework...")
        .options(vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue.js"),
            SelectOption::new("angular", "Angular"),
        ])
});`,
					screenshot: 'combobox-default.png'
				},
				{
					title: 'Pre-selected',
					code: `let combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Search or select a language...")
        .value("rust")
        .input_value("Rust")
        .options(vec![
            SelectOption::new("rust", "Rust"),
            SelectOption::new("typescript", "TypeScript"),
            SelectOption::new("python", "Python"),
        ])
});`,
					screenshot: 'combobox-preselected.png'
				},
				{
					title: 'Disabled',
					code: `let disabled_combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Disabled combobox...")
        .disabled(true)
        .options(vec![
            SelectOption::new("option1", "Option 1"),
            SelectOption::new("option2", "Option 2"),
        ])
});`,
					screenshot: 'combobox-disabled.png'
				},
				{
					title: 'Grouped Options',
					description: 'Organize options into logical groups for better organization',
					code: `use fluix::SelectOptionGroup;

let grouped_combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Select a language...")
        .option_groups(vec![
            SelectOptionGroup::new("Frontend", vec![
                SelectOption::new("javascript", "JavaScript"),
                SelectOption::new("typescript", "TypeScript"),
                SelectOption::new("react", "React"),
            ]),
            SelectOptionGroup::new("Backend", vec![
                SelectOption::new("rust", "Rust"),
                SelectOption::new("python", "Python"),
                SelectOption::new("go", "Go"),
            ]),
        ])
});`,
					screenshot: 'combobox-grouped.png'
				},
				{
					title: 'Fixed Width',
					description: 'Use fixed_width(true) to keep the expand icon at the right edge when using borders',
					code: `let fixed_combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Select an option...")
        .fixed_width(true)  // Icon stays at right edge
        .options(vec![
            SelectOption::new("option1", "Short"),
            SelectOption::new("option2", "Very Long Option Text"),
            SelectOption::new("option3", "Medium"),
        ])
});`,
					screenshot: 'combobox-fixed-width.png'
				},
				{
					title: 'Right-Aligned Text',
					description: 'Use text_alignment(TextAlign::Right) with fixed_width(true) to align text to the right edge',
					code: `use gpui::TextAlign;

let right_aligned_combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Select an option...")
        .fixed_width(true)
        .text_alignment(TextAlign::Right)  // Text aligns to right
        .options(vec![
            SelectOption::new("option1", "Short"),
            SelectOption::new("option2", "Very Long Option Text"),
            SelectOption::new("option3", "Medium"),
        ])
});`,
					screenshot: 'combobox-right-aligned.png'
				}
			]
		},
		model_selector: {
			description: 'AI model selection component with search, grouping, and right-alignment capabilities.',
			installation: 'fluix = "0.1.24"',
			usage: 'Import and use the ModelSelector component:',
			usageCode: `use fluix::{ModelSelector, ModelInfo, ModelCapability};
use gpui::*;

let models = vec![
    ModelInfo {
        id: "gpt-4o".to_string(),
        name: "GPT-4o".to_string(),
        provider: "OpenAI".to_string(),
        capabilities: vec![ModelCapability::TextGeneration],
        // ... other fields
    }
];

let selector = cx.new(|cx| {
    ModelSelector::new(cx, models)
});`,
			examples: [
				{
					title: 'Default',
					description: 'Basic model selector with default configuration',
					code: `let selector = cx.new(|cx| {
    ModelSelector::new(cx, models)
});`,
					screenshot: 'model-selector-default.png'
				},
				{
					title: 'Right-Aligned',
					description: 'Model selector with right-aligned text (expand button stays fixed at right edge)',
					code: `let right_aligned_selector = cx.new(|cx| {
    ModelSelector::new(cx, models)
        .right_align_text(true)  // Text aligns to right
});`,
					screenshot: 'model-selector-right-aligned.png'
				},
				{
					title: 'Grouped by Provider',
					description: 'Organize models by provider with grouping',
					code: `let grouped_selector = cx.new(|cx| {
    ModelSelector::new(cx, models)
        .group_by_provider(true)
});`,
					screenshot: 'model-selector-grouped.png'
				}
			]
		}
	};
	
	function getScreenshotPath(componentId: string, screenshot?: string): string {
		if (screenshot) {
			return `${base}/screenshots/components/${screenshot}`;
		}
		return `${base}/screenshots/components/${componentId}-demo.png`;
	}
	
	function getScreenshotPaths(componentId: string): string[] {
		if (componentId === 'icon') {
			return [
				`${base}/screenshots/components/icon-demo.png`,
				`${base}/screenshots/components/icon-demo2.png`
			];
		}
		if (componentId === 'textinput') {
			return [
				`${base}/screenshots/components/textinput-demo.png`,
				`${base}/screenshots/components/textinput-demo2.png`
			];
		}
		if (componentId === 'select') {
			return [
				`${base}/screenshots/components/select-demo.png`,
				`${base}/screenshots/components/select-demo2.png`,
				`${base}/screenshots/components/select-demo3.png`
			];
		}
		return [getScreenshotPath(componentId)];
	}
	
	function copyCode(exampleId: string, code: string) {
		navigator.clipboard.writeText(code).then(() => {
			copiedCode[exampleId] = true;
			setTimeout(() => {
				copiedCode[exampleId] = false;
			}, 2000);
		});
	}
	
	function getExampleId(componentId: string, index: number): string {
		return `${componentId}-${index}`;
	}
	
	function setTab(componentId: string, exampleIndex: number, tab: 'preview' | 'code') {
		const exampleId = getExampleId(componentId, exampleIndex);
		activeTab[exampleId] = tab;
	}
	
	function getTab(componentId: string, exampleIndex: number): 'preview' | 'code' {
		const exampleId = getExampleId(componentId, exampleIndex);
		return activeTab[exampleId] || 'preview';
	}
</script>

<div class="components-layout">
	<!-- Sidebar Navigation -->
	<aside class="sidebar">
		<!-- Components Overview -->
		<button 
			class="nav-item nav-title"
			class:active={selectedComponent === 'components-overview'}
			onclick={() => selectedComponent = 'components-overview'}
		>
			Components Overview
		</button>
		
		<!-- Design System -->
		<button 
			class="nav-item nav-title"
			class:active={selectedComponent === 'design-system' && selectedDesignSystemSection === 'overview'}
			onclick={() => { selectedComponent = 'design-system'; selectedDesignSystemSection = 'overview'; }}
		>
			Design System
		</button>
		<nav class="component-nav">
			<button 
				class="nav-item sub-item"
				class:active={selectedComponent === 'design-system' && selectedDesignSystemSection === 'sizes'}
				onclick={() => { selectedComponent = 'design-system'; selectedDesignSystemSection = 'sizes'; }}
			>
				Sizes
			</button>
			<button 
				class="nav-item sub-item"
				class:active={selectedComponent === 'design-system' && selectedDesignSystemSection === 'colors'}
				onclick={() => { selectedComponent = 'design-system'; selectedDesignSystemSection = 'colors'; }}
			>
				Colors
			</button>
		</nav>
		
		<!-- UI Components -->
		<button 
			class="nav-item nav-title"
			class:active={selectedComponent === 'ui-components-list'}
			onclick={() => selectedComponent = 'ui-components-list'}
		>
			UI Components
		</button>
		<nav class="component-nav">
			{#each components.filter(c => c.category === 'ui') as component}
				<button 
					class="nav-item sub-item"
					class:active={selectedComponent === component.id}
					onclick={() => selectedComponent = component.id}
				>
					{component.name}
				</button>
			{/each}
		</nav>
		
		<!-- AI Components -->
		<button 
			class="nav-item nav-title"
			class:active={selectedComponent === 'ai-components-list'}
			onclick={() => selectedComponent = 'ai-components-list'}
		>
			AI Components
		</button>
		<nav class="component-nav">
			{#each components.filter(c => c.category === 'ai') as component}
				<button 
					class="nav-item sub-item"
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
		{#if selectedComponent === 'components-overview'}
			<!-- Components Overview -->
			<div class="content-header">
				<h1>Components Overview</h1>
				<p class="component-description">Complete overview of all Fluix components, organized by category.</p>
			</div>

			<!-- Design System Section -->
			<section class="section">
				<h2>Design System</h2>
				<p class="section-description">Unified size, color, and theme standards that apply to all Fluix components.</p>
				
				<div class="overview-grid">
					<div class="overview-card clickable-card" onclick={() => { selectedComponent = 'design-system'; selectedDesignSystemSection = 'sizes'; }}>
						<h3>Sizes</h3>
						<p>Standardized component sizes (XSmall to XLarge) with consistent dimensions and spacing.</p>
						<div class="card-arrow">→</div>
					</div>
					<div class="overview-card clickable-card" onclick={() => { selectedComponent = 'design-system'; selectedDesignSystemSection = 'colors'; }}>
						<h3>Colors</h3>
						<p>Unified color palette with primary, secondary, and semantic colors for consistent theming.</p>
						<div class="card-arrow">→</div>
					</div>
				</div>
			</section>

			<!-- UI Components Section -->
			<section class="section">
				<h2>UI Components</h2>
				<p class="section-description">Essential user interface components for building interactive applications.</p>
				
				<div class="overview-grid">
					{#each components.filter(c => c.category === 'ui') as component}
						<div class="overview-card clickable-card" onclick={() => selectedComponent = component.id}>
							<h3>{component.name}</h3>
							{#if componentData[component.id]}
								<p>{componentData[component.id].description}</p>
							{/if}
							<div class="card-arrow">→</div>
						</div>
					{/each}
				</div>
			</section>

			<!-- AI Components Section -->
			<section class="section">
				<h2>AI Components</h2>
				<p class="section-description">Specialized components for AI-powered applications and model interactions.</p>
				
				<div class="overview-grid">
					{#each components.filter(c => c.category === 'ai') as component}
						<div class="overview-card clickable-card" onclick={() => selectedComponent = component.id}>
							<h3>{component.name}</h3>
							{#if componentData[component.id]}
								<p>{componentData[component.id].description}</p>
							{/if}
							<div class="card-arrow">→</div>
						</div>
					{/each}
				</div>
			</section>
		{:else if selectedComponent === 'design-system'}
			{#if selectedDesignSystemSection === 'overview'}
				<!-- Design System Content -->
				<div class="content-header">
					<h1>Design System</h1>
					<p class="component-description">Unified size, color, and theme standards that apply to all Fluix components.</p>
				</div>

				<!-- Component Sizes -->
				<section class="section">
					<h2>Component Sizes</h2>
				<p>All components use a unified size system defined by the <code>ComponentSize</code> enum:</p>
				
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`pub enum ComponentSize {
    XSmall,   // 11px font, 20px height
    Small,    // 13px font, 28px height
    Medium,   // 14px font, 36px height (default)
    Large,    // 16px font, 44px height
    XLarge,   // 18px font, 52px height
}`} language="rust" />
				</div>

				<h3>Size Standards</h3>
				<p>All components using <code>ComponentSize</code> follow standardized dimensions:</p>
				
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Size</th>
								<th>Font Size</th>
								<th>Height</th>
								<th>Padding (Y/X)</th>
								<th>Min Width</th>
							</tr>
						</thead>
						<tbody>
							<tr><td>XSmall</td><td>11px</td><td>~20px</td><td>4px / 8px</td><td>60px</td></tr>
							<tr><td>Small</td><td>13px</td><td>~28px</td><td>6px / 12px</td><td>72px</td></tr>
							<tr><td>Medium</td><td>14px</td><td>~36px</td><td>8px / 16px</td><td>88px</td></tr>
							<tr><td>Large</td><td>16px</td><td>~44px</td><td>10px / 20px</td><td>104px</td></tr>
							<tr><td>XLarge</td><td>18px</td><td>~52px</td><td>12px / 24px</td><td>120px</td></tr>
						</tbody>
					</table>
				</div>

				<h3>Size Usage Guidelines</h3>
				<ul>
					<li><strong>XSmall</strong>: Compact buttons, dense UIs</li>
					<li><strong>Small</strong>: Secondary actions, compact forms</li>
					<li><strong>Medium</strong>: Standard buttons, most common use case (default)</li>
					<li><strong>Large</strong>: Primary actions, prominent buttons</li>
					<li><strong>XLarge</strong>: Hero buttons, very prominent actions</li>
				</ul>

				<h3>Width Behavior</h3>
				<ul>
					<li><strong>Minimum Width</strong>: Each size has a standard minimum width</li>
					<li><strong>Auto-expand</strong>: If content exceeds minimum width, components automatically expand to fit content</li>
					<li><strong>Manual Override</strong>: Some components support <code>.full_width(true)</code> to fill container width</li>
				</ul>
				</section>
			{:else if selectedDesignSystemSection === 'sizes'}
				<!-- Component Sizes -->
				<div class="content-header">
					<h1>Component Sizes</h1>
					<p class="component-description">Unified size system for all Fluix components.</p>
				</div>

				<section class="section">
					<h2>Component Sizes</h2>
					<p>All components use a unified size system defined by the <code>ComponentSize</code> enum:</p>
					
					<div class="code-container">
						<div class="code-header">
							<span class="code-label">Rust</span>
						</div>
						<CodeBlock code={`pub enum ComponentSize {
    XSmall,   // 11px font, 20px height
    Small,    // 13px font, 28px height
    Medium,   // 14px font, 36px height (default)
    Large,    // 16px font, 44px height
    XLarge,   // 18px font, 52px height
}`} language="rust" />
					</div>

					<h3>Size Standards</h3>
					<p>All components using <code>ComponentSize</code> follow standardized dimensions:</p>
					
					<div class="standards-table">
						<table>
							<thead>
								<tr>
									<th>Size</th>
									<th>Font Size</th>
									<th>Height</th>
									<th>Padding (Y/X)</th>
									<th>Min Width</th>
								</tr>
							</thead>
							<tbody>
								<tr><td>XSmall</td><td>11px</td><td>~20px</td><td>4px / 8px</td><td>60px</td></tr>
								<tr><td>Small</td><td>13px</td><td>~28px</td><td>6px / 12px</td><td>72px</td></tr>
								<tr><td>Medium</td><td>14px</td><td>~36px</td><td>8px / 16px</td><td>88px</td></tr>
								<tr><td>Large</td><td>16px</td><td>~44px</td><td>10px / 20px</td><td>104px</td></tr>
								<tr><td>XLarge</td><td>18px</td><td>~52px</td><td>12px / 24px</td><td>120px</td></tr>
							</tbody>
						</table>
					</div>

					<h3>Size Usage Guidelines</h3>
					<ul>
						<li><strong>XSmall</strong>: Compact buttons, dense UIs</li>
						<li><strong>Small</strong>: Secondary actions, compact forms</li>
						<li><strong>Medium</strong>: Standard buttons, most common use case (default)</li>
						<li><strong>Large</strong>: Primary actions, prominent buttons</li>
						<li><strong>XLarge</strong>: Hero buttons, very prominent actions</li>
					</ul>

					<h3>Width Behavior</h3>
					<ul>
						<li><strong>Minimum Width</strong>: Each size has a standard minimum width</li>
						<li><strong>Auto-expand</strong>: If content exceeds minimum width, components automatically expand to fit content</li>
						<li><strong>Manual Override</strong>: Some components support <code>.full_width(true)</code> to fill container width</li>
					</ul>
				</section>
			{:else if selectedDesignSystemSection === 'colors'}
				<!-- Color System -->
				<div class="content-header">
					<h1>Color System</h1>
					<p class="component-description">Unified theme color system for all Fluix components.</p>
				</div>

				<section class="section">
					<h2>Color System</h2>
				<p>Fluix provides a unified theme color system that all components follow.</p>

				<h3>Theme Colors</h3>
				<p>Access theme colors using <code>Theme::default()</code>:</p>
				
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`use fluix::theme::*;

let theme = Theme::default();

// Primary colors
theme.colors.primary      // #696FC7 (Purple)
theme.colors.secondary    // #6B7280 (Gray)

// Semantic colors
theme.colors.success      // #22C55E (Green)
theme.colors.warning      // #F59E0B (Orange)
theme.colors.error        // #EF4444 (Red)
theme.colors.info         // #3B82F6 (Blue)

// Neutral colors
theme.colors.background   // #FFFFFF (White)
theme.colors.surface      // #F9FAFB (Light Gray)
theme.colors.border       // #E5E7EB (Border Gray)
theme.colors.text         // #111827 (Dark Gray)
theme.colors.text_muted   // #6B7280 (Muted Gray)`} language="rust" />
				</div>

				<h3>Customizing Theme Colors</h3>
				<p>You can create a custom theme with your own colors:</p>
				
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`use fluix::theme::*;
use gpui::*;

// Method 1: Modify default theme
let mut theme = Theme::default();
theme.colors.primary = rgb(0x3498DB);  // Custom blue
theme.colors.secondary = rgb(0x95A5A6); // Custom gray

// Method 2: Create custom ColorPalette
let custom_colors = ColorPalette {
    primary: rgb(0x3498DB),
    primary_hover: rgb(0x5DADE2),
    primary_active: rgb(0x2874A6),
    secondary: rgb(0x95A5A6),
    // ... set other colors as needed
    ..ColorPalette::default()  // Use defaults for remaining colors
};

let custom_theme = Theme::custom(custom_colors);`} language="rust" />
				</div>

				<h3>Component Variant Colors</h3>
				<p>Component variants use unified color definitions:</p>
				
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Variant</th>
								<th>Background</th>
								<th>Text Color</th>
								<th>Border</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>Primary</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #696FC7;"></span>
										<span>#696FC7 (Purple)</span>
									</div>
								</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #FFFFFF;"></span>
										<span>#FFFFFF (White)</span>
									</div>
								</td>
								<td>None</td>
							</tr>
							<tr>
								<td>Secondary</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #FFFFFF;"></span>
										<span>#FFFFFF (White)</span>
									</div>
								</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #333333;"></span>
										<span>#333333 (Dark Gray)</span>
									</div>
								</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #E0E0E0;"></span>
										<span>#E0E0E0</span>
									</div>
								</td>
							</tr>
							<tr>
								<td>Outline</td>
								<td>Transparent</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #333333;"></span>
										<span>#333333 (Dark Gray)</span>
									</div>
								</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #E0E0E0;"></span>
										<span>#E0E0E0</span>
									</div>
								</td>
							</tr>
							<tr>
								<td>Text</td>
								<td>Transparent</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #333333;"></span>
										<span>#333333 (Dark Gray)</span>
									</div>
								</td>
								<td>None</td>
							</tr>
							<tr>
								<td>Danger</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #E74C3C;"></span>
										<span>#E74C3C (Red)</span>
									</div>
								</td>
								<td>
									<div class="color-cell">
										<span class="color-swatch" style="background-color: #FFFFFF;"></span>
										<span>#FFFFFF (White)</span>
									</div>
								</td>
								<td>None</td>
							</tr>
						</tbody>
					</table>
				</div>
				</section>
			{/if}
		{:else if selectedComponent === 'ui-components-list'}
			<!-- UI Components List -->
			<div class="content-header">
				<h1>UI Components</h1>
				<p class="component-description">Essential user interface components for building interactive applications.</p>
			</div>

			<div class="components-grid">
				{#each components.filter(c => c.category === 'ui') as component}
					<div class="component-card-link" onclick={() => selectedComponent = component.id}>
						<div class="component-card-header">
							<h3>{component.name}</h3>
							{#if componentData[component.id]}
								<p class="component-card-description">{componentData[component.id].description}</p>
							{/if}
						</div>
						<div class="component-card-footer">
							<span class="component-card-link-text">View Details →</span>
						</div>
					</div>
				{/each}
			</div>
		{:else if selectedComponent === 'ai-components-list'}
			<!-- AI Components List -->
			<div class="content-header">
				<h1>AI Components</h1>
				<p class="component-description">Specialized components for AI-powered applications and model interactions.</p>
			</div>

			<div class="components-grid">
				{#each components.filter(c => c.category === 'ai') as component}
					<div class="component-card-link" onclick={() => selectedComponent = component.id}>
						<div class="component-card-header">
							<h3>{component.name}</h3>
							{#if componentData[component.id]}
								<p class="component-card-description">{componentData[component.id].description}</p>
							{/if}
						</div>
						<div class="component-card-footer">
							<span class="component-card-link-text">View Details →</span>
						</div>
					</div>
				{/each}
			</div>
		{:else if componentData[selectedComponent]}
			{@const data = componentData[selectedComponent]}
			{@const componentName = components.find(c => c.id === selectedComponent)?.name || ''}
			
			<!-- Header -->
			<div class="content-header">
				<h1>{componentName}</h1>
				<p class="component-description">{data.description}</p>
			</div>
			
			<!-- Installation -->
			<section class="section">
				<h2>Installation</h2>
				<p>Add Fluix to your <code>Cargo.toml</code>:</p>
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Cargo.toml</span>
						<button 
							class="copy-button"
							onclick={() => copyCode('installation', `fluix = "${data.installation.split('"')[1] || '0.1.22'}"`)}
							title="Copy code"
						>
							{#if copiedCode['installation']}
								<Check size={14} />
							{:else}
								<Copy size={14} />
							{/if}
						</button>
					</div>
					<CodeBlock code={data.installation} language="toml" filename="Cargo.toml" />
				</div>
			</section>
			
			<!-- Usage -->
			<section class="section">
				<h2>Usage</h2>
				<p>{data.usage}</p>
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
						<button 
							class="copy-button"
							onclick={() => copyCode('usage', data.usageCode)}
							title="Copy code"
						>
							{#if copiedCode['usage']}
								<Check size={14} />
							{:else}
								<Copy size={14} />
							{/if}
						</button>
					</div>
					<CodeBlock code={data.usageCode} language="rust" />
				</div>
			</section>
			
			<!-- API Reference -->
			{#if selectedComponent === 'textinput'}
			<section class="section">
				<h2>API Reference</h2>
				<h3>Methods</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Method</th>
								<th>Parameters</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>.placeholder()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set placeholder text displayed when input is empty</td>
							</tr>
							<tr>
								<td><code>.value()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set initial value for the input</td>
							</tr>
							<tr>
								<td><code>.disabled()</code></td>
								<td><code>bool</code></td>
								<td>Enable/disable the input. Disabled inputs cannot be focused or edited</td>
							</tr>
							<tr>
								<td><code>.password()</code></td>
								<td><code>bool</code></td>
								<td>Enable password mode (mask characters). Requires <code>.password_mask_mode()</code> for customization</td>
							</tr>
							<tr>
								<td><code>.password_mask_mode()</code></td>
								<td><code>PasswordMaskMode</code></td>
								<td>Set password masking mode. See <strong>Password Mask Modes</strong> below for details</td>
							</tr>
							<tr>
								<td><code>.show_password()</code></td>
								<td><code>bool</code></td>
								<td>Set initial password visibility state (<code>true</code> = visible, <code>false</code> = masked)</td>
							</tr>
							<tr>
								<td><code>.max_length()</code></td>
								<td><code>usize</code></td>
								<td>Set maximum allowed length. Attempts to exceed this limit will be rejected</td>
							</tr>
							<tr>
								<td><code>.validator()</code></td>
								<td><code>Fn(&amp;str) -&gt; bool</code></td>
								<td>Set custom validation function. Returns <code>true</code> for valid values, <code>false</code> to reject</td>
							</tr>
							<tr>
								<td><code>.no_border()</code></td>
								<td>-</td>
								<td>Remove border (useful for embedded use cases like combobox)</td>
							</tr>
							<tr>
								<td><code>.bg_color()</code></td>
								<td><code>Rgba</code></td>
								<td>Set custom background color</td>
							</tr>
							<tr>
								<td><code>.border_color()</code></td>
								<td><code>Rgba</code></td>
								<td>Set custom border color</td>
							</tr>
							<tr>
								<td><code>.transparent()</code></td>
								<td>-</td>
								<td>Make background transparent (convenience method)</td>
							</tr>
						</tbody>
					</table>
				</div>
				
				<h3>Password Mask Modes</h3>
				<p>TextInput supports flexible password masking with two modes:</p>
				
				<h4>1. Full Mask Mode (Default)</h4>
				<p>All characters are masked with bullets (•). This provides maximum security by hiding all password characters.</p>
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`use fluix::components::form::text_input::PasswordMaskMode;

let password_input = cx.new(|cx| {
    TextInput::new(cx)
        .password(true)
        .password_mask_mode(PasswordMaskMode::All)
        .placeholder("Enter password");
});
// Password "secret123" displays as "•••••••••"`} language="rust" />
				</div>
				
				<h4>2. Partial Mask Mode</h4>
				<p>Show first and last few characters, mask the middle part. This provides a balance between security and usability, allowing users to verify they're typing the correct password while still hiding most of it.</p>
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`use fluix::components::form::text_input::PasswordMaskMode;

let password_input = cx.new(|cx| {
    TextInput::new(cx)
        .password(true)
        .password_mask_mode(PasswordMaskMode::Partial {
            prefix_len: 2,  // Show first 2 characters
            suffix_len: 2,  // Show last 2 characters
        })
        .placeholder("Enter password");
});
// Password "password123" displays as "pa••••••••23"
// Password "f26612345678944u9" displays as "f2••••••••••••••44u9"

// Note: If password is too short (length <= prefix_len + suffix_len),
// all characters will be masked regardless of the mode`} language="rust" />
				</div>
				
				<h4>Password Visibility Toggle</h4>
				<p>You can toggle password visibility programmatically:</p>
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`let password_input = cx.new(|cx| {
    TextInput::new(cx)
        .password(true)
        .password_mask_mode(PasswordMaskMode::Partial {
            prefix_len: 2,
            suffix_len: 2,
        })
        .show_password(false)  // Start hidden
});

// Toggle visibility when user clicks a button
password_input.update(cx, |input, cx| {
    input.toggle_password_visibility(cx);
});`} language="rust" />
				</div>
				
				<h3>Events</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Event</th>
								<th>Payload</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>TextInputEvent::Change</code></td>
								<td><code>String</code></td>
								<td>Emitted when input value changes (typing, deleting, or programmatic change)</td>
							</tr>
							<tr>
								<td><code>TextInputEvent::Submit</code></td>
								<td><code>String</code></td>
								<td>Emitted when Enter key is pressed</td>
							</tr>
							<tr>
								<td><code>TextInputEvent::Focus</code></td>
								<td>-</td>
								<td>Emitted when input gains focus</td>
							</tr>
							<tr>
								<td><code>TextInputEvent::Blur</code></td>
								<td>-</td>
								<td>Emitted when input loses focus</td>
							</tr>
						</tbody>
					</table>
				</div>
			</section>
			{:else if selectedComponent === 'checkbox'}
			<section class="section">
				<h2>API Reference</h2>
				<h3>Methods</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Method</th>
								<th>Parameters</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>.label()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set label text displayed next to the checkbox</td>
							</tr>
							<tr>
								<td><code>.checked()</code></td>
								<td><code>bool</code></td>
								<td>Set initial checked state (<code>true</code> = checked, <code>false</code> = unchecked)</td>
							</tr>
							<tr>
								<td><code>.disabled()</code></td>
								<td><code>bool</code></td>
								<td>Enable/disable the checkbox. Disabled checkboxes cannot be clicked</td>
							</tr>
							<tr>
								<td><code>.size()</code></td>
								<td><code>ComponentSize</code></td>
								<td>Set checkbox size (XSmall, Small, Medium, Large, XLarge). Affects box size and text size</td>
							</tr>
							<tr>
								<td><code>.text_color()</code></td>
								<td><code>Rgba</code></td>
								<td>Set custom text color for the label</td>
							</tr>
						</tbody>
					</table>
				</div>
				
				<h3>Events</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Event</th>
								<th>Payload</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>CheckboxEvent::Changed</code></td>
								<td><code>bool</code></td>
								<td>Emitted when checkbox state changes (checked/unchecked)</td>
							</tr>
						</tbody>
					</table>
				</div>
				
				<h3>Size Reference</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Size</th>
								<th>Box Size</th>
								<th>Font Size</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>XSmall</td>
								<td>14px</td>
								<td>11px</td>
							</tr>
							<tr>
								<td>Small</td>
								<td>16px</td>
								<td>13px</td>
							</tr>
							<tr>
								<td>Medium</td>
								<td>18px</td>
								<td>14px (default)</td>
							</tr>
							<tr>
								<td>Large</td>
								<td>20px</td>
								<td>16px</td>
							</tr>
							<tr>
								<td>XLarge</td>
								<td>22px</td>
								<td>18px</td>
							</tr>
						</tbody>
					</table>
				</div>
			</section>
			{:else if selectedComponent === 'select'}
			<section class="section">
				<h2>API Reference</h2>
				<h3>Methods</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Method</th>
								<th>Parameters</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>.options()</code></td>
								<td><code>Vec&lt;SelectOption&gt;</code></td>
								<td>Set available options (flat list)</td>
							</tr>
							<tr>
								<td><code>.option_groups()</code></td>
								<td><code>Vec&lt;SelectOptionGroup&gt;</code></td>
								<td>Set grouped options (hierarchical structure)</td>
							</tr>
							<tr>
								<td><code>.placeholder()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set placeholder text displayed when no option is selected</td>
							</tr>
							<tr>
								<td><code>.value()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set selected value (single select)</td>
							</tr>
							<tr>
								<td><code>.values()</code></td>
								<td><code>Vec&lt;String&gt;</code></td>
								<td>Set selected values (multi select)</td>
							</tr>
							<tr>
								<td><code>.disabled()</code></td>
								<td><code>bool</code></td>
								<td>Enable/disable the select. Disabled selects cannot be opened</td>
							</tr>
							<tr>
								<td><code>.size()</code></td>
								<td><code>ComponentSize</code></td>
								<td>Set component size (XSmall to XLarge)</td>
							</tr>
							<tr>
								<td><code>.variant()</code></td>
								<td><code>SelectVariant</code></td>
								<td>Set visual variant (Default, Ghost, Outline)</td>
							</tr>
							<tr>
								<td><code>.dropdown_direction()</code></td>
								<td><code>DropdownDirection</code></td>
								<td>Set dropdown expansion direction (Down, Up, Auto)</td>
							</tr>
							<tr>
								<td><code>.dropdown_alignment()</code></td>
								<td><code>DropdownAlignment</code></td>
								<td>Set dropdown alignment (Left, Right, Center)</td>
							</tr>
							<tr>
								<td><code>.dropdown_width()</code></td>
								<td><code>DropdownWidth</code></td>
								<td>Set dropdown width (MatchTrigger, Fixed, MinWidth, MaxWidth)</td>
							</tr>
							<tr>
								<td><code>.multiple()</code></td>
								<td><code>bool</code></td>
								<td>Enable multiple selection mode</td>
							</tr>
							<tr>
								<td><code>.no_border()</code></td>
								<td>-</td>
								<td>Remove border (convenience method)</td>
							</tr>
							<tr>
								<td><code>.no_shadow()</code></td>
								<td>-</td>
								<td>Remove shadow (convenience method)</td>
							</tr>
							<tr>
								<td><code>.transparent()</code></td>
								<td>-</td>
								<td>Make background transparent (convenience method)</td>
							</tr>
							<tr>
								<td><code>.compact()</code></td>
								<td>-</td>
								<td>Use compact spacing for dropdown items (convenience method)</td>
							</tr>
						</tbody>
					</table>
				</div>
				
				<h3>Events</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Event</th>
								<th>Payload</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>SelectEvent::Changed</code></td>
								<td><code>String</code></td>
								<td>Emitted when single selection changes</td>
							</tr>
							<tr>
								<td><code>SelectEvent::MultiChanged</code></td>
								<td><code>Vec&lt;String&gt;</code></td>
								<td>Emitted when multiple selection changes</td>
							</tr>
						</tbody>
					</table>
				</div>
			</section>
			{:else if selectedComponent === 'combobox'}
			<section class="section">
				<h2>API Reference</h2>
				<h3>Methods</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Method</th>
								<th>Parameters</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>.options()</code></td>
								<td><code>Vec&lt;SelectOption&gt;</code></td>
								<td>Set available options for dropdown selection</td>
							</tr>
							<tr>
								<td><code>.option_groups()</code></td>
								<td><code>Vec&lt;SelectOptionGroup&gt;</code></td>
								<td>Set grouped options (hierarchical structure)</td>
							</tr>
							<tr>
								<td><code>.placeholder()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set placeholder text displayed when input is empty</td>
							</tr>
							<tr>
								<td><code>.value()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set selected value (matches with option and updates input_value)</td>
							</tr>
							<tr>
								<td><code>.input_value()</code></td>
								<td><code>impl Into&lt;String&gt;</code></td>
								<td>Set current input text value (for free text input)</td>
							</tr>
							<tr>
								<td><code>.disabled()</code></td>
								<td><code>bool</code></td>
								<td>Enable/disable the combobox. Disabled comboboxes cannot be interacted with</td>
							</tr>
							<tr>
								<td><code>.size()</code></td>
								<td><code>ComponentSize</code></td>
								<td>Set component size (XSmall to XLarge)</td>
							</tr>
							<tr>
								<td><code>.dropdown_direction()</code></td>
								<td><code>DropdownDirection</code></td>
								<td>Set dropdown expansion direction (Down, Up, Auto)</td>
							</tr>
							<tr>
								<td><code>.dropdown_alignment()</code></td>
								<td><code>DropdownAlignment</code></td>
								<td>Set dropdown alignment (Left, Right, Center)</td>
							</tr>
							<tr>
								<td><code>.dropdown_width()</code></td>
								<td><code>DropdownWidth</code></td>
								<td>Set dropdown width (MatchTrigger, Fixed, MinWidth, MaxWidth)</td>
							</tr>
							<tr>
								<td><code>.fixed_width()</code></td>
								<td><code>bool</code></td>
								<td>Control width behavior: <code>true</code> = fixed width (icon stays at right edge), <code>false</code> = dynamic width (default, icon moves with text)</td>
							</tr>
							<tr>
								<td><code>.no_border()</code></td>
								<td>-</td>
								<td>Remove border (convenience method)</td>
							</tr>
							<tr>
								<td><code>.no_shadow()</code></td>
								<td>-</td>
								<td>Remove shadow (convenience method)</td>
							</tr>
							<tr>
								<td><code>.transparent_background()</code></td>
								<td>-</td>
								<td>Make background transparent (convenience method)</td>
							</tr>
							<tr>
								<td><code>.compact()</code></td>
								<td>-</td>
								<td>Use compact spacing for dropdown items (convenience method)</td>
							</tr>
						</tbody>
					</table>
				</div>
				
				<h3>Events</h3>
				<div class="standards-table">
					<table>
						<thead>
							<tr>
								<th>Event</th>
								<th>Payload</th>
								<th>Description</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td><code>ComboboxEvent::Changed</code></td>
								<td><code>String</code></td>
								<td>Emitted when an option is selected from dropdown</td>
							</tr>
							<tr>
								<td><code>ComboboxEvent::InputChanged</code></td>
								<td><code>String</code></td>
								<td>Emitted when input value changes (user typing or programmatic change)</td>
							</tr>
						</tbody>
					</table>
				</div>
				
				<h3>Features</h3>
				<ul>
					<li><strong>Dual Mode:</strong> Combobox supports both preset option selection and free text input</li>
					<li><strong>Filtering:</strong> Options are automatically filtered as user types (only when actively typing)</li>
					<li><strong>Search:</strong> User can search by typing to filter options or enter custom values</li>
					<li><strong>Preselection:</strong> Use <code>.value()</code> to set a preselected option that matches with available options</li>
					<li><strong>Grouped Options:</strong> Organize options into groups using <code>.option_groups()</code></li>
					<li><strong>Flexible Width Control:</strong> Use <code>.fixed_width()</code> to control how the component behaves with borders</li>
					<li><strong>Auto Focus Blur:</strong> Automatically removes focus from input after selection for cleaner UI</li>
				</ul>

				<h3>Width Behavior</h3>
				<p>Combobox provides flexible width control to adapt to different design needs:</p>

				<h4>Dynamic Width Mode (Default: <code>fixed_width(false)</code>)</h4>
				<p>Best for borderless designs. The component width adjusts based on content:</p>
				<ul>
					<li>Text container expands/shrinks with input content</li>
					<li>Expand icon stays close to text (minimal spacing)</li>
					<li>Ideal for inline or compact layouts</li>
				</ul>

				<h4>Fixed Width Mode (<code>fixed_width(true)</code>)</h4>
				<p>Best for bordered designs. The component maintains a fixed width:</p>
				<ul>
					<li>Text container stays at content size</li>
					<li>Expand icon is pushed to the right edge of the border</li>
					<li>Border remains fixed regardless of content changes</li>
					<li>Provides consistent visual alignment in forms</li>
				</ul>

				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`// Dynamic width mode (borderless, default)
let combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Search...")
        .no_border()
        .options(vec![...])
        // fixed_width(false) is default
});

// Fixed width mode (with border)
let combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Select...")
        .fixed_width(true)  // Icon stays at right edge
        .options(vec![...])
});`} language="rust" />
				</div>

				<h3>Grouped Options Example</h3>
				<div class="code-container">
					<div class="code-header">
						<span class="code-label">Rust</span>
					</div>
					<CodeBlock code={`use fluix::{Combobox, SelectOption, SelectOptionGroup, DropdownDirection};

let combobox = cx.new(|cx| {
    Combobox::new(cx)
        .placeholder("Select a language...")
        .option_groups(vec![
            SelectOptionGroup::new("Frontend", vec![
                SelectOption::new("javascript", "JavaScript"),
                SelectOption::new("typescript", "TypeScript"),
                SelectOption::new("react", "React"),
            ]),
            SelectOptionGroup::new("Backend", vec![
                SelectOption::new("rust", "Rust"),
                SelectOption::new("python", "Python"),
                SelectOption::new("go", "Go"),
            ]),
        ])
        .dropdown_direction(DropdownDirection::Up)
        .fixed_width(true)
});`} language="rust" />
				</div>
			</section>
			{/if}
			
			{#if selectedComponent === 'button'}
			<!-- Design System Reference -->
			<section class="section">
				<h2>Design System Standards</h2>
				<p>The Button component follows Fluix's unified design system standards, including size, color, and theme. These standards apply to all components.</p>
				<p>See <button class="link-button" onclick={() => { selectedComponent = 'design-system'; selectedDesignSystemSection = 'overview'; }}>Design System</button> in the sidebar for complete size, color, and theme standards.</p>
			</section>
			{/if}
			
			<!-- Examples -->
			<section class="section">
				<h2>Examples</h2>
				{#each data.examples as example, index}
					{@const exampleId = getExampleId(selectedComponent, index)}
					{@const currentTab = getTab(selectedComponent, index)}
					<div class="example-card">
						<div class="example-header">
							<h3>{example.title}</h3>
							{#if example.description}
								<p class="example-description">{example.description}</p>
							{/if}
						</div>
						
						<div class="example-tabs">
							<button
								class="tab-button"
								class:active={currentTab === 'preview'}
								onclick={() => setTab(selectedComponent, index, 'preview')}
							>
								Preview
							</button>
							<button
								class="tab-button"
								class:active={currentTab === 'code'}
								onclick={() => setTab(selectedComponent, index, 'code')}
							>
								Code
							</button>
						</div>
						
						<div class="example-content">
							{#if currentTab === 'preview'}
								<div 
									class="preview-container" 
									class:all-icons-preview={selectedComponent === 'icon' && example.title === 'All Icons'}
									class:icon-preview={selectedComponent === 'icon' && example.title !== 'All Icons'}
									class:form-preview={selectedComponent === 'textinput' || (selectedComponent === 'checkbox' && example.title !== 'Sizes')}
									class:form-sizes-preview={selectedComponent === 'checkbox' && example.title === 'Sizes'}
									class:select-preview={selectedComponent === 'select' && example.title !== 'Disabled'}
									class:select-disabled-preview={selectedComponent === 'select' && example.title === 'Disabled'}
									class:combobox-preview={selectedComponent === 'combobox' && example.title !== 'Disabled'}
									class:combobox-disabled-preview={selectedComponent === 'combobox' && example.title === 'Disabled'}
									class:model-selector-preview={selectedComponent === 'model_selector'}
								>
									<img 
										src={getScreenshotPath(selectedComponent, example.screenshot)}
										alt="{example.title} preview"
										class="preview-image"
										class:all-icons-image={selectedComponent === 'icon' && example.title === 'All Icons'}
										class:icon-image={selectedComponent === 'icon' && example.title !== 'All Icons'}
										class:form-image={selectedComponent === 'textinput' || (selectedComponent === 'checkbox' && example.title !== 'Sizes')}
										class:form-sizes-image={selectedComponent === 'checkbox' && example.title === 'Sizes'}
										class:select-image={selectedComponent === 'select' && example.title !== 'Disabled'}
										class:select-disabled-image={selectedComponent === 'select' && example.title === 'Disabled'}
										class:combobox-image={selectedComponent === 'combobox' && example.title !== 'Disabled'}
										class:combobox-disabled-image={selectedComponent === 'combobox' && example.title === 'Disabled'}
										class:model-selector-image={selectedComponent === 'model_selector'}
										onerror={(e) => {
											const img = e.target as HTMLImageElement;
											img.style.display = 'none';
											const placeholder = img.nextElementSibling as HTMLElement;
											if (placeholder) placeholder.style.display = 'flex';
										}}
									/>
									<div class="preview-placeholder" style="display: none;">
										<div class="placeholder-content">
											<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
												<rect x="3" y="3" width="18" height="18" rx="2" stroke-dasharray="4 4"/>
												<path d="M8 12h8M12 8v8"/>
											</svg>
											<p>Screenshot not available</p>
										</div>
									</div>
								</div>
							{:else}
								<div class="code-container">
									<div class="code-header">
										<span class="code-label">Rust</span>
										<button 
											class="copy-button"
											onclick={() => copyCode(exampleId, example.code)}
											title="Copy code"
										>
											{#if copiedCode[exampleId]}
												<Check size={14} />
											{:else}
												<Copy size={14} />
											{/if}
										</button>
									</div>
									<CodeBlock code={example.code} language="rust" />
								</div>
							{/if}
						</div>
					</div>
				{/each}
			</section>
		{/if}
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
		margin-top: 2rem;
		margin-bottom: 1rem;
		color: var(--foreground);
	}

	.sidebar h2:first-of-type {
		margin-top: 0;
	}

	.nav-title {
		font-size: 1.125rem !important;
		font-weight: 600 !important;
		margin-top: 0;
		margin-bottom: 1rem;
		padding: 0.75rem 1rem;
		color: var(--foreground);
	}

	.component-nav {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		margin-bottom: 1rem;
	}

	.component-nav:last-child {
		margin-bottom: 0;
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

	.nav-item.sub-item {
		padding-left: 2rem;
		font-size: 0.8125rem;
		font-weight: 400;
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
		font-size: 2.5rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		color: var(--foreground);
	}

	.component-description {
		color: var(--muted-foreground);
		margin-bottom: 2rem;
		font-size: 1.125rem;
		line-height: 1.6;
	}

	.section {
		margin-bottom: 3rem;
	}

	.section h2 {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: var(--foreground);
	}

	.section p {
		color: var(--muted-foreground);
		margin-bottom: 0.75rem;
		line-height: 1.6;
	}

	.section code {
		background: var(--secondary);
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.875rem;
		color: var(--foreground);
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
	}

	.section a.link {
		color: var(--primary);
		text-decoration: none;
		font-weight: 500;
		border-bottom: 1px solid transparent;
		transition: border-color 0.2s;
	}

	.section a.link:hover {
		border-bottom-color: var(--primary);
	}

	.link-button {
		background: none;
		border: none;
		padding: 0;
		color: var(--primary);
		text-decoration: none;
		font-weight: 500;
		font-size: inherit;
		font-family: inherit;
		cursor: pointer;
		border-bottom: 1px solid transparent;
		transition: border-color 0.2s;
	}

	.link-button:hover {
		border-bottom-color: var(--primary);
	}

	.code-container {
		background: var(--card);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		overflow: hidden;
		margin-top: 0.75rem;
	}

	.code-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: var(--secondary);
		border-bottom: 1px solid var(--border);
	}

	.code-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--muted-foreground);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.copy-button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.25rem;
		border: none;
		background: transparent;
		color: var(--muted-foreground);
		cursor: pointer;
		border-radius: 0.25rem;
		transition: all 0.2s;
	}

	.copy-button:hover {
		background: var(--accent);
		color: var(--foreground);
	}

	.code-block {
		margin: 0;
		padding: 1rem;
		background: var(--card);
		border: none;
		overflow-x: auto;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 0.75rem;
		line-height: 1.6;
		color: var(--foreground);
	}

	.code-block code {
		background: none;
		padding: 0;
		color: inherit;
		font-size: inherit;
		font-family: inherit;
		display: block;
		padding-left: 1rem;
		text-indent: 0;
	}

	/* Example Cards */
	.example-card {
		background: var(--card);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		margin-bottom: 2rem;
		overflow: hidden;
	}

	.example-header {
		padding: 1.5rem 1.5rem 1rem 1.5rem;
	}

	.example-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--foreground);
	}

	.example-description {
		color: var(--muted-foreground);
		font-size: 0.875rem;
		margin: 0;
	}

	.example-tabs {
		display: flex;
		border-bottom: 1px solid var(--border);
		background: var(--secondary);
	}

	.tab-button {
		padding: 0.75rem 1.5rem;
		border: none;
		background: transparent;
		color: var(--muted-foreground);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		transition: all 0.2s;
	}

	.tab-button:hover {
		color: var(--foreground);
		background: var(--accent);
	}

	.tab-button.active {
		color: var(--foreground);
		border-bottom-color: var(--primary);
	}

	.example-content {
		padding: 0;
	}

	.preview-container {
		background: var(--background);
		padding: 0.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 90px;
		position: relative;
	}

	.preview-image {
		max-height: 50px;
		width: auto;
		height: auto;
		border-radius: 0.25rem;
		object-fit: contain;
		max-width: 100%;
	}

	.preview-container.icon-preview {
		padding: 1.5rem;
		min-height: 150px;
	}

	.preview-image.icon-image {
		max-height: 60px;
		height: auto;
	}

	.preview-container.form-preview {
		padding: 1.5rem;
		min-height: 150px;
	}

	.preview-image.form-image {
		max-height: 50px !important;
		height: auto !important;
	}

	.preview-container.form-sizes-preview {
		padding: 2rem;
		min-height: auto;
		overflow-x: auto;
		justify-content: flex-start;
	}

	.preview-image.form-sizes-image {
		max-height: none !important;
		height: auto !important;
	}

	.preview-container.select-preview {
		padding: 1.5rem;
		min-height: 200px;
	}

	.preview-image.select-image {
		max-height: 250px !important;
		height: auto !important;
	}

	.preview-container.select-disabled-preview {
		padding: 1.5rem;
		min-height: 200px;
	}

	.preview-image.select-disabled-image {
		max-height: 60px !important;
		height: auto !important;
	}

	.preview-container.combobox-preview {
		padding: 1.5rem;
		min-height: 200px;
	}

	.preview-image.combobox-image {
		max-height: 250px !important;
		height: auto !important;
	}

	.preview-container.combobox-disabled-preview {
		padding: 1.5rem;
		min-height: 200px;
	}

	.preview-image.combobox-disabled-image {
		max-height: 60px !important;
		height: auto !important;
	}

	.preview-container.model-selector-preview {
		padding: 1.5rem;
		min-height: 200px;
	}

	.preview-image.model-selector-image {
		max-height: 300px !important;
		height: auto !important;
	}

	.preview-container.all-icons-preview {
		padding: 2rem;
		min-height: auto;
		overflow-x: auto;
		justify-content: flex-start;
	}

	.preview-image.all-icons-image {
		max-height: none;
		height: auto;
	}

	.preview-placeholder {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 8px;
		color: var(--text_muted);
		font-size: 0.875rem;
	}

	.placeholder-content {
		text-align: center;
		color: var(--muted-foreground);
	}

	.placeholder-content svg {
		margin-bottom: 0.5rem;
	}

	.placeholder-content p {
		margin: 0;
		font-size: 0.875rem;
	}

	/* Standards Table */
	.standards-table {
		margin: 1.5rem 0;
		overflow-x: auto;
	}

	.standards-table table {
		width: 100%;
		border-collapse: collapse;
		background: var(--card);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.standards-table thead {
		background: var(--secondary);
	}

	.standards-table th {
		padding: 0.75rem 1rem;
		text-align: left;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--foreground);
		border-bottom: 1px solid var(--border);
	}

	.standards-table td {
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		color: var(--muted-foreground);
		border-bottom: 1px solid var(--border);
	}

	.standards-table tbody tr:last-child td {
		border-bottom: none;
	}

	.standards-table tbody tr:hover {
		background: var(--secondary);
	}

	.standards-table code {
		background: var(--secondary);
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.875rem;
		color: var(--foreground);
	}

	.color-cell {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.color-swatch {
		display: inline-block;
		width: 1.25rem;
		height: 1.25rem;
		border-radius: 0.25rem;
		border: 1px solid var(--border);
		flex-shrink: 0;
	}

	/* Components Grid */
	.components-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.component-card-link {
		background: var(--card);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 1.5rem;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		min-height: 140px;
	}

	.component-card-link:hover {
		border-color: var(--primary);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	.component-card-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0 0 0.5rem 0;
		color: var(--foreground);
	}

	.component-card-description {
		color: var(--muted-foreground);
		font-size: 0.875rem;
		line-height: 1.5;
		margin: 0;
	}

	.component-card-footer {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid var(--border);
	}

	.component-card-link-text {
		color: var(--primary);
		font-size: 0.875rem;
		font-weight: 500;
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

		.content-header h1 {
			font-size: 2rem;
		}
	}

	/* Overview Page Styles */
	.section-description {
		color: var(--text_muted);
		margin-bottom: 1.5rem;
		font-size: 1rem;
		line-height: 1.6;
	}

	.overview-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 1.5rem;
		margin-top: 1.5rem;
	}

	.overview-card {
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.2s ease;
		position: relative;
	}

	.overview-card:hover {
		border-color: var(--primary);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	.clickable-card {
		cursor: pointer;
	}

	.clickable-card:hover .card-arrow {
		transform: translateX(4px);
	}

	.overview-card h3 {
		margin: 0 0 0.75rem 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text);
	}

	.overview-card p {
		margin: 0 0 1rem 0;
		color: var(--text_muted);
		line-height: 1.5;
		font-size: 0.875rem;
	}

	.card-arrow {
		position: absolute;
		bottom: 1.5rem;
		right: 1.5rem;
		color: var(--primary);
		font-size: 1.25rem;
		font-weight: bold;
		transition: transform 0.2s ease;
	}

	.card-button {
		background: var(--primary);
		color: white;
		border: none;
		border-radius: 6px;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.card-button:hover {
		background: var(--primary_hover);
	}
</style>
