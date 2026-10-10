import assert from 'node:assert/strict';
import { isUtf8 } from 'node:buffer';
import test from 'node:test';

import { canonicalJsonBytes, canonicalNumberViolation, parseJsonStrict } from './model-contract.mjs';

// #679 made canonicalJsonBytes and parseJsonStrict several times faster by serializing/parsing
// directly instead of building intermediate copies. The contract is "the same bytes, the same
// refusals", so this file keeps the PREVIOUS implementations verbatim as reference oracles
// (everything between the two markers below) and requires the live ones to agree with them,
// byte for byte and refusal for refusal, on a fixed corpus and on a large seeded fuzz.
const PLAIN = Object.getPrototypeOf({});
// ---- reference implementation (pre-#679, unmodified apart from names) ----
function assertUnicodeScalars(value) {
  for (let i = 0; i < value.length; i += 1) {
    const unit = value.charCodeAt(i);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const next = value.charCodeAt(i + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) throw new TypeError('string must contain Unicode scalar values');
      i += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) {
      throw new TypeError('string must contain Unicode scalar values');
    }
  }
}


function defineJsonProperty(target, key, value) {
  Object.defineProperty(target, key, { value, enumerable: true, configurable: true, writable: true });
}


function normalizeJson(value, seen = new Set()) {
  if (value === null || typeof value === 'boolean') return value;
  if (typeof value === 'string') {
    assertUnicodeScalars(value);
    return value;
  }
  if (typeof value === 'number') {
    const violation = canonicalNumberViolation(value);
    if (violation) throw new TypeError(`JSON number ${violation}`);
    return Object.is(value, -0) ? 0 : value;
  }
  if (!value || typeof value !== 'object') throw new TypeError('value is not JSON data');
  if (seen.has(value)) throw new TypeError('JSON value must be acyclic');
  seen.add(value);
  try {
    if (Array.isArray(value)) {
      for (let index = 0; index < value.length; index += 1) {
        if (!Object.hasOwn(value, index)) throw new TypeError('sparse array is not JSON data');
      }
      return value.map((entry) => normalizeJson(entry, seen));
    }
    if (Object.getPrototypeOf(value) !== PLAIN && Object.getPrototypeOf(value) !== null) throw new TypeError('JSON object must be plain');
    const out = {};
    for (const key of Object.keys(value).sort()) {
      assertUnicodeScalars(key);
      if (value[key] === undefined) throw new TypeError('undefined is not JSON data');
      defineJsonProperty(out, key, normalizeJson(value[key], seen));
    }
    return out;
  } finally {
    seen.delete(value);
  }
}


function referenceCanonicalJsonBytes(value) {
  return Buffer.from(JSON.stringify(normalizeJson(value)), 'utf8');
}


function referenceParseJsonStrict(input, options = {}) {
  let text;
  if (Buffer.isBuffer(input) || input instanceof Uint8Array) {
    const bytes = Buffer.from(input);
    if (!isUtf8(bytes)) throw new SyntaxError('JSON input is not valid UTF-8');
    text = bytes.toString('utf8');
  } else {
    text = String(input);
  }
  const maxBytes = options.maxBytes ?? 16 * 1024 * 1024;
  const maxDepth = options.maxDepth ?? 128;
  if (Buffer.byteLength(text, 'utf8') > maxBytes) throw new SyntaxError('JSON input exceeds its byte limit');
  let cursor = 0;
  // RFC 8259 permits exactly space, tab, carriage return, and line feed between tokens.
  // JavaScript's `\s` also accepts BOM, NBSP, and Unicode separators.
  const white = () => { while (text[cursor] === ' ' || text[cursor] === '\t' || text[cursor] === '\r' || text[cursor] === '\n') cursor += 1; };
  const fail = (message) => { throw new SyntaxError(`${message} at byte ${cursor}`); };
  const stringValue = () => {
    if (text[cursor] !== '"') fail('expected JSON string');
    const start = cursor;
    cursor += 1;
    let escaped = false;
    for (; cursor < text.length; cursor += 1) {
      const ch = text[cursor];
      if (escaped) { escaped = false; continue; }
      if (ch === '\\') { escaped = true; continue; }
      if (ch === '"') {
        cursor += 1;
        const value = JSON.parse(text.slice(start, cursor));
        assertUnicodeScalars(value);
        return value;
      }
      if (ch.charCodeAt(0) < 0x20) fail('unescaped control character');
    }
    fail('unterminated JSON string');
  };
  const value = (depth) => {
    if (depth > maxDepth) fail('JSON nesting exceeds its limit');
    white();
    const ch = text[cursor];
    if (ch === '"') return stringValue();
    if (ch === '{') {
      cursor += 1;
      const out = {};
      const keys = new Set();
      white();
      if (text[cursor] === '}') { cursor += 1; return out; }
      for (;;) {
        white();
        const key = stringValue();
        if (keys.has(key)) fail(`duplicate JSON key '${key}'`);
        keys.add(key);
        white();
        if (text[cursor] !== ':') fail('expected colon');
        cursor += 1;
        defineJsonProperty(out, key, value(depth + 1));
        white();
        if (text[cursor] === '}') { cursor += 1; return out; }
        if (text[cursor] !== ',') fail('expected comma');
        cursor += 1;
      }
    }
    if (ch === '[') {
      cursor += 1;
      const out = [];
      white();
      if (text[cursor] === ']') { cursor += 1; return out; }
      for (;;) {
        out.push(value(depth + 1));
        white();
        if (text[cursor] === ']') { cursor += 1; return out; }
        if (text[cursor] !== ',') fail('expected comma');
        cursor += 1;
      }
    }
    for (const [token, result] of [['true', true], ['false', false], ['null', null]]) {
      if (text.startsWith(token, cursor)) { cursor += token.length; return result; }
    }
    const match = text.slice(cursor).match(/^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/);
    if (!match) fail('invalid JSON value');
    cursor += match[0].length;
    const number = Number(match[0]);
    if (!Number.isFinite(number)) fail('JSON number must be finite');
    if (!/[.eE]/.test(match[0]) && !Number.isSafeInteger(number)) fail('JSON integer must be a safe integer');
    return Object.is(number, -0) ? 0 : number;
  };
  const parsed = value(0);
  white();
  if (cursor !== text.length) fail('trailing JSON data');
  return parsed;
}


// ---- end reference implementation ----

function outcome(operation) {
  try { return { ok: operation() }; }
  catch (error) { return { error: `${error.constructor.name}: ${error.message}` }; }
}

function assertSameOutcome(actual, expected, label) {
  assert.deepEqual(actual, expected, label);
}

function lcg(seed) {
  let state = seed >>> 0;
  return () => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return state / 0x100000000;
  };
}

const KEYS = ['a', 'b', 'z', 'A', '__proto__', 'constructor', 'toString', 'hasOwnProperty', '0', '1', '2', '10', '9',
  '01', '4294967294', '4294967295', '9007199254740993', 'é', '中', '😀', 'k"q', 'k\\b', '', 'x y',
  '\ud800'];
const STRINGS = ['', 'a', 'é', '中', '😀', '\ud800', '\udc00', 'a\ud800b', '"', '\\', '\n', '\u0001',
  ' ', ' ', 'long'.repeat(30)];
const SCALARS = [null, true, false, 0, -0, 1.5, 1e21, 123456789012345680, Number.NaN, Number.POSITIVE_INFINITY,
  undefined, 9007199254740993, 1e-7, 5e-324, -1, 2 ** 53];

function generated(random, depth = 0) {
  const pick = (list) => list[Math.floor(random() * list.length)];
  const roll = random();
  if (depth > 4 || roll < 0.35) return random() < 0.5 ? pick(STRINGS) : pick(SCALARS);
  if (roll < 0.6) {
    const array = Array.from({ length: Math.floor(random() * 4) }, () => generated(random, depth + 1));
    if (random() < 0.03) array.length += 1;
    return array;
  }
  const object = {};
  for (let index = Math.floor(random() * 5); index > 0; index -= 1) {
    Object.defineProperty(object, pick(KEYS), {
      value: generated(random, depth + 1), enumerable: true, configurable: true, writable: true,
    });
  }
  return random() < 0.02 ? Object.setPrototypeOf(object, null) : object;
}

test('canonicalJsonBytes matches the pre-#679 implementation byte for byte and refusal for refusal', () => {
  const corpus = [
    { b: 1, a: { 10: 'x', 2: 'y', c: [1, 2], 1: null }, '4294967295': 1, '4294967294': 2, zz: -0 },
    { '__proto__': 1, constructor: 2, toString: 3 },
    JSON.parse('{"__proto__":{"x":1},"a":1}'),
    ['é', '中', '😀', '"\\\n\u0001 '],
    [], {}, '', 0, -0, null, true, 1e-7, 123456789.125, 9007199254740991,
    { n: 1e21 }, { n: 9007199254740993 }, { n: Number.NaN }, { s: '\ud800' }, { '\udc00': 1 }, { u: undefined },
    [1, , 3], { f() {} }, new Date(0), Object.create({ inherited: 1 }), [Symbol('x')],
  ];
  const cyclic = { a: 1 }; cyclic.self = cyclic;
  corpus.push(cyclic);
  for (const [index, value] of corpus.entries()) {
    assertSameOutcome(
      outcome(() => canonicalJsonBytes(value).toString('hex')),
      outcome(() => referenceCanonicalJsonBytes(value).toString('hex')),
      `corpus entry ${index}`,
    );
  }
  const random = lcg(0x679);
  let accepted = 0;
  for (let round = 0; round < 3000; round += 1) {
    const value = generated(random);
    const expected = outcome(() => referenceCanonicalJsonBytes(value).toString('hex'));
    assertSameOutcome(outcome(() => canonicalJsonBytes(value).toString('hex')), expected, `fuzz round ${round}`);
    if (expected.ok !== undefined) accepted += 1;
  }
  assert.ok(accepted > 500, 'the fuzz must exercise accepted values, not only refusals');
});

test('parseJsonStrict matches the pre-#679 implementation on valid, mutated and hostile text', () => {
  const random = lcg(0x1679);
  const mutations = [
    (text) => text, (text) => text.replace(',', ', '), (text) => text.replace('"', '"\\'), (text) => text.slice(0, -1),
    (text) => `${text} x`, (text) => ` ${text}\n`, (text) => text.replace(':', '::'),
    (text) => text.replace(/\d+/, '0123'), (text) => text.replace('"', "'"),
    (text) => text.replace('{', '{"a":1,"a":2,'), (text) => text.replace(/\d/, '$&e999'),
    (text) => text.replace('"', '"\\ud800'), (text) => text.replace('"', '"\\u00'), (text) => `﻿${text}`,
    (text) => text.replace('[', '[,'), (text) => text.replace('"', '"\u0001'),
    (text) => text.replace('"', '"\\ud83d\\ude00'), (text) => text.replace('"', '"\\x'),
    (text) => text.replace('{', '{"__proto__":1,"__proto__":2,'),
    (text) => text.replace('{', '{"constructor":1,"constructor":2,'),
    (text) => text.replace(/\d+/, '9007199254740993'), (text) => text.replace(/\d+/, '-0'),
    (text) => text.replace(/\d+/, '1.0E+2'), (text) => text.replace(/\d+/, '1.'),
    (text) => text.replace('{', '{"a":tru'), (text) => text.replace('[', '[nul'), (text) => `${text}\t\r\n `,
  ];
  let parsed = 0;
  for (let round = 0; round < 200; round += 1) {
    const value = generated(random);
    const base = outcome(() => referenceCanonicalJsonBytes(value).toString('utf8')).ok ?? (JSON.stringify(value) ?? 'x');
    for (const [index, mutate] of mutations.entries()) {
      const text = mutate(base);
      for (const maxDepth of [2, 128]) {
        const expected = outcome(() => JSON.stringify(referenceParseJsonStrict(text, { maxDepth })));
        assertSameOutcome(outcome(() => JSON.stringify(parseJsonStrict(text, { maxDepth }))), expected,
          `round ${round} mutation ${index} depth ${maxDepth}: ${JSON.stringify(text)}`);
        if (expected.ok !== undefined) parsed += 1;
      }
    }
  }
  assert.ok(parsed > 600, 'the fuzz must exercise accepted documents, not only refusals');
  // Buffers go through the UTF-8 gate; strings skip it. Both agree with the reference.
  for (const bytes of [Buffer.from('{"a":"é"}'), Buffer.from([0x7b, 0x22, 0xff, 0x22, 0x3a, 0x31, 0x7d]),
    Buffer.from('[1,2]'), new Uint8Array([0x5b, 0x5d])]) {
    assertSameOutcome(outcome(() => JSON.stringify(parseJsonStrict(bytes))),
      outcome(() => JSON.stringify(referenceParseJsonStrict(bytes))));
  }
});

test('parsed objects keep own, writable, enumerable members even for names Object.prototype owns', () => {
  const parsed = parseJsonStrict('{"__proto__":{"polluted":true},"constructor":1,"toString":"x","valueOf":null,"a":2}');
  assert.equal(Object.getPrototypeOf(parsed), Object.prototype);
  assert.equal(parsed.polluted, undefined);
  assert.deepEqual(Object.keys(parsed), ['__proto__', 'constructor', 'toString', 'valueOf', 'a']);
  for (const key of Object.keys(parsed)) {
    assert.deepEqual(Object.getOwnPropertyDescriptor(parsed, key),
      { value: parsed[key], writable: true, enumerable: true, configurable: true }, key);
  }
  // A duplicate of such a name is still a duplicate.
  assert.throws(() => parseJsonStrict('{"constructor":1,"constructor":2}'), /duplicate JSON key 'constructor'/);
  assert.throws(() => parseJsonStrict('{"__proto__":1,"__proto__":2}'), /duplicate JSON key '__proto__'/);
  assert.equal(canonicalNumberViolation(1), null);
});

test('canonical output keeps the engine ordering of array-index keys', () => {
  // JSON.stringify of an object emits array-index keys first, ascending; the canonical bytes have
  // always been defined through it, so they are part of the artifact byte contract.
  assert.equal(canonicalJsonBytes({ b: 1, 10: 'x', a: 2, 2: 'y', 4294967295: 'z' }).toString(),
    '{"2":"y","10":"x","4294967295":"z","a":2,"b":1}');
});
