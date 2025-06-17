var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/worker/shim.mjs
import rn from "./a3bdb1bc00909aecbf27a5d4d9ff906f44c344fc-index.wasm";
import { WorkerEntrypoint as _n } from "cloudflare:workers";
var C = Object.defineProperty;
var D = /* @__PURE__ */ __name((e2, t) => {
  for (var n in t) C(e2, n, { get: t[n], enumerable: true });
}, "D");
var w = {};
D(w, { IntoUnderlyingByteSource: /* @__PURE__ */ __name(() => T, "IntoUnderlyingByteSource"), IntoUnderlyingSink: /* @__PURE__ */ __name(() => I, "IntoUnderlyingSink"), IntoUnderlyingSource: /* @__PURE__ */ __name(() => R, "IntoUnderlyingSource"), MinifyConfig: /* @__PURE__ */ __name(() => O, "MinifyConfig"), PolishConfig: /* @__PURE__ */ __name(() => v, "PolishConfig"), R2Range: /* @__PURE__ */ __name(() => k, "R2Range"), RequestRedirect: /* @__PURE__ */ __name(() => G, "RequestRedirect"), __wbg_String_8f0eb39a4a4c2f66: /* @__PURE__ */ __name(() => te, "__wbg_String_8f0eb39a4a4c2f66"), __wbg_abort_410ec47a64ac6117: /* @__PURE__ */ __name(() => ne, "__wbg_abort_410ec47a64ac6117"), __wbg_abort_775ef1d17fc65868: /* @__PURE__ */ __name(() => re, "__wbg_abort_775ef1d17fc65868"), __wbg_append_8c7dd8d641a5f01b: /* @__PURE__ */ __name(() => _e, "__wbg_append_8c7dd8d641a5f01b"), __wbg_body_018617e858cb7195: /* @__PURE__ */ __name(() => oe, "__wbg_body_018617e858cb7195"), __wbg_body_0b8fd1fe671660df: /* @__PURE__ */ __name(() => ce, "__wbg_body_0b8fd1fe671660df"), __wbg_buffer_09165b52af8c5237: /* @__PURE__ */ __name(() => ie, "__wbg_buffer_09165b52af8c5237"), __wbg_buffer_609cc3eee51ed158: /* @__PURE__ */ __name(() => se, "__wbg_buffer_609cc3eee51ed158"), __wbg_byobRequest_77d9adf63337edfb: /* @__PURE__ */ __name(() => ue, "__wbg_byobRequest_77d9adf63337edfb"), __wbg_byteLength_e674b853d9c77e1d: /* @__PURE__ */ __name(() => fe, "__wbg_byteLength_e674b853d9c77e1d"), __wbg_byteOffset_fd862df290ef848d: /* @__PURE__ */ __name(() => ae, "__wbg_byteOffset_fd862df290ef848d"), __wbg_call_672a4d21634d4a24: /* @__PURE__ */ __name(() => be, "__wbg_call_672a4d21634d4a24"), __wbg_call_7cccdd69e0791ae2: /* @__PURE__ */ __name(() => ge, "__wbg_call_7cccdd69e0791ae2"), __wbg_cancel_8a308660caa6cadf: /* @__PURE__ */ __name(() => de, "__wbg_cancel_8a308660caa6cadf"), __wbg_catch_a6e601879b2610e9: /* @__PURE__ */ __name(() => we, "__wbg_catch_a6e601879b2610e9"), __wbg_cause_9940c4e8dfcd5129: /* @__PURE__ */ __name(() => le, "__wbg_cause_9940c4e8dfcd5129"), __wbg_cf_123509d53a2ea003: /* @__PURE__ */ __name(() => pe, "__wbg_cf_123509d53a2ea003"), __wbg_clearTimeout_b1115618e821c3b2: /* @__PURE__ */ __name(() => xe, "__wbg_clearTimeout_b1115618e821c3b2"), __wbg_close_304cc1fef3466669: /* @__PURE__ */ __name(() => ye, "__wbg_close_304cc1fef3466669"), __wbg_close_5ce03e29be453811: /* @__PURE__ */ __name(() => he, "__wbg_close_5ce03e29be453811"), __wbg_done_769e5ede4b31c67b: /* @__PURE__ */ __name(() => me, "__wbg_done_769e5ede4b31c67b"), __wbg_enqueue_bb16ba72f537dc9e: /* @__PURE__ */ __name(() => Re, "__wbg_enqueue_bb16ba72f537dc9e"), __wbg_entries_2a52db465d0421fb: /* @__PURE__ */ __name(() => Fe, "__wbg_entries_2a52db465d0421fb"), __wbg_error_524f506f44df1645: /* @__PURE__ */ __name(() => Se, "__wbg_error_524f506f44df1645"), __wbg_error_7534b8e9a36f1ab4: /* @__PURE__ */ __name(() => Te, "__wbg_error_7534b8e9a36f1ab4"), __wbg_fetch_3afbdcc7ddbf16fe: /* @__PURE__ */ __name(() => Ie, "__wbg_fetch_3afbdcc7ddbf16fe"), __wbg_fetch_509096533071c657: /* @__PURE__ */ __name(() => Oe, "__wbg_fetch_509096533071c657"), __wbg_getRandomValues_3c9c0d586e575a16: /* @__PURE__ */ __name(() => ke, "__wbg_getRandomValues_3c9c0d586e575a16"), __wbg_getReader_48e00749fe3f6089: /* @__PURE__ */ __name(() => Ee, "__wbg_getReader_48e00749fe3f6089"), __wbg_getTime_46267b1c24877e30: /* @__PURE__ */ __name(() => ze, "__wbg_getTime_46267b1c24877e30"), __wbg_get_67b2ba62fc30de12: /* @__PURE__ */ __name(() => Me, "__wbg_get_67b2ba62fc30de12"), __wbg_get_b9b93047fe3cf45b: /* @__PURE__ */ __name(() => je, "__wbg_get_b9b93047fe3cf45b"), __wbg_getdone_d47073731acd3e74: /* @__PURE__ */ __name(() => qe, "__wbg_getdone_d47073731acd3e74"), __wbg_getvalue_009dcd63692bee1f: /* @__PURE__ */ __name(() => Le, "__wbg_getvalue_009dcd63692bee1f"), __wbg_has_a5ea9117f258a0ec: /* @__PURE__ */ __name(() => Ae, "__wbg_has_a5ea9117f258a0ec"), __wbg_headers_7852a8ea641c1379: /* @__PURE__ */ __name(() => Ce, "__wbg_headers_7852a8ea641c1379"), __wbg_headers_9cb51cfd2ac780a4: /* @__PURE__ */ __name(() => De, "__wbg_headers_9cb51cfd2ac780a4"), __wbg_httpProtocol_4cc3ab4fde2ecf82: /* @__PURE__ */ __name(() => Ue, "__wbg_httpProtocol_4cc3ab4fde2ecf82"), __wbg_instanceof_Error_4d54113b22d20306: /* @__PURE__ */ __name(() => We, "__wbg_instanceof_Error_4d54113b22d20306"), __wbg_instanceof_Response_f2cc20d9f7dfd644: /* @__PURE__ */ __name(() => Pe, "__wbg_instanceof_Response_f2cc20d9f7dfd644"), __wbg_iterator_9a24c88df860dc65: /* @__PURE__ */ __name(() => $e, "__wbg_iterator_9a24c88df860dc65"), __wbg_length_a446193dc22c12f8: /* @__PURE__ */ __name(() => Ne, "__wbg_length_a446193dc22c12f8"), __wbg_method_3dcc854b644c5a56: /* @__PURE__ */ __name(() => Ve, "__wbg_method_3dcc854b644c5a56"), __wbg_new0_f788a2397c7ca929: /* @__PURE__ */ __name(() => Be, "__wbg_new0_f788a2397c7ca929"), __wbg_new_018dcc2d6c8c2f6a: /* @__PURE__ */ __name(() => ve, "__wbg_new_018dcc2d6c8c2f6a"), __wbg_new_23a2665fac83c611: /* @__PURE__ */ __name(() => Ge, "__wbg_new_23a2665fac83c611"), __wbg_new_405e22f390576ce2: /* @__PURE__ */ __name(() => He, "__wbg_new_405e22f390576ce2"), __wbg_new_8a6f238a6ece86ea: /* @__PURE__ */ __name(() => Je, "__wbg_new_8a6f238a6ece86ea"), __wbg_new_a12002a7f91c75be: /* @__PURE__ */ __name(() => Xe, "__wbg_new_a12002a7f91c75be"), __wbg_new_c68d7209be747379: /* @__PURE__ */ __name(() => Ke, "__wbg_new_c68d7209be747379"), __wbg_new_e25e5aab09ff45db: /* @__PURE__ */ __name(() => Qe, "__wbg_new_e25e5aab09ff45db"), __wbg_newnoargs_105ed471475aaf50: /* @__PURE__ */ __name(() => Ye, "__wbg_newnoargs_105ed471475aaf50"), __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a: /* @__PURE__ */ __name(() => Ze, "__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a"), __wbg_newwithintounderlyingsource_b47f6a6a596a7f24: /* @__PURE__ */ __name(() => et, "__wbg_newwithintounderlyingsource_b47f6a6a596a7f24"), __wbg_newwithlength_a381634e90c276d4: /* @__PURE__ */ __name(() => tt, "__wbg_newwithlength_a381634e90c276d4"), __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1: /* @__PURE__ */ __name(() => nt, "__wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1"), __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e: /* @__PURE__ */ __name(() => rt, "__wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e"), __wbg_newwithoptstrandinit_615a266ef226c260: /* @__PURE__ */ __name(() => _t, "__wbg_newwithoptstrandinit_615a266ef226c260"), __wbg_newwithstrandinit_06c535e0a867c635: /* @__PURE__ */ __name(() => ot, "__wbg_newwithstrandinit_06c535e0a867c635"), __wbg_next_25feadfc0913fea9: /* @__PURE__ */ __name(() => ct, "__wbg_next_25feadfc0913fea9"), __wbg_next_6574e1a8a62d1055: /* @__PURE__ */ __name(() => it, "__wbg_next_6574e1a8a62d1055"), __wbg_now_2c95c9de01293173: /* @__PURE__ */ __name(() => st, "__wbg_now_2c95c9de01293173"), __wbg_performance_7a3ffd0b17f663ad: /* @__PURE__ */ __name(() => ut, "__wbg_performance_7a3ffd0b17f663ad"), __wbg_queueMicrotask_97d92b4fcc8a61c5: /* @__PURE__ */ __name(() => ft, "__wbg_queueMicrotask_97d92b4fcc8a61c5"), __wbg_queueMicrotask_d3219def82552485: /* @__PURE__ */ __name(() => at, "__wbg_queueMicrotask_d3219def82552485"), __wbg_read_a2434af1186cb56c: /* @__PURE__ */ __name(() => bt, "__wbg_read_a2434af1186cb56c"), __wbg_redirect_14b0c8193458f8c3: /* @__PURE__ */ __name(() => gt, "__wbg_redirect_14b0c8193458f8c3"), __wbg_releaseLock_091899af97991d2e: /* @__PURE__ */ __name(() => dt, "__wbg_releaseLock_091899af97991d2e"), __wbg_resolve_4851785c9c5f573d: /* @__PURE__ */ __name(() => wt, "__wbg_resolve_4851785c9c5f573d"), __wbg_respond_1f279fa9f8edcb1c: /* @__PURE__ */ __name(() => lt, "__wbg_respond_1f279fa9f8edcb1c"), __wbg_setTimeout_ca12ead8b48245e2: /* @__PURE__ */ __name(() => pt, "__wbg_setTimeout_ca12ead8b48245e2"), __wbg_set_65595bdd868b3009: /* @__PURE__ */ __name(() => xt, "__wbg_set_65595bdd868b3009"), __wbg_set_bb8cecf6a62b9f46: /* @__PURE__ */ __name(() => yt, "__wbg_set_bb8cecf6a62b9f46"), __wbg_set_wasm: /* @__PURE__ */ __name(() => E, "__wbg_set_wasm"), __wbg_setbody_5923b78a95eedf29: /* @__PURE__ */ __name(() => ht, "__wbg_setbody_5923b78a95eedf29"), __wbg_setcredentials_c3a22f1cd105a2c6: /* @__PURE__ */ __name(() => mt, "__wbg_setcredentials_c3a22f1cd105a2c6"), __wbg_setheaders_3b47c898e8de6d44: /* @__PURE__ */ __name(() => Rt, "__wbg_setheaders_3b47c898e8de6d44"), __wbg_setheaders_834c0bdb6a8949ad: /* @__PURE__ */ __name(() => Ft, "__wbg_setheaders_834c0bdb6a8949ad"), __wbg_sethighwatermark_793c99c89830c8e9: /* @__PURE__ */ __name(() => St, "__wbg_sethighwatermark_793c99c89830c8e9"), __wbg_setmethod_3c5280fe5d890842: /* @__PURE__ */ __name(() => Tt, "__wbg_setmethod_3c5280fe5d890842"), __wbg_setmode_5dc300b865044b65: /* @__PURE__ */ __name(() => It, "__wbg_setmode_5dc300b865044b65"), __wbg_setsignal_75b21ef3a81de905: /* @__PURE__ */ __name(() => Ot, "__wbg_setsignal_75b21ef3a81de905"), __wbg_setstatus_51b4fc011091cbb3: /* @__PURE__ */ __name(() => kt, "__wbg_setstatus_51b4fc011091cbb3"), __wbg_signal_02f4435f82019061: /* @__PURE__ */ __name(() => Et, "__wbg_signal_02f4435f82019061"), __wbg_signal_aaf9ad74119f20a4: /* @__PURE__ */ __name(() => zt, "__wbg_signal_aaf9ad74119f20a4"), __wbg_stack_0ed75d68575b0f3c: /* @__PURE__ */ __name(() => Mt, "__wbg_stack_0ed75d68575b0f3c"), __wbg_static_accessor_GLOBAL_88a902d13a557d07: /* @__PURE__ */ __name(() => jt, "__wbg_static_accessor_GLOBAL_88a902d13a557d07"), __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0: /* @__PURE__ */ __name(() => qt, "__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0"), __wbg_static_accessor_SELF_37c5d418e4bf5819: /* @__PURE__ */ __name(() => Lt, "__wbg_static_accessor_SELF_37c5d418e4bf5819"), __wbg_static_accessor_WINDOW_5de37043a91a9c40: /* @__PURE__ */ __name(() => At, "__wbg_static_accessor_WINDOW_5de37043a91a9c40"), __wbg_status_f6360336ca686bf0: /* @__PURE__ */ __name(() => Ct, "__wbg_status_f6360336ca686bf0"), __wbg_stringify_f7ed6987935b4a24: /* @__PURE__ */ __name(() => Dt, "__wbg_stringify_f7ed6987935b4a24"), __wbg_then_44b73946d2fb3e7d: /* @__PURE__ */ __name(() => Ut, "__wbg_then_44b73946d2fb3e7d"), __wbg_then_48b406749878a531: /* @__PURE__ */ __name(() => Wt, "__wbg_then_48b406749878a531"), __wbg_toString_c813bbd34d063839: /* @__PURE__ */ __name(() => Pt, "__wbg_toString_c813bbd34d063839"), __wbg_url_8f9653b899456042: /* @__PURE__ */ __name(() => $t, "__wbg_url_8f9653b899456042"), __wbg_url_ae10c34ca209681d: /* @__PURE__ */ __name(() => Nt, "__wbg_url_ae10c34ca209681d"), __wbg_value_cd1ffa7b1ab794f1: /* @__PURE__ */ __name(() => Vt, "__wbg_value_cd1ffa7b1ab794f1"), __wbg_view_fd8a56e8983f448d: /* @__PURE__ */ __name(() => Bt, "__wbg_view_fd8a56e8983f448d"), __wbindgen_cb_drop: /* @__PURE__ */ __name(() => vt, "__wbindgen_cb_drop"), __wbindgen_closure_wrapper4896: /* @__PURE__ */ __name(() => Gt, "__wbindgen_closure_wrapper4896"), __wbindgen_closure_wrapper5060: /* @__PURE__ */ __name(() => Ht, "__wbindgen_closure_wrapper5060"), __wbindgen_debug_string: /* @__PURE__ */ __name(() => Jt, "__wbindgen_debug_string"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(() => Xt, "__wbindgen_init_externref_table"), __wbindgen_is_function: /* @__PURE__ */ __name(() => Kt, "__wbindgen_is_function"), __wbindgen_is_object: /* @__PURE__ */ __name(() => Qt, "__wbindgen_is_object"), __wbindgen_is_undefined: /* @__PURE__ */ __name(() => Yt, "__wbindgen_is_undefined"), __wbindgen_memory: /* @__PURE__ */ __name(() => Zt, "__wbindgen_memory"), __wbindgen_string_get: /* @__PURE__ */ __name(() => en, "__wbindgen_string_get"), __wbindgen_string_new: /* @__PURE__ */ __name(() => tn, "__wbindgen_string_new"), __wbindgen_throw: /* @__PURE__ */ __name(() => nn, "__wbindgen_throw"), fetch: /* @__PURE__ */ __name(() => z, "fetch") });
var r;
function E(e2) {
  r = e2;
}
__name(E, "E");
var b = 0;
var h = null;
function y() {
  return (h === null || h.byteLength === 0) && (h = new Uint8Array(r.memory.buffer)), h;
}
__name(y, "y");
var U = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var m = new U("utf-8");
var W = typeof m.encodeInto == "function" ? function(e2, t) {
  return m.encodeInto(e2, t);
} : function(e2, t) {
  let n = m.encode(e2);
  return t.set(n), { read: e2.length, written: n.length };
};
function l(e2, t, n) {
  if (n === void 0) {
    let u = m.encode(e2), x = t(u.length, 1) >>> 0;
    return y().subarray(x, x + u.length).set(u), b = u.length, x;
  }
  let _ = e2.length, o = t(_, 1) >>> 0, a = y(), i = 0;
  for (; i < _; i++) {
    let u = e2.charCodeAt(i);
    if (u > 127) break;
    a[o + i] = u;
  }
  if (i !== _) {
    i !== 0 && (e2 = e2.slice(i)), o = n(o, _, _ = i + e2.length * 3, 1) >>> 0;
    let u = y().subarray(o + i, o + _), x = W(e2, u);
    i += x.written, o = n(o, _, i, 1) >>> 0;
  }
  return b = i, o;
}
__name(l, "l");
var p = null;
function f() {
  return (p === null || p.buffer.detached === true || p.buffer.detached === void 0 && p.buffer !== r.memory.buffer) && (p = new DataView(r.memory.buffer)), p;
}
__name(f, "f");
var P = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var q = new P("utf-8", { ignoreBOM: true, fatal: true });
q.decode();
function g(e2, t) {
  return e2 = e2 >>> 0, q.decode(y().subarray(e2, e2 + t));
}
__name(g, "g");
function d(e2) {
  let t = r.__externref_table_alloc();
  return r.__wbindgen_export_4.set(t, e2), t;
}
__name(d, "d");
function c(e2, t) {
  try {
    return e2.apply(this, t);
  } catch (n) {
    let _ = d(n);
    r.__wbindgen_exn_store(_);
  }
}
__name(c, "c");
function s(e2) {
  return e2 == null;
}
__name(s, "s");
function $(e2, t) {
  return e2 = e2 >>> 0, y().subarray(e2 / 1, e2 / 1 + t);
}
__name($, "$");
var M = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e2) => {
  r.__wbindgen_export_6.get(e2.dtor)(e2.a, e2.b);
});
function L(e2, t, n, _) {
  let o = { a: e2, b: t, cnt: 1, dtor: n }, a = /* @__PURE__ */ __name((...i) => {
    o.cnt++;
    let u = o.a;
    o.a = 0;
    try {
      return _(u, o.b, ...i);
    } finally {
      --o.cnt === 0 ? (r.__wbindgen_export_6.get(o.dtor)(u, o.b), M.unregister(o)) : o.a = u;
    }
  }, "a");
  return a.original = o, M.register(a, o, o), a;
}
__name(L, "L");
function S(e2) {
  let t = typeof e2;
  if (t == "number" || t == "boolean" || e2 == null) return `${e2}`;
  if (t == "string") return `"${e2}"`;
  if (t == "symbol") {
    let o = e2.description;
    return o == null ? "Symbol" : `Symbol(${o})`;
  }
  if (t == "function") {
    let o = e2.name;
    return typeof o == "string" && o.length > 0 ? `Function(${o})` : "Function";
  }
  if (Array.isArray(e2)) {
    let o = e2.length, a = "[";
    o > 0 && (a += S(e2[0]));
    for (let i = 1; i < o; i++) a += ", " + S(e2[i]);
    return a += "]", a;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(e2)), _;
  if (n && n.length > 1) _ = n[1];
  else return toString.call(e2);
  if (_ == "Object") try {
    return "Object(" + JSON.stringify(e2) + ")";
  } catch {
    return "Object";
  }
  return e2 instanceof Error ? `${e2.name}: ${e2.message}
${e2.stack}` : _;
}
__name(S, "S");
function z(e2, t, n) {
  return r.fetch(e2, t, n);
}
__name(z, "z");
function N(e2, t) {
  r._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8deaea13237faeb3(e2, t);
}
__name(N, "N");
function V(e2, t, n) {
  r.closure2544_externref_shim(e2, t, n);
}
__name(V, "V");
function B(e2, t, n, _) {
  r.closure2678_externref_shim(e2, t, n, _);
}
__name(B, "B");
var v = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var G = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var H = ["bytes"];
var J = ["omit", "same-origin", "include"];
var X = ["same-origin", "no-cors", "cors", "navigate"];
var K = ["follow", "error", "manual"];
var Q = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e2) => r.__wbg_intounderlyingbytesource_free(e2 >>> 0, 1));
var T = class {
  static {
    __name(this, "T");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Q.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingbytesource_free(t, 0);
  }
  get type() {
    let t = r.intounderlyingbytesource_type(this.__wbg_ptr);
    return H[t];
  }
  get autoAllocateChunkSize() {
    return r.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(t) {
    r.intounderlyingbytesource_start(this.__wbg_ptr, t);
  }
  pull(t) {
    return r.intounderlyingbytesource_pull(this.__wbg_ptr, t);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingbytesource_cancel(t);
  }
};
var Y = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e2) => r.__wbg_intounderlyingsink_free(e2 >>> 0, 1));
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Y.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsink_free(t, 0);
  }
  write(t) {
    return r.intounderlyingsink_write(this.__wbg_ptr, t);
  }
  close() {
    let t = this.__destroy_into_raw();
    return r.intounderlyingsink_close(t);
  }
  abort(t) {
    let n = this.__destroy_into_raw();
    return r.intounderlyingsink_abort(n, t);
  }
};
var j = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e2) => r.__wbg_intounderlyingsource_free(e2 >>> 0, 1));
var R = class e {
  static {
    __name(this, "e");
  }
  static __wrap(t) {
    t = t >>> 0;
    let n = Object.create(e.prototype);
    return n.__wbg_ptr = t, j.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, j.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsource_free(t, 0);
  }
  pull(t) {
    return r.intounderlyingsource_pull(this.__wbg_ptr, t);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingsource_cancel(t);
  }
};
var Z = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e2) => r.__wbg_minifyconfig_free(e2 >>> 0, 1));
var O = class {
  static {
    __name(this, "O");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Z.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_minifyconfig_free(t, 0);
  }
  get js() {
    return r.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(t) {
    r.__wbg_set_minifyconfig_js(this.__wbg_ptr, t);
  }
  get html() {
    return r.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(t) {
    r.__wbg_set_minifyconfig_html(this.__wbg_ptr, t);
  }
  get css() {
    return r.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(t) {
    r.__wbg_set_minifyconfig_css(this.__wbg_ptr, t);
  }
};
var ee = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e2) => r.__wbg_r2range_free(e2 >>> 0, 1));
var k = class {
  static {
    __name(this, "k");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ee.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_r2range_free(t, 0);
  }
  get offset() {
    let t = r.__wbg_get_r2range_offset(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set offset(t) {
    r.__wbg_set_r2range_offset(this.__wbg_ptr, !s(t), s(t) ? 0 : t);
  }
  get length() {
    let t = r.__wbg_get_r2range_length(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set length(t) {
    r.__wbg_set_r2range_length(this.__wbg_ptr, !s(t), s(t) ? 0 : t);
  }
  get suffix() {
    let t = r.__wbg_get_r2range_suffix(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set suffix(t) {
    r.__wbg_set_r2range_suffix(this.__wbg_ptr, !s(t), s(t) ? 0 : t);
  }
};
function te(e2, t) {
  let n = String(t), _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = b;
  f().setInt32(e2 + 4 * 1, o, true), f().setInt32(e2 + 4 * 0, _, true);
}
__name(te, "te");
function ne(e2, t) {
  e2.abort(t);
}
__name(ne, "ne");
function re(e2) {
  e2.abort();
}
__name(re, "re");
function _e() {
  return c(function(e2, t, n, _, o) {
    e2.append(g(t, n), g(_, o));
  }, arguments);
}
__name(_e, "_e");
function oe(e2) {
  let t = e2.body;
  return s(t) ? 0 : d(t);
}
__name(oe, "oe");
function ce(e2) {
  let t = e2.body;
  return s(t) ? 0 : d(t);
}
__name(ce, "ce");
function ie(e2) {
  return e2.buffer;
}
__name(ie, "ie");
function se(e2) {
  return e2.buffer;
}
__name(se, "se");
function ue(e2) {
  let t = e2.byobRequest;
  return s(t) ? 0 : d(t);
}
__name(ue, "ue");
function fe(e2) {
  return e2.byteLength;
}
__name(fe, "fe");
function ae(e2) {
  return e2.byteOffset;
}
__name(ae, "ae");
function be() {
  return c(function(e2, t) {
    return e2.call(t);
  }, arguments);
}
__name(be, "be");
function ge() {
  return c(function(e2, t, n) {
    return e2.call(t, n);
  }, arguments);
}
__name(ge, "ge");
function de(e2) {
  return e2.cancel();
}
__name(de, "de");
function we(e2, t) {
  return e2.catch(t);
}
__name(we, "we");
function le(e2) {
  return e2.cause;
}
__name(le, "le");
function pe() {
  return c(function(e2) {
    let t = e2.cf;
    return s(t) ? 0 : d(t);
  }, arguments);
}
__name(pe, "pe");
function xe(e2) {
  return clearTimeout(e2);
}
__name(xe, "xe");
function ye() {
  return c(function(e2) {
    e2.close();
  }, arguments);
}
__name(ye, "ye");
function he() {
  return c(function(e2) {
    e2.close();
  }, arguments);
}
__name(he, "he");
function me(e2) {
  return e2.done;
}
__name(me, "me");
function Re() {
  return c(function(e2, t) {
    e2.enqueue(t);
  }, arguments);
}
__name(Re, "Re");
function Fe(e2) {
  return e2.entries();
}
__name(Fe, "Fe");
function Se(e2) {
  console.error(e2);
}
__name(Se, "Se");
function Te(e2, t) {
  let n, _;
  try {
    n = e2, _ = t, console.error(g(e2, t));
  } finally {
    r.__wbindgen_free(n, _, 1);
  }
}
__name(Te, "Te");
function Ie(e2) {
  return fetch(e2);
}
__name(Ie, "Ie");
function Oe(e2, t) {
  return e2.fetch(t);
}
__name(Oe, "Oe");
function ke() {
  return c(function(e2, t) {
    globalThis.crypto.getRandomValues($(e2, t));
  }, arguments);
}
__name(ke, "ke");
function Ee() {
  return c(function(e2) {
    return e2.getReader();
  }, arguments);
}
__name(Ee, "Ee");
function ze(e2) {
  return e2.getTime();
}
__name(ze, "ze");
function Me() {
  return c(function(e2, t) {
    return Reflect.get(e2, t);
  }, arguments);
}
__name(Me, "Me");
function je(e2, t) {
  return e2[t >>> 0];
}
__name(je, "je");
function qe(e2) {
  let t = e2.done;
  return s(t) ? 16777215 : t ? 1 : 0;
}
__name(qe, "qe");
function Le(e2) {
  return e2.value;
}
__name(Le, "Le");
function Ae() {
  return c(function(e2, t) {
    return Reflect.has(e2, t);
  }, arguments);
}
__name(Ae, "Ae");
function Ce(e2) {
  return e2.headers;
}
__name(Ce, "Ce");
function De(e2) {
  return e2.headers;
}
__name(De, "De");
function Ue() {
  return c(function(e2, t) {
    let n = t.httpProtocol, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = b;
    f().setInt32(e2 + 4 * 1, o, true), f().setInt32(e2 + 4 * 0, _, true);
  }, arguments);
}
__name(Ue, "Ue");
function We(e2) {
  let t;
  try {
    t = e2 instanceof Error;
  } catch {
    t = false;
  }
  return t;
}
__name(We, "We");
function Pe(e2) {
  let t;
  try {
    t = e2 instanceof Response;
  } catch {
    t = false;
  }
  return t;
}
__name(Pe, "Pe");
function $e() {
  return Symbol.iterator;
}
__name($e, "$e");
function Ne(e2) {
  return e2.length;
}
__name(Ne, "Ne");
function Ve(e2, t) {
  let n = t.method, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = b;
  f().setInt32(e2 + 4 * 1, o, true), f().setInt32(e2 + 4 * 0, _, true);
}
__name(Ve, "Ve");
function Be() {
  return /* @__PURE__ */ new Date();
}
__name(Be, "Be");
function ve() {
  return c(function() {
    return new Headers();
  }, arguments);
}
__name(ve, "ve");
function Ge(e2, t) {
  try {
    var n = { a: e2, b: t }, _ = /* @__PURE__ */ __name((a, i) => {
      let u = n.a;
      n.a = 0;
      try {
        return B(u, n.b, a, i);
      } finally {
        n.a = u;
      }
    }, "_");
    return new Promise(_);
  } finally {
    n.a = n.b = 0;
  }
}
__name(Ge, "Ge");
function He() {
  return new Object();
}
__name(He, "He");
function Je() {
  return new Error();
}
__name(Je, "Je");
function Xe(e2) {
  return new Uint8Array(e2);
}
__name(Xe, "Xe");
function Ke(e2, t) {
  return new Error(g(e2, t));
}
__name(Ke, "Ke");
function Qe() {
  return c(function() {
    return new AbortController();
  }, arguments);
}
__name(Qe, "Qe");
function Ye(e2, t) {
  return new Function(g(e2, t));
}
__name(Ye, "Ye");
function Ze(e2, t, n) {
  return new Uint8Array(e2, t >>> 0, n >>> 0);
}
__name(Ze, "Ze");
function et(e2, t) {
  return new ReadableStream(R.__wrap(e2), t);
}
__name(et, "et");
function tt(e2) {
  return new Uint8Array(e2 >>> 0);
}
__name(tt, "tt");
function nt() {
  return c(function(e2, t) {
    return new Response(e2, t);
  }, arguments);
}
__name(nt, "nt");
function rt() {
  return c(function(e2, t) {
    return new Response(e2, t);
  }, arguments);
}
__name(rt, "rt");
function _t() {
  return c(function(e2, t, n) {
    return new Response(e2 === 0 ? void 0 : g(e2, t), n);
  }, arguments);
}
__name(_t, "_t");
function ot() {
  return c(function(e2, t, n) {
    return new Request(g(e2, t), n);
  }, arguments);
}
__name(ot, "ot");
function ct(e2) {
  return e2.next;
}
__name(ct, "ct");
function it() {
  return c(function(e2) {
    return e2.next();
  }, arguments);
}
__name(it, "it");
function st(e2) {
  return e2.now();
}
__name(st, "st");
function ut(e2) {
  return e2.performance;
}
__name(ut, "ut");
function ft(e2) {
  queueMicrotask(e2);
}
__name(ft, "ft");
function at(e2) {
  return e2.queueMicrotask;
}
__name(at, "at");
function bt(e2) {
  return e2.read();
}
__name(bt, "bt");
function gt(e2) {
  let t = e2.redirect;
  return (K.indexOf(t) + 1 || 4) - 1;
}
__name(gt, "gt");
function dt(e2) {
  e2.releaseLock();
}
__name(dt, "dt");
function wt(e2) {
  return Promise.resolve(e2);
}
__name(wt, "wt");
function lt() {
  return c(function(e2, t) {
    e2.respond(t >>> 0);
  }, arguments);
}
__name(lt, "lt");
function pt(e2, t) {
  return setTimeout(e2, t);
}
__name(pt, "pt");
function xt(e2, t, n) {
  e2.set(t, n >>> 0);
}
__name(xt, "xt");
function yt() {
  return c(function(e2, t, n) {
    return Reflect.set(e2, t, n);
  }, arguments);
}
__name(yt, "yt");
function ht(e2, t) {
  e2.body = t;
}
__name(ht, "ht");
function mt(e2, t) {
  e2.credentials = J[t];
}
__name(mt, "mt");
function Rt(e2, t) {
  e2.headers = t;
}
__name(Rt, "Rt");
function Ft(e2, t) {
  e2.headers = t;
}
__name(Ft, "Ft");
function St(e2, t) {
  e2.highWaterMark = t;
}
__name(St, "St");
function Tt(e2, t, n) {
  e2.method = g(t, n);
}
__name(Tt, "Tt");
function It(e2, t) {
  e2.mode = X[t];
}
__name(It, "It");
function Ot(e2, t) {
  e2.signal = t;
}
__name(Ot, "Ot");
function kt(e2, t) {
  e2.status = t;
}
__name(kt, "kt");
function Et(e2) {
  return e2.signal;
}
__name(Et, "Et");
function zt(e2) {
  return e2.signal;
}
__name(zt, "zt");
function Mt(e2, t) {
  let n = t.stack, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = b;
  f().setInt32(e2 + 4 * 1, o, true), f().setInt32(e2 + 4 * 0, _, true);
}
__name(Mt, "Mt");
function jt() {
  let e2 = typeof global > "u" ? null : global;
  return s(e2) ? 0 : d(e2);
}
__name(jt, "jt");
function qt() {
  let e2 = typeof globalThis > "u" ? null : globalThis;
  return s(e2) ? 0 : d(e2);
}
__name(qt, "qt");
function Lt() {
  let e2 = typeof self > "u" ? null : self;
  return s(e2) ? 0 : d(e2);
}
__name(Lt, "Lt");
function At() {
  let e2 = typeof window > "u" ? null : window;
  return s(e2) ? 0 : d(e2);
}
__name(At, "At");
function Ct(e2) {
  return e2.status;
}
__name(Ct, "Ct");
function Dt() {
  return c(function(e2) {
    return JSON.stringify(e2);
  }, arguments);
}
__name(Dt, "Dt");
function Ut(e2, t) {
  return e2.then(t);
}
__name(Ut, "Ut");
function Wt(e2, t, n) {
  return e2.then(t, n);
}
__name(Wt, "Wt");
function Pt(e2) {
  return e2.toString();
}
__name(Pt, "Pt");
function $t(e2, t) {
  let n = t.url, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = b;
  f().setInt32(e2 + 4 * 1, o, true), f().setInt32(e2 + 4 * 0, _, true);
}
__name($t, "$t");
function Nt(e2, t) {
  let n = t.url, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = b;
  f().setInt32(e2 + 4 * 1, o, true), f().setInt32(e2 + 4 * 0, _, true);
}
__name(Nt, "Nt");
function Vt(e2) {
  return e2.value;
}
__name(Vt, "Vt");
function Bt(e2) {
  let t = e2.view;
  return s(t) ? 0 : d(t);
}
__name(Bt, "Bt");
function vt(e2) {
  let t = e2.original;
  return t.cnt-- == 1 ? (t.a = 0, true) : false;
}
__name(vt, "vt");
function Gt(e2, t, n) {
  return L(e2, t, 2498, N);
}
__name(Gt, "Gt");
function Ht(e2, t, n) {
  return L(e2, t, 2545, V);
}
__name(Ht, "Ht");
function Jt(e2, t) {
  let n = S(t), _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = b;
  f().setInt32(e2 + 4 * 1, o, true), f().setInt32(e2 + 4 * 0, _, true);
}
__name(Jt, "Jt");
function Xt() {
  let e2 = r.__wbindgen_export_4, t = e2.grow(4);
  e2.set(0, void 0), e2.set(t + 0, void 0), e2.set(t + 1, null), e2.set(t + 2, true), e2.set(t + 3, false);
}
__name(Xt, "Xt");
function Kt(e2) {
  return typeof e2 == "function";
}
__name(Kt, "Kt");
function Qt(e2) {
  let t = e2;
  return typeof t == "object" && t !== null;
}
__name(Qt, "Qt");
function Yt(e2) {
  return e2 === void 0;
}
__name(Yt, "Yt");
function Zt() {
  return r.memory;
}
__name(Zt, "Zt");
function en(e2, t) {
  let n = t, _ = typeof n == "string" ? n : void 0;
  var o = s(_) ? 0 : l(_, r.__wbindgen_malloc, r.__wbindgen_realloc), a = b;
  f().setInt32(e2 + 4 * 1, a, true), f().setInt32(e2 + 4 * 0, o, true);
}
__name(en, "en");
function tn(e2, t) {
  return g(e2, t);
}
__name(tn, "tn");
function nn(e2, t) {
  throw new Error(g(e2, t));
}
__name(nn, "nn");
var A = new WebAssembly.Instance(rn, { "./index_bg.js": w });
E(A.exports);
A.exports.__wbindgen_start?.();
var F = class extends _n {
  static {
    __name(this, "F");
  }
  async fetch(t) {
    return await z(t, this.env, this.ctx);
  }
  async queue(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
  async scheduled(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
};
var on = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(w).map((e2) => {
  on.includes(e2) | e2.startsWith("__") || (F.prototype[e2] = w[e2]);
});
var fn = F;
export {
  T as IntoUnderlyingByteSource,
  I as IntoUnderlyingSink,
  R as IntoUnderlyingSource,
  O as MinifyConfig,
  v as PolishConfig,
  k as R2Range,
  G as RequestRedirect,
  te as __wbg_String_8f0eb39a4a4c2f66,
  ne as __wbg_abort_410ec47a64ac6117,
  re as __wbg_abort_775ef1d17fc65868,
  _e as __wbg_append_8c7dd8d641a5f01b,
  oe as __wbg_body_018617e858cb7195,
  ce as __wbg_body_0b8fd1fe671660df,
  ie as __wbg_buffer_09165b52af8c5237,
  se as __wbg_buffer_609cc3eee51ed158,
  ue as __wbg_byobRequest_77d9adf63337edfb,
  fe as __wbg_byteLength_e674b853d9c77e1d,
  ae as __wbg_byteOffset_fd862df290ef848d,
  be as __wbg_call_672a4d21634d4a24,
  ge as __wbg_call_7cccdd69e0791ae2,
  de as __wbg_cancel_8a308660caa6cadf,
  we as __wbg_catch_a6e601879b2610e9,
  le as __wbg_cause_9940c4e8dfcd5129,
  pe as __wbg_cf_123509d53a2ea003,
  xe as __wbg_clearTimeout_b1115618e821c3b2,
  ye as __wbg_close_304cc1fef3466669,
  he as __wbg_close_5ce03e29be453811,
  me as __wbg_done_769e5ede4b31c67b,
  Re as __wbg_enqueue_bb16ba72f537dc9e,
  Fe as __wbg_entries_2a52db465d0421fb,
  Se as __wbg_error_524f506f44df1645,
  Te as __wbg_error_7534b8e9a36f1ab4,
  Ie as __wbg_fetch_3afbdcc7ddbf16fe,
  Oe as __wbg_fetch_509096533071c657,
  ke as __wbg_getRandomValues_3c9c0d586e575a16,
  Ee as __wbg_getReader_48e00749fe3f6089,
  ze as __wbg_getTime_46267b1c24877e30,
  Me as __wbg_get_67b2ba62fc30de12,
  je as __wbg_get_b9b93047fe3cf45b,
  qe as __wbg_getdone_d47073731acd3e74,
  Le as __wbg_getvalue_009dcd63692bee1f,
  Ae as __wbg_has_a5ea9117f258a0ec,
  Ce as __wbg_headers_7852a8ea641c1379,
  De as __wbg_headers_9cb51cfd2ac780a4,
  Ue as __wbg_httpProtocol_4cc3ab4fde2ecf82,
  We as __wbg_instanceof_Error_4d54113b22d20306,
  Pe as __wbg_instanceof_Response_f2cc20d9f7dfd644,
  $e as __wbg_iterator_9a24c88df860dc65,
  Ne as __wbg_length_a446193dc22c12f8,
  Ve as __wbg_method_3dcc854b644c5a56,
  Be as __wbg_new0_f788a2397c7ca929,
  ve as __wbg_new_018dcc2d6c8c2f6a,
  Ge as __wbg_new_23a2665fac83c611,
  He as __wbg_new_405e22f390576ce2,
  Je as __wbg_new_8a6f238a6ece86ea,
  Xe as __wbg_new_a12002a7f91c75be,
  Ke as __wbg_new_c68d7209be747379,
  Qe as __wbg_new_e25e5aab09ff45db,
  Ye as __wbg_newnoargs_105ed471475aaf50,
  Ze as __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a,
  et as __wbg_newwithintounderlyingsource_b47f6a6a596a7f24,
  tt as __wbg_newwithlength_a381634e90c276d4,
  nt as __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1,
  rt as __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e,
  _t as __wbg_newwithoptstrandinit_615a266ef226c260,
  ot as __wbg_newwithstrandinit_06c535e0a867c635,
  ct as __wbg_next_25feadfc0913fea9,
  it as __wbg_next_6574e1a8a62d1055,
  st as __wbg_now_2c95c9de01293173,
  ut as __wbg_performance_7a3ffd0b17f663ad,
  ft as __wbg_queueMicrotask_97d92b4fcc8a61c5,
  at as __wbg_queueMicrotask_d3219def82552485,
  bt as __wbg_read_a2434af1186cb56c,
  gt as __wbg_redirect_14b0c8193458f8c3,
  dt as __wbg_releaseLock_091899af97991d2e,
  wt as __wbg_resolve_4851785c9c5f573d,
  lt as __wbg_respond_1f279fa9f8edcb1c,
  pt as __wbg_setTimeout_ca12ead8b48245e2,
  xt as __wbg_set_65595bdd868b3009,
  yt as __wbg_set_bb8cecf6a62b9f46,
  E as __wbg_set_wasm,
  ht as __wbg_setbody_5923b78a95eedf29,
  mt as __wbg_setcredentials_c3a22f1cd105a2c6,
  Rt as __wbg_setheaders_3b47c898e8de6d44,
  Ft as __wbg_setheaders_834c0bdb6a8949ad,
  St as __wbg_sethighwatermark_793c99c89830c8e9,
  Tt as __wbg_setmethod_3c5280fe5d890842,
  It as __wbg_setmode_5dc300b865044b65,
  Ot as __wbg_setsignal_75b21ef3a81de905,
  kt as __wbg_setstatus_51b4fc011091cbb3,
  Et as __wbg_signal_02f4435f82019061,
  zt as __wbg_signal_aaf9ad74119f20a4,
  Mt as __wbg_stack_0ed75d68575b0f3c,
  jt as __wbg_static_accessor_GLOBAL_88a902d13a557d07,
  qt as __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0,
  Lt as __wbg_static_accessor_SELF_37c5d418e4bf5819,
  At as __wbg_static_accessor_WINDOW_5de37043a91a9c40,
  Ct as __wbg_status_f6360336ca686bf0,
  Dt as __wbg_stringify_f7ed6987935b4a24,
  Ut as __wbg_then_44b73946d2fb3e7d,
  Wt as __wbg_then_48b406749878a531,
  Pt as __wbg_toString_c813bbd34d063839,
  $t as __wbg_url_8f9653b899456042,
  Nt as __wbg_url_ae10c34ca209681d,
  Vt as __wbg_value_cd1ffa7b1ab794f1,
  Bt as __wbg_view_fd8a56e8983f448d,
  vt as __wbindgen_cb_drop,
  Gt as __wbindgen_closure_wrapper4896,
  Ht as __wbindgen_closure_wrapper5060,
  Jt as __wbindgen_debug_string,
  Xt as __wbindgen_init_externref_table,
  Kt as __wbindgen_is_function,
  Qt as __wbindgen_is_object,
  Yt as __wbindgen_is_undefined,
  Zt as __wbindgen_memory,
  en as __wbindgen_string_get,
  tn as __wbindgen_string_new,
  nn as __wbindgen_throw,
  fn as default,
  z as fetch,
  rn as wasmModule
};
//# sourceMappingURL=shim.js.map
