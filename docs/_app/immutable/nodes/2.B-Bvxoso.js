import{c as he,l as ge,m as _e,a as P,d as U,f as me}from"../chunks/DOevuEX7.js";import{i as pe}from"../chunks/CQiARHzH.js";import{Z as ie,h as N,_ as Q,o as L,w as $e,aL as be,a0 as ye,a9 as ae,a7 as F,al as Y,e as z,$ as Ne,aq as we,b as j,s as xe,c as Ee,a6 as ke,a as le,aN as Ce,j as te,ao as oe,aA as Me,aR as Te,r as de,p as Ae,aS as Pe,aT as Ie,aU as J,d as ve,aj as Se,aV as ze,an as Re,aW as De,i as ce,aX as Ue,ac as He,aY as Ve,ah as We,aD as Be,M as Oe,Q as qe,g as re,J as G,R as _,U as A,T as m,N as D,aQ as Ge,aZ as Le,a8 as C}from"../chunks/DA4lL2sB.js";import{l as R,p as B,s as O}from"../chunks/BopED7Rw.js";import{B as Fe}from"../chunks/Dlo5nQhW.js";import{a as se}from"../chunks/DZn_FljI.js";function Ye(i,a){return a}function Ze(i,a,e){for(var s=i.items,o=[],d=a.length,t=0;t<d;t++)ze(a[t].e,o,!0);var r=d>0&&o.length===0&&e!==null;if(r){var f=e.parentNode;Re(f),f.append(e),s.clear(),T(i,a[0].prev,a[d-1].next)}De(o,()=>{for(var $=0;$<d;$++){var h=a[$];r||(s.delete(h.k),T(i,h.prev,h.next)),ve(h.e,!r)}})}function Qe(i,a,e,s,o,d=null){var t=i,r={flags:a,items:new Map,first:null};N&&Q();var f=null,$=!1,h=new Map,y=$e(()=>{var n=e();return Me(n)?n:n==null?[]:oe(n)}),l,g;function v(){Xe(g,l,r,h,t,o,a,s,e),d!==null&&(l.length===0?f?de(f):f=j(()=>d(t)):f!==null&&Ae(f,()=>{f=null}))}ie(()=>{g??=ce,l=L(y);var n=l.length;if($&&n===0)return;$=n===0;let p=!1;if(N){var k=be(t)===ye;k!==(n===0)&&(t=ae(),F(t),Y(!1),p=!0)}if(N){for(var w=null,c,u=0;u<n;u++){if(z.nodeType===Ne&&z.data===we){t=z,p=!0,Y(!1);break}var b=l[u],x=s(b,u);c=ee(z,r,w,null,b,x,u,o,a,e),r.items.set(x,c),w=c}n>0&&F(ae())}if(N)n===0&&d&&(f=j(()=>d(t)));else if(xe()){var I=new Set,M=Ee;for(u=0;u<n;u+=1){b=l[u],x=s(b,u);var E=r.items.get(x)??h.get(x);E?ue(E,b,u):(c=ee(null,r,null,null,b,x,u,o,a,e,!0),h.set(x,c)),I.add(x)}for(const[V,S]of r.items)I.has(V)||M.skipped_effects.add(S.e);M.oncommit(v)}else v();p&&Y(!0),L(y)}),N&&(t=z)}function Xe(i,a,e,s,o,d,t,r,f){var $=a.length,h=e.items,y=e.first,l=y,g,v=null,n=[],p=[],k,w,c,u;for(u=0;u<$;u+=1){if(k=a[u],w=r(k,u),c=h.get(w),c===void 0){var b=s.get(w);if(b!==void 0){s.delete(w),h.set(w,b);var x=v?v.next:l;T(e,v,b),T(e,b,x),K(b,x,o),v=b}else{var I=l?l.e.nodes_start:o;v=ee(I,e,v,v===null?e.first:v.next,k,w,u,d,t,f)}h.set(w,v),n=[],p=[],l=v.next;continue}if(ue(c,k,u),(c.e.f&J)!==0&&de(c.e),c!==l){if(g!==void 0&&g.has(c)){if(n.length<p.length){var M=p[0],E;v=M.prev;var V=n[0],S=n[n.length-1];for(E=0;E<n.length;E+=1)K(n[E],M,o);for(E=0;E<p.length;E+=1)g.delete(p[E]);T(e,V.prev,S.next),T(e,v,V),T(e,S,M),l=M,v=S,u-=1,n=[],p=[]}else g.delete(c),K(c,l,o),T(e,c.prev,c.next),T(e,c,v===null?e.first:v.next),T(e,v,c),v=c;continue}for(n=[],p=[];l!==null&&l.k!==w;)(l.e.f&J)===0&&(g??=new Set).add(l),p.push(l),l=l.next;if(l===null)continue;c=l}n.push(c),v=c,l=c.next}if(l!==null||g!==void 0){for(var W=g===void 0?[]:oe(g);l!==null;)(l.e.f&J)===0&&W.push(l),l=l.next;var Z=W.length;if(Z>0){var X=null;Ze(e,W,X)}}i.first=e.first&&e.first.e,i.last=v&&v.e;for(var fe of s.values())ve(fe.e);s.clear()}function ue(i,a,e,s){ke(i.v,a),i.i=e}function ee(i,a,e,s,o,d,t,r,f,$,h){var y=(f&Pe)!==0,l=(f&Ie)===0,g=y?l?Ce(o,!1,!1):te(o):o,v=(f&Te)===0?t:te(t),n={i:v,v:g,k:d,a:null,e:null,prev:e,next:s};try{if(i===null){var p=document.createDocumentFragment();p.append(i=le())}return n.e=j(()=>r(i,g,v,$),N),n.e.prev=e&&e.e,n.e.next=s&&s.e,e===null?h||(a.first=n):(e.next=n,e.e.next=n.e),s!==null&&(s.prev=n,s.e.prev=n.e),n}finally{}}function K(i,a,e){for(var s=i.next?i.next.e.nodes_start:e,o=a?a.e.nodes_start:e,d=i.e.nodes_start;d!==null&&d!==s;){var t=Se(d);o.before(d),d=t}}function T(i,a,e){a===null?i.first=e:(a.next=e,a.e.next=e&&e.e),e!==null&&(e.prev=a,e.e.prev=a&&a.e)}function H(i,a,e,s,o){N&&Q();var d=a.$$slots?.[e],t=!1;d===!0&&(d=a.children,t=!0),d===void 0||d(i,t?()=>s:s)}function Je(i,a,e,s,o,d){let t=N;N&&Q();var r=null;N&&z.nodeType===Ue&&(r=z,Q());var f=N?z:i,$=new Fe(f,!1);ie(()=>{const h=a()||null;var y=Ve;if(h===null){$.ensure(null,null);return}return $.ensure(h,l=>{if(h){if(r=N?r:document.createElementNS(y,h),he(r,r),s){N&&ge(h)&&r.append(document.createComment(""));var g=N?We(r):r.appendChild(le());N&&(g===null?Y(!1):F(g)),s(r,g)}ce.nodes_end=r,l.before(r)}N&&F(l)}),()=>{}},He),Be(()=>{}),t&&(Y(!0),F(f))}/**
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
 */const Ke={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"};var je=_e("<svg><!><!></svg>");function q(i,a){const e=R(a,["children","$$slots","$$events","$$legacy"]),s=R(e,["name","color","size","strokeWidth","absoluteStrokeWidth","iconNode"]);Oe(a,!1);let o=B(a,"name",8,void 0),d=B(a,"color",8,"currentColor"),t=B(a,"size",8,24),r=B(a,"strokeWidth",8,2),f=B(a,"absoluteStrokeWidth",8,!1),$=B(a,"iconNode",24,()=>[]);const h=(...v)=>v.filter((n,p,k)=>!!n&&k.indexOf(n)===p).join(" ");pe();var y=je();se(y,(v,n)=>({...Ke,...s,width:t(),height:t(),stroke:d(),"stroke-width":v,class:n}),[()=>(G(f()),G(r()),G(t()),re(()=>f()?Number(r())*24/Number(t()):r())),()=>(G(o()),G(e),re(()=>h("lucide-icon","lucide",o()?`lucide-${o()}`:"",e.class)))]);var l=_(y);Qe(l,1,$,Ye,(v,n)=>{var p=Ge(()=>Le(L(n),2));let k=()=>L(p)[0],w=()=>L(p)[1];var c=U(),u=D(c);Je(u,k,!0,(b,x)=>{se(b,()=>({...w()}))}),P(v,c)});var g=A(l);H(g,a,"default",{}),m(y),P(i,y),qe()}function ne(i,a){const e=R(a,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M10 22V7a1 1 0 0 0-1-1H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5a1 1 0 0 0-1-1H2"}],["rect",{x:"14",y:"2",width:"8",height:"8",rx:"1"}]];q(i,O({name:"blocks"},()=>e,{get iconNode(){return s},children:(o,d)=>{var t=U(),r=D(t);H(r,a,"default",{}),P(o,t)},$$slots:{default:!0}}))}function ea(i,a){const e=R(a,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M3 3v16a2 2 0 0 0 2 2h16"}],["path",{d:"M18 17V9"}],["path",{d:"M13 17V5"}],["path",{d:"M8 17v-3"}]];q(i,O({name:"chart-column"},()=>e,{get iconNode(){return s},children:(o,d)=>{var t=U(),r=D(t);H(r,a,"default",{}),P(o,t)},$$slots:{default:!0}}))}function aa(i,a){const e=R(a,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M12 22a1 1 0 0 1 0-20 10 9 0 0 1 10 9 5 5 0 0 1-5 5h-2.25a1.75 1.75 0 0 0-1.4 2.8l.3.4a1.75 1.75 0 0 1-1.4 2.8z"}],["circle",{cx:"13.5",cy:"6.5",r:".5",fill:"currentColor"}],["circle",{cx:"17.5",cy:"10.5",r:".5",fill:"currentColor"}],["circle",{cx:"6.5",cy:"12.5",r:".5",fill:"currentColor"}],["circle",{cx:"8.5",cy:"7.5",r:".5",fill:"currentColor"}]];q(i,O({name:"palette"},()=>e,{get iconNode(){return s},children:(o,d)=>{var t=U(),r=D(t);H(r,a,"default",{}),P(o,t)},$$slots:{default:!0}}))}function ta(i,a){const e=R(a,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["rect",{width:"18",height:"18",x:"3",y:"3",rx:"2"}],["path",{d:"M3 9h18"}],["path",{d:"M9 21V9"}]];q(i,O({name:"panels-top-left"},()=>e,{get iconNode(){return s},children:(o,d)=>{var t=U(),r=D(t);H(r,a,"default",{}),P(o,t)},$$slots:{default:!0}}))}function ra(i,a){const e=R(a,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"m10 9-3 3 3 3"}],["path",{d:"m14 15 3-3-3-3"}],["rect",{x:"3",y:"3",width:"18",height:"18",rx:"2"}]];q(i,O({name:"square-code"},()=>e,{get iconNode(){return s},children:(o,d)=>{var t=U(),r=D(t);H(r,a,"default",{}),P(o,t)},$$slots:{default:!0}}))}function sa(i,a){const e=R(a,["children","$$slots","$$events","$$legacy"]);/**
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
 */const s=[["path",{d:"M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"}]];q(i,O({name:"zap"},()=>e,{get iconNode(){return s},children:(o,d)=>{var t=U(),r=D(t);H(r,a,"default",{}),P(o,t)},$$slots:{default:!0}}))}var na=me(`<div class="banner svelte-1uha8ag"><h1 class="svelte-1uha8ag">Fluix Component</h1> <div class="banner-description svelte-1uha8ag">Rust UI components for building fantastic cross-platform desktop applications by using GPUI 0.2.</div> <div class="actions svelte-1uha8ag"><a href="/docs/getting-started" class="btn-primary svelte-1uha8ag">Get Started</a> <a href="/docs/components" class="svelte-1uha8ag"><!> Components</a></div></div> <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 mb-12"><div class="feature-card svelte-1uha8ag"><h3 class="svelte-1uha8ag"><div class="icon bg-green-500 dark:bg-green-700 svelte-1uha8ag"><!></div> <div>46+ Components</div></h3> <div>Comprehensive library of cross-platform desktop UI components for building feature-rich applications.</div></div> <div class="feature-card svelte-1uha8ag"><h3 class="svelte-1uha8ag"><div class="icon bg-blue-500 dark:bg-blue-700 svelte-1uha8ag"><!></div> <div>High Performance</div></h3> <div>GPU-accelerated rendering powered by GPUI for smooth user experience.</div></div> <div class="feature-card svelte-1uha8ag"><h3 class="svelte-1uha8ag"><div class="icon bg-red-500 dark:bg-red-700 svelte-1uha8ag"><!></div> <div>Themeable</div></h3> <div>Built-in theme system with flexible theme and style customization support.</div></div> <div class="feature-card svelte-1uha8ag"><h3 class="svelte-1uha8ag"><div class="icon bg-yellow-500 dark:bg-yellow-700 svelte-1uha8ag"><!></div> <div>Type Safe</div></h3> <div>Leverage Rust's type system for compile-time type safety guarantees.</div></div> <div class="feature-card svelte-1uha8ag"><h3 class="svelte-1uha8ag"><div class="icon bg-pink-500 dark:bg-pink-700 svelte-1uha8ag"><!></div> <div>Easy to Use</div></h3> <div>Simple and consistent API design for quick onboarding.</div></div> <div class="feature-card svelte-1uha8ag"><h3 class="svelte-1uha8ag"><div class="icon bg-cyan-500 dark:bg-cyan-700 svelte-1uha8ag"><!></div> <div>Well Documented</div></h3> <div>Comprehensive API documentation, tutorials, and example code.</div></div></div>`,1);function ua(i){var a=na(),e=D(a),s=A(_(e),4),o=A(_(s),2),d=_(o);ne(d,{}),C(),m(o),m(s),m(e);var t=A(e,2),r=_(t),f=_(r),$=_(f),h=_($);ne(h,{}),m($),C(2),m(f),C(2),m(r);var y=A(r,2),l=_(y),g=_(l),v=_(g);sa(v,{}),m(g),C(2),m(l),C(2),m(y);var n=A(y,2),p=_(n),k=_(p),w=_(k);aa(w,{}),m(k),C(2),m(p),C(2),m(n);var c=A(n,2),u=_(c),b=_(u),x=_(b);ta(x,{}),m(b),C(2),m(u),C(2),m(c);var I=A(c,2),M=_(I),E=_(M),V=_(E);ea(V,{}),m(E),C(2),m(M),C(2),m(I);var S=A(I,2),W=_(S),Z=_(W),X=_(Z);ra(X,{}),m(Z),C(2),m(W),C(2),m(S),m(t),P(i,a)}export{ua as component};
