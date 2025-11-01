import{d as b,a as f,k as se,f as _,o as A}from"../chunks/DChJZV8W.js";import{b as Q,E as ne,N as oe,a$ as le,h as M,a2 as ie,J as ce,a4 as K,d as V,c as Z,G as ve,b0 as de,aE as ue,u as fe,f as he,ac as g,b1 as pe,w as y,v as me,z as c,n as h,y as _e,A as v,aj as $,aD as j,b2 as ye,X as ge,B as d,x as U,b3 as $e}from"../chunks/CBCph-ZT.js";import{B as be,l as k,s as w}from"../chunks/CywfKMh-.js";import{I as x,s as q,r as ke,a as we}from"../chunks/BpY2q9gV.js";import{o as xe}from"../chunks/DgvRZuL8.js";import{s as qe}from"../chunks/BA_B2HV_.js";import{b as Ce,i as J}from"../chunks/CppJ630h.js";import"../chunks/DoGbsmQH.js";function Ee(e,r,...n){var o=new be(e);Q(()=>{const a=r()??null;o.ensure(a,a&&(t=>a(t,...n)))},ne)}function Ne(e,r){let n=null,o=M;var a;if(M){n=ve;for(var t=ie(document.head);t!==null&&(t.nodeType!==ce||t.data!==e);)t=K(t);if(t===null)V(!1);else{var l=K(t);t.remove(),Z(l)}}M||(a=document.head.appendChild(oe()));try{Q(()=>r(a),le)}finally{o&&(V(!0),Z(n))}}function Me(e,r,n=r){var o=new WeakSet;de(e,"input",async a=>{var t=a?e.defaultValue:e.value;if(t=B(e)?D(t):t,n(t),g!==null&&o.add(g),await ue(),t!==(t=r())){var l=e.selectionStart,i=e.selectionEnd,m=e.value.length;if(e.value=t??"",i!==null){var p=e.value.length;l===i&&i===m&&p>m?(e.selectionStart=p,e.selectionEnd=p):(e.selectionStart=l,e.selectionEnd=Math.min(i,p))}}}),(M&&e.defaultValue!==e.value||fe(r)==null&&e.value)&&(n(B(e)?D(e.value):e.value),g!==null&&o.add(g)),he(()=>{var a=r();if(e===document.activeElement){var t=pe??g;if(o.has(t))return}B(e)&&a===D(e.value)||e.type==="date"&&!a&&!e.value||a!==e.value&&(e.value=a??"")})}function B(e){var r=e.type;return r==="number"||r==="range"}function D(e){return e===""?null:+e}const Se=!0,ze=!1,Xe=Object.freeze(Object.defineProperty({__proto__:null,prerender:Se,ssr:ze},Symbol.toStringTag,{value:"Module"})),Fe="data:image/svg+xml,%3csvg%20xmlns='http://www.w3.org/2000/svg'%20fill='none'%20viewBox='0%200%2024%2024'%20stroke-width='1.5'%20stroke='currentColor'%20class='size-6'%3e%3cpath%20stroke-linecap='round'%20stroke-linejoin='round'%20d='M2.25%207.125C2.25%206.504%202.754%206%203.375%206h6c.621%200%201.125.504%201.125%201.125v3.75c0%20.621-.504%201.125-1.125%201.125h-6a1.125%201.125%200%200%201-1.125-1.125v-3.75ZM14.25%208.625c0-.621.504-1.125%201.125-1.125h5.25c.621%200%201.125.504%201.125%201.125v8.25c0%20.621-.504%201.125-1.125%201.125h-5.25a1.125%201.125%200%200%201-1.125-1.125v-8.25ZM3.75%2016.125c0-.621.504-1.125%201.125-1.125h5.25c.621%200%201.125.504%201.125%201.125v2.25c0%20.621-.504%201.125-1.125%201.125h-5.25a1.125%201.125%200%200%201-1.125-1.125v-2.25Z'%20/%3e%3c/svg%3e";function Pe(e,r){const n=k(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v0.552.0 - ISC
 *
 * ISC License
 *
 * Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2023 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2025.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The MIT License (MIT) (for portions derived from Feather)
 *
 * Copyright (c) 2013-2023 Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const o=[["path",{d:"m6 9 6 6 6-6"}]];x(e,w({name:"chevron-down"},()=>n,{get iconNode(){return o},children:(a,t)=>{var l=b(),i=y(l);q(i,r,"default",{}),f(a,l)},$$slots:{default:!0}}))}function Te(e,r){const n=k(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v0.552.0 - ISC
 *
 * ISC License
 *
 * Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2023 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2025.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The MIT License (MIT) (for portions derived from Feather)
 *
 * Copyright (c) 2013-2023 Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const o=[["circle",{cx:"12",cy:"12",r:"1"}],["circle",{cx:"19",cy:"12",r:"1"}],["circle",{cx:"5",cy:"12",r:"1"}]];x(e,w({name:"ellipsis"},()=>n,{get iconNode(){return o},children:(a,t)=>{var l=b(),i=y(l);q(i,r,"default",{}),f(a,l)},$$slots:{default:!0}}))}function L(e,r){const n=k(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v0.552.0 - ISC
 *
 * ISC License
 *
 * Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2023 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2025.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The MIT License (MIT) (for portions derived from Feather)
 *
 * Copyright (c) 2013-2023 Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const o=[["path",{d:"M15 3h6v6"}],["path",{d:"M10 14 21 3"}],["path",{d:"M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"}]];x(e,w({name:"external-link"},()=>n,{get iconNode(){return o},children:(a,t)=>{var l=b(),i=y(l);q(i,r,"default",{}),f(a,l)},$$slots:{default:!0}}))}function Ae(e,r){const n=k(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v0.552.0 - ISC
 *
 * ISC License
 *
 * Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2023 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2025.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The MIT License (MIT) (for portions derived from Feather)
 *
 * Copyright (c) 2013-2023 Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const o=[["path",{d:"M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"}],["path",{d:"M9 18c-4.51 2-5-2-7-2"}]];x(e,w({name:"github"},()=>n,{get iconNode(){return o},children:(a,t)=>{var l=b(),i=y(l);q(i,r,"default",{}),f(a,l)},$$slots:{default:!0}}))}function je(e,r){const n=k(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v0.552.0 - ISC
 *
 * ISC License
 *
 * Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2023 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2025.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The MIT License (MIT) (for portions derived from Feather)
 *
 * Copyright (c) 2013-2023 Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const o=[["path",{d:"m21 21-4.34-4.34"}],["circle",{cx:"11",cy:"11",r:"8"}]];x(e,w({name:"search"},()=>n,{get iconNode(){return o},children:(a,t)=>{var l=b(),i=y(l);q(i,r,"default",{}),f(a,l)},$$slots:{default:!0}}))}var Be=_('<div class="dropdown-menu svelte-rfuq4y"><a href="/docs/tutorials" class="dropdown-item svelte-rfuq4y">Tutorials</a> <a href="/docs/faq" class="dropdown-item svelte-rfuq4y">FAQ</a> <a href="https://github.com/lipish/fluix/releases" target="_blank" rel="noopener noreferrer" class="dropdown-item svelte-rfuq4y">Releases <!></a></div>'),De=_('<span class="github-stars svelte-rfuq4y"> </span>'),Ge=_('<span class="github-stars svelte-rfuq4y">GitHub</span>'),He=_('<nav class="navbar svelte-rfuq4y"><div class="nav-container svelte-rfuq4y"><a href="/" class="logo-link svelte-rfuq4y"><div class="logo-icon-wrapper svelte-rfuq4y"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="8" height="8" fill="currentColor" stroke="none"></rect><rect x="13" y="3" width="8" height="8" stroke="currentColor" fill="none"></rect><rect x="3" y="13" width="8" height="8" stroke="currentColor" fill="none"></rect><rect x="13" y="13" width="8" height="8" fill="currentColor" stroke="none"></rect></svg></div> <span class="logo-text svelte-rfuq4y">Fluix Component</span></a> <div class="search-container svelte-rfuq4y"><div class="search-input-wrapper svelte-rfuq4y"><!> <input type="text" class="search-input svelte-rfuq4y" placeholder="Search"/> <div class="search-shortcut svelte-rfuq4y"><span class="shortcut-key svelte-rfuq4y">⌘</span> <span class="shortcut-key svelte-rfuq4y">K</span></div></div></div> <div class="nav-links svelte-rfuq4y"><a href="/" class="nav-link svelte-rfuq4y">Home</a> <a href="/docs/getting-started" class="nav-link svelte-rfuq4y">Getting Started</a> <a href="/docs/components" class="nav-link svelte-rfuq4y">Components</a> <a href="https://docs.rs/fluix" target="_blank" rel="noopener noreferrer" class="nav-link svelte-rfuq4y">API Doc <!></a> <div class="nav-link resources-dropdown svelte-rfuq4y"><span>Resources</span> <!> <!></div></div> <a href="https://github.com/lipish/fluix" target="_blank" rel="noopener noreferrer" class="github-link svelte-rfuq4y"><!> <!></a> <button class="more-button svelte-rfuq4y" aria-label="More options"><!></button></div></nav>');function Ie(e,r){me(r,!0);let n=j(""),o=j(!1),a=j(null),t;xe(async()=>{try{const s=await fetch("https://api.github.com/repos/lipish/fluix");if(s.ok){const u=await s.json();$(a,u.stargazers_count,!0)}}catch(s){console.error("Failed to fetch GitHub stars:",s),$(a,null)}});function l(s){s.key==="Enter"&&h(n).trim()&&console.log("Search:",h(n))}function i(s){(s.metaKey||s.ctrlKey)&&s.key==="k"&&(s.preventDefault(),t?.focus())}var m=He();A("keydown",ye,i);var p=v(m),S=c(v(p),2),G=v(S),H=v(G);je(H,{class:"search-icon"});var C=c(H,2);ke(C),C.__keydown=l,Ce(C,s=>t=s,()=>t),ge(2),d(G),d(S);var z=c(S,2),F=c(v(z),6),W=c(v(F));L(W,{class:"external-icon"}),d(F);var E=c(F,2),I=c(v(E),2);Pe(I,{class:"chevron-icon"});var X=c(I,2);{var Y=s=>{var u=Be(),N=c(v(u),4),T=c(v(N));L(T,{class:"external-icon-small"}),d(N),d(u),f(s,u)};J(X,s=>{h(o)&&s(Y)})}d(E),d(z);var P=c(z,2),O=v(P);Ae(O,{class:"github-icon"});var ee=c(O,2);{var ae=s=>{var u=De(),N=v(u,!0);d(u),U(T=>qe(N,T),[()=>h(a)>=1e3?(h(a)/1e3).toFixed(1)+"k":h(a)]),f(s,u)},te=s=>{var u=Ge();f(s,u)};J(ee,s=>{h(a)!==null?s(ae):s(te,!1)})}d(P);var R=c(P,2),re=v(R);Te(re,{}),d(R),d(p),d(m),Me(C,()=>h(n),s=>$(n,s)),A("mouseenter",E,()=>$(o,!0)),A("mouseleave",E,()=>$(o,!1)),f(e,m),_e()}se(["keydown"]);var Oe=_('<link rel="icon"/>'),Re=_('<!> <main class="main-content svelte-12qhfyh"><!></main>',1);function Ye(e,r){var n=Re();Ne("12qhfyh",l=>{var i=Oe();$e.title="Fluix Component - Rust UI Component Library",U(()=>we(i,"href",Fe)),f(l,i)});var o=y(n);Ie(o,{});var a=c(o,2),t=v(a);Ee(t,()=>r.children),d(a),f(e,n)}export{Ye as component,Xe as universal};
