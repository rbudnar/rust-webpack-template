!function(e){function t(t){for(var n,o,i=t[0],s=t[1],a=0,u=[];a<i.length;a++)o=i[a],r[o]&&u.push(r[o][0]),r[o]=0;for(n in s)Object.prototype.hasOwnProperty.call(s,n)&&(e[n]=s[n]);for(_&&_(t);u.length;)u.shift()()}var n={},r={0:0};var o={};var i={7:function(){return{"./rust_mazes":{__wbindgen_object_drop_ref:function(e){return n[6].exports.__wbindgen_object_drop_ref(e)},__wbindgen_string_new:function(e,t){return n[6].exports.__wbindgen_string_new(e,t)},__widl_instanceof_Window:function(e){return n[6].exports.__widl_instanceof_Window(e)},__widl_f_set_property_CSSStyleDeclaration:function(e,t,r,o,i,s){return n[6].exports.__widl_f_set_property_CSSStyleDeclaration(e,t,r,o,i,s)},__widl_f_add_DOMTokenList:function(e,t,r){return n[6].exports.__widl_f_add_DOMTokenList(e,t,r)},__widl_f_create_element_Document:function(e,t,r,o){return n[6].exports.__widl_f_create_element_Document(e,t,r,o)},__widl_f_query_selector_Document:function(e,t,r,o){return n[6].exports.__widl_f_query_selector_Document(e,t,r,o)},__widl_f_body_Document:function(e){return n[6].exports.__widl_f_body_Document(e)},__widl_f_head_Document:function(e){return n[6].exports.__widl_f_head_Document(e)},__widl_f_class_list_Element:function(e){return n[6].exports.__widl_f_class_list_Element(e)},__widl_f_inner_html_Element:function(e,t){return n[6].exports.__widl_f_inner_html_Element(e,t)},__widl_f_set_inner_html_Element:function(e,t,r){return n[6].exports.__widl_f_set_inner_html_Element(e,t,r)},__widl_f_remove_Element:function(e){return n[6].exports.__widl_f_remove_Element(e)},__widl_instanceof_HTMLElement:function(e){return n[6].exports.__widl_instanceof_HTMLElement(e)},__widl_f_style_HTMLElement:function(e){return n[6].exports.__widl_f_style_HTMLElement(e)},__widl_f_append_child_Node:function(e,t,r){return n[6].exports.__widl_f_append_child_Node(e,t,r)},__widl_f_document_Window:function(e){return n[6].exports.__widl_f_document_Window(e)},__wbg_call_ee8306f6b79399de:function(e,t,r){return n[6].exports.__wbg_call_ee8306f6b79399de(e,t,r)},__wbg_newnoargs_96cbdf0d056b2fa8:function(e,t){return n[6].exports.__wbg_newnoargs_96cbdf0d056b2fa8(e,t)},__wbg_new_55d176391eb5bbb4:function(){return n[6].exports.__wbg_new_55d176391eb5bbb4()},__wbg_push_a3d2caf057c8ab57:function(e,t){return n[6].exports.__wbg_push_a3d2caf057c8ab57(e,t)},__wbg_random_fabf73e8a709437c:function(){return n[6].exports.__wbg_random_fabf73e8a709437c()},__wbindgen_number_get:function(e,t){return n[6].exports.__wbindgen_number_get(e,t)},__wbindgen_string_get:function(e,t){return n[6].exports.__wbindgen_string_get(e,t)},__wbindgen_boolean_get:function(e){return n[6].exports.__wbindgen_boolean_get(e)},__wbindgen_object_clone_ref:function(e){return n[6].exports.__wbindgen_object_clone_ref(e)},__wbindgen_is_null:function(e){return n[6].exports.__wbindgen_is_null(e)},__wbindgen_is_undefined:function(e){return n[6].exports.__wbindgen_is_undefined(e)},__wbindgen_is_symbol:function(e){return n[6].exports.__wbindgen_is_symbol(e)}}}}};function s(t){if(n[t])return n[t].exports;var r=n[t]={i:t,l:!1,exports:{}};return e[t].call(r.exports,r,r.exports,s),r.l=!0,r.exports}s.e=function(e){var t=[],n=r[e];if(0!==n)if(n)t.push(n[2]);else{var a=new Promise(function(t,o){n=r[e]=[t,o]});t.push(n[2]=a);var u,c=document.getElementsByTagName("head")[0],_=document.createElement("script");_.charset="utf-8",_.timeout=120,s.nc&&_.setAttribute("nonce",s.nc),_.src=function(e){return s.p+""+e+".bundle.js"}(e),u=function(t){_.onerror=_.onload=null,clearTimeout(f);var n=r[e];if(0!==n){if(n){var o=t&&("load"===t.type?"missing":t.type),i=t&&t.target&&t.target.src,s=new Error("Loading chunk "+e+" failed.\n("+o+": "+i+")");s.type=o,s.request=i,n[1](s)}r[e]=void 0}};var f=setTimeout(function(){u({type:"timeout",target:_})},12e4);_.onerror=_.onload=u,c.appendChild(_)}return({1:[7]}[e]||[]).forEach(function(e){var n=o[e];if(n)t.push(n);else{var r,a=i[e](),u=fetch(s.p+""+{7:"be636c95bb488c72b74a"}[e]+".module.wasm");if(a instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(u),a]).then(function(e){return WebAssembly.instantiate(e[0],e[1])});else if("function"==typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(u,a);else{r=u.then(function(e){return e.arrayBuffer()}).then(function(e){return WebAssembly.instantiate(e,a)})}t.push(o[e]=r.then(function(t){return s.w[e]=(t.instance||t).exports}))}}),Promise.all(t)},s.m=e,s.c=n,s.d=function(e,t,n){s.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},s.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},s.t=function(e,t){if(1&t&&(e=s(e)),8&t)return e;if(4&t&&"object"==typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(s.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)s.d(n,r,function(t){return e[t]}.bind(null,r));return n},s.n=function(e){var t=e&&e.__esModule?function(){return e.default}:function(){return e};return s.d(t,"a",t),t},s.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},s.p="",s.oe=function(e){throw console.error(e),e},s.w={};var a=window.webpackJsonp=window.webpackJsonp||[],u=a.push.bind(a);a.push=t,a=a.slice();for(var c=0;c<a.length;c++)t(a[c]);var _=u;s(s.s=0)}([function(e,t,n){"use strict";n.r(t);var r;n(1);n.e(1).then(n.bind(null,6)).then(e=>{r=e,e.basic_binary_tree(8,8)});const o=document.querySelector("#size-selector"),i=document.querySelector("#algorithm-selector"),s=document.querySelector("#new-maze"),a=()=>{let e=parseInt(i.value),t=parseInt(o.value);switch(e){case 1:r.basic_binary_tree(t,t);break;case 2:r.sidewinder(t,t);break;case 3:r.aldous_broder(t,t);break;case 4:r.wilson(t,t);break;default:console.warn("not a valid value"),r.basic_binary_tree(t,t)}};o.addEventListener("change",()=>a()),i.addEventListener("change",()=>a()),s.addEventListener("click",()=>a());const u=document.querySelector("#colorize");u.addEventListener("click",()=>{r.on_colorize_change(u.checked)})},function(e,t,n){var r=n(2);"string"==typeof r&&(r=[[e.i,r,""]]);var o={hmr:!0,transform:void 0,insertInto:void 0};n(4)(r,o);r.locals&&(e.exports=r.locals)},function(e,t,n){(e.exports=n(3)(!1)).push([e.i,"body {\n  height: 100vh; }\n\n.bt {\n  border-top: 1px solid black; }\n\n.bl {\n  border-left: 1px solid black; }\n\n.br {\n  border-right: 1px solid black; }\n\n.bb {\n  border-bottom: 1px solid black; }\n",""])},function(e,t){e.exports=function(e){var t=[];return t.toString=function(){return this.map(function(t){var n=function(e,t){var n=e[1]||"",r=e[3];if(!r)return n;if(t&&"function"==typeof btoa){var o=function(e){return"/*# sourceMappingURL=data:application/json;charset=utf-8;base64,"+btoa(unescape(encodeURIComponent(JSON.stringify(e))))+" */"}(r),i=r.sources.map(function(e){return"/*# sourceURL="+r.sourceRoot+e+" */"});return[n].concat(i).concat([o]).join("\n")}return[n].join("\n")}(t,e);return t[2]?"@media "+t[2]+"{"+n+"}":n}).join("")},t.i=function(e,n){"string"==typeof e&&(e=[[null,e,""]]);for(var r={},o=0;o<this.length;o++){var i=this[o][0];"number"==typeof i&&(r[i]=!0)}for(o=0;o<e.length;o++){var s=e[o];"number"==typeof s[0]&&r[s[0]]||(n&&!s[2]?s[2]=n:n&&(s[2]="("+s[2]+") and ("+n+")"),t.push(s))}},t}},function(e,t,n){var r={},o=function(e){var t;return function(){return void 0===t&&(t=e.apply(this,arguments)),t}}(function(){return window&&document&&document.all&&!window.atob}),i=function(e){var t={};return function(e,n){if("function"==typeof e)return e();if(void 0===t[e]){var r=function(e,t){return t?t.querySelector(e):document.querySelector(e)}.call(this,e,n);if(window.HTMLIFrameElement&&r instanceof window.HTMLIFrameElement)try{r=r.contentDocument.head}catch(e){r=null}t[e]=r}return t[e]}}(),s=null,a=0,u=[],c=n(5);function _(e,t){for(var n=0;n<e.length;n++){var o=e[n],i=r[o.id];if(i){i.refs++;for(var s=0;s<i.parts.length;s++)i.parts[s](o.parts[s]);for(;s<o.parts.length;s++)i.parts.push(m(o.parts[s],t))}else{var a=[];for(s=0;s<o.parts.length;s++)a.push(m(o.parts[s],t));r[o.id]={id:o.id,refs:1,parts:a}}}}function f(e,t){for(var n=[],r={},o=0;o<e.length;o++){var i=e[o],s=t.base?i[0]+t.base:i[0],a={css:i[1],media:i[2],sourceMap:i[3]};r[s]?r[s].parts.push(a):n.push(r[s]={id:s,parts:[a]})}return n}function l(e,t){var n=i(e.insertInto);if(!n)throw new Error("Couldn't find a style target. This probably means that the value for the 'insertInto' parameter is invalid.");var r=u[u.length-1];if("top"===e.insertAt)r?r.nextSibling?n.insertBefore(t,r.nextSibling):n.appendChild(t):n.insertBefore(t,n.firstChild),u.push(t);else if("bottom"===e.insertAt)n.appendChild(t);else{if("object"!=typeof e.insertAt||!e.insertAt.before)throw new Error("[Style Loader]\n\n Invalid value for parameter 'insertAt' ('options.insertAt') found.\n Must be 'top', 'bottom', or Object.\n (https://github.com/webpack-contrib/style-loader#insertat)\n");var o=i(e.insertAt.before,n);n.insertBefore(t,o)}}function d(e){if(null===e.parentNode)return!1;e.parentNode.removeChild(e);var t=u.indexOf(e);t>=0&&u.splice(t,1)}function p(e){var t=document.createElement("style");if(void 0===e.attrs.type&&(e.attrs.type="text/css"),void 0===e.attrs.nonce){var r=function(){0;return n.nc}();r&&(e.attrs.nonce=r)}return b(t,e.attrs),l(e,t),t}function b(e,t){Object.keys(t).forEach(function(n){e.setAttribute(n,t[n])})}function m(e,t){var n,r,o,i;if(t.transform&&e.css){if(!(i="function"==typeof t.transform?t.transform(e.css):t.transform.default(e.css)))return function(){};e.css=i}if(t.singleton){var u=a++;n=s||(s=p(t)),r=w.bind(null,n,u,!1),o=w.bind(null,n,u,!0)}else e.sourceMap&&"function"==typeof URL&&"function"==typeof URL.createObjectURL&&"function"==typeof URL.revokeObjectURL&&"function"==typeof Blob&&"function"==typeof btoa?(n=function(e){var t=document.createElement("link");return void 0===e.attrs.type&&(e.attrs.type="text/css"),e.attrs.rel="stylesheet",b(t,e.attrs),l(e,t),t}(t),r=function(e,t,n){var r=n.css,o=n.sourceMap,i=void 0===t.convertToAbsoluteUrls&&o;(t.convertToAbsoluteUrls||i)&&(r=c(r));o&&(r+="\n/*# sourceMappingURL=data:application/json;base64,"+btoa(unescape(encodeURIComponent(JSON.stringify(o))))+" */");var s=new Blob([r],{type:"text/css"}),a=e.href;e.href=URL.createObjectURL(s),a&&URL.revokeObjectURL(a)}.bind(null,n,t),o=function(){d(n),n.href&&URL.revokeObjectURL(n.href)}):(n=p(t),r=function(e,t){var n=t.css,r=t.media;r&&e.setAttribute("media",r);if(e.styleSheet)e.styleSheet.cssText=n;else{for(;e.firstChild;)e.removeChild(e.firstChild);e.appendChild(document.createTextNode(n))}}.bind(null,n),o=function(){d(n)});return r(e),function(t){if(t){if(t.css===e.css&&t.media===e.media&&t.sourceMap===e.sourceMap)return;r(e=t)}else o()}}e.exports=function(e,t){if("undefined"!=typeof DEBUG&&DEBUG&&"object"!=typeof document)throw new Error("The style-loader cannot be used in a non-browser environment");(t=t||{}).attrs="object"==typeof t.attrs?t.attrs:{},t.singleton||"boolean"==typeof t.singleton||(t.singleton=o()),t.insertInto||(t.insertInto="head"),t.insertAt||(t.insertAt="bottom");var n=f(e,t);return _(n,t),function(e){for(var o=[],i=0;i<n.length;i++){var s=n[i];(a=r[s.id]).refs--,o.push(a)}e&&_(f(e,t),t);for(i=0;i<o.length;i++){var a;if(0===(a=o[i]).refs){for(var u=0;u<a.parts.length;u++)a.parts[u]();delete r[a.id]}}}};var h=function(){var e=[];return function(t,n){return e[t]=n,e.filter(Boolean).join("\n")}}();function w(e,t,n,r){var o=n?"":r.css;if(e.styleSheet)e.styleSheet.cssText=h(t,o);else{var i=document.createTextNode(o),s=e.childNodes;s[t]&&e.removeChild(s[t]),s.length?e.insertBefore(i,s[t]):e.appendChild(i)}}},function(e,t){e.exports=function(e){var t="undefined"!=typeof window&&window.location;if(!t)throw new Error("fixUrls requires window.location");if(!e||"string"!=typeof e)return e;var n=t.protocol+"//"+t.host,r=n+t.pathname.replace(/\/[^\/]*$/,"/");return e.replace(/url\s*\(((?:[^)(]|\((?:[^)(]+|\([^)(]*\))*\))*)\)/gi,function(e,t){var o,i=t.trim().replace(/^"(.*)"$/,function(e,t){return t}).replace(/^'(.*)'$/,function(e,t){return t});return/^(#|data:|http:\/\/|https:\/\/|file:\/\/\/|\s*$)/i.test(i)?e:(o=0===i.indexOf("//")?i:0===i.indexOf("/")?n+i:r+i.replace(/^\.\//,""),"url("+JSON.stringify(o)+")")})}}]);