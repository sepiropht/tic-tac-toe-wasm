(window.webpackJsonp=window.webpackJsonp||[]).push([[1],{343:function(t,r,e){"use strict";(function(t){e.d(r,"c",(function(){return f})),e.d(r,"a",(function(){return s})),e.d(r,"b",(function(){return _})),e.d(r,"d",(function(){return d})),e.d(r,"h",(function(){return p})),e.d(r,"g",(function(){return l})),e.d(r,"e",(function(){return h})),e.d(r,"f",(function(){return w})),e.d(r,"i",(function(){return b}));var n=e(344);let o=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});o.decode();let i=null;function c(t,r){return o.decode((null!==i&&i.buffer===n.o.buffer||(i=new Uint8Array(n.o.buffer)),i).subarray(t,t+r))}function u(t,r){if(!(t instanceof r))throw new Error("expected instance of "+r.name);return t.ptr}function a(t){return()=>{throw new Error(t+" is not defined")}}const f=Object.freeze({EMPTY:0,0:"EMPTY",PLAYER1:1,1:"PLAYER1",PLAYER2:2,2:"PLAYER2",TIE:3,3:"TIE"});class s{static __wrap(t){const r=Object.create(s.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.a(t)}constructor(){var t=n.f();return s.__wrap(t)}update_scores(t){n.g(this.ptr,t)}get_best_move(t){u(t,_);var r=n.e(this.ptr,t.ptr);return d.__wrap(r)}aiMove(t,r){u(t,_);var e=n.d(this.ptr,t.ptr,r);return d.__wrap(e)}}class _{static __wrap(t){const r=Object.create(_.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.b(t)}constructor(t){var r=n.m(t);return _.__wrap(r)}get_dim(){return n.k(this.ptr)>>>0}getCell(t,r){return n.j(this.ptr,t,r)>>>0}get_index(t,r){return n.l(this.ptr,t,r)>>>0}playerMove(t,r,e){n.n(this.ptr,t,r,e)}checkWin(){return n.h(this.ptr)>>>0}static clone_board(t){u(t,_);var r=n.i(t.ptr);return _.__wrap(r)}}class d{static __wrap(t){const r=Object.create(d.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.c(t)}getX(){return n.p(this.ptr)>>>0}getY(){return n.q(this.ptr)>>>0}}const p=function(t,r){console.time(c(t,r))},l=function(t,r){console.timeEnd(c(t,r))},h="function"==typeof Math.floor?Math.floor:a("Math.floor"),w="function"==typeof Math.random?Math.random:a("Math.random"),b=function(t,r){throw new Error(c(t,r))}}).call(this,e(345)(t))},344:function(t,r,e){"use strict";var n=e.w[t.i];t.exports=n;e(343);n.r()},345:function(t,r){t.exports=function(t){if(!t.webpackPolyfill){var r=Object.create(t);r.children||(r.children=[]),Object.defineProperty(r,"loaded",{enumerable:!0,get:function(){return r.l}}),Object.defineProperty(r,"id",{enumerable:!0,get:function(){return r.i}}),Object.defineProperty(r,"exports",{enumerable:!0}),r.webpackPolyfill=1}return r}},346:function(t,r,e){"use strict";e.r(r);var n=e(343);e.d(r,"Cell",(function(){return n.c})),e.d(r,"Ai",(function(){return n.a})),e.d(r,"Board",(function(){return n.b})),e.d(r,"Point",(function(){return n.d})),e.d(r,"__wbg_time_5fc08471a5677727",(function(){return n.h})),e.d(r,"__wbg_timeEnd_a09962993fd6d095",(function(){return n.g})),e.d(r,"__wbg_floor_f64e9321ec658305",(function(){return n.e})),e.d(r,"__wbg_random_69f01e50b30c8d0a",(function(){return n.f})),e.d(r,"__wbindgen_throw",(function(){return n.i}))}}]);