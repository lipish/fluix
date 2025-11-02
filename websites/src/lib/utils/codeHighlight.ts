import Prism from 'prismjs';
import 'prismjs/components/prism-rust';
import 'prismjs/components/prism-toml';
import 'prismjs/components/prism-bash';
import 'prismjs/components/prism-markdown';

/**
 * 高亮代码块
 * @param code 代码字符串
 * @param language 语言类型 (rust, toml, bash, etc.)
 * @returns 高亮后的 HTML 字符串
 */
export function highlightCode(code: string, language: string = 'rust'): string {
	// 确保语言已加载
	const lang = language.toLowerCase();
	
	// 如果语言不支持，转义 HTML 并返回
	if (!Prism.languages[lang]) {
		// 简单转义 HTML 字符
		return code
			.replace(/&/g, '&amp;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;')
			.replace(/"/g, '&quot;')
			.replace(/'/g, '&#39;');
	}
	
	return Prism.highlight(code, Prism.languages[lang], lang);
}

/**
 * 检测代码语言（根据文件名或代码内容推断）
 */
export function detectLanguage(filename?: string, code?: string): string {
	if (filename) {
		const ext = filename.split('.').pop()?.toLowerCase();
		switch (ext) {
			case 'rs':
				return 'rust';
			case 'toml':
				return 'toml';
			case 'sh':
			case 'bash':
				return 'bash';
			case 'md':
				return 'markdown';
			default:
				return 'rust'; // 默认为 Rust
		}
	}
	
	// 根据代码内容推断
	if (code) {
		if (code.includes('use fluix') || code.includes('use gpui') || code.includes('let ') && code.includes('cx.new')) {
			return 'rust';
		}
		if (code.includes('[package]') || code.startsWith('fluix =')) {
			return 'toml';
		}
	}
	
	return 'rust';
}
