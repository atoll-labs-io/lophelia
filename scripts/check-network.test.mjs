import assert from "node:assert/strict";
import { test } from "node:test";
import { findNetworkApis } from "./check-network.mjs";

test("fetch is refused", () => {
  assert.deepEqual(findNetworkApis("const r=fetch(u);"), ["fetch"]);
});

test("XMLHttpRequest is refused", () => {
  assert.deepEqual(findNetworkApis("var x=new XMLHttpRequest;"), ["XMLHttpRequest"]);
});

test("WebSocket is refused", () => {
  assert.deepEqual(findNetworkApis("new WebSocket('ws://h')"), ["WebSocket"]);
});

test("Obsidian's requestUrl is refused", () => {
  assert.deepEqual(findNetworkApis("var o=require('obsidian');o.requestUrl({url:u})"), ["requestUrl"]);
});

test("http, https and net modules are refused, with or without node:", () => {
  for (const source of [
    'require("http")',
    "require('node:https')",
    'import { connect } from "net"',
    'await import("node:http")',
  ]) {
    assert.deepEqual(findNetworkApis(source), ["http/https/net module"], source);
  }
});

test("a bundle without network API passes", () => {
  const bundle =
    'var o=require("obsidian"),p=require("path");class V extends o.ItemView{onOpen(){this.contentEl.setText("Moteur 0.1.0")}}';
  assert.deepEqual(findNetworkApis(bundle), []);
});

test("an identifier that only contains fetch passes", () => {
  assert.deepEqual(findNetworkApis("function prefetch(){return fetched_}var http2x=1;"), []);
});
