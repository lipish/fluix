<script lang="ts">
	import { highlightCode, detectLanguage } from '$lib/utils/codeHighlight';
	import { onMount } from 'svelte';
	
	interface Props {
		code: string;
		language?: string;
		filename?: string;
	}
	
	let { code, language, filename }: Props = $props();
	
	let codeElement: HTMLElement;
	let highlightedCode = $state('');
	let detectedLang = $state('rust');
	
	$effect(() => {
		detectedLang = language || detectLanguage(filename, code);
		highlightedCode = highlightCode(code, detectedLang);
		if (codeElement) {
			codeElement.className = `language-${detectedLang}`;
			codeElement.innerHTML = highlightedCode;
		}
	});
</script>

<pre class="code-block"><code bind:this={codeElement}></code></pre>

<style>
	.code-block {
		font-size: 0.75rem !important;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Courier New', monospace !important;
	}
	
	:global(.code-block code) {
		background: none !important;
		padding: 0 !important;
		padding-left: 1rem !important;
		color: inherit !important;
		font-size: inherit !important;
		font-family: inherit !important;
		display: block !important;
		text-indent: 0 !important;
	}
</style>
