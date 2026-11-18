// Internal: adapts the generated classes in place: typed errors, `create`,
// value getters as properties, fallible optional strings merged from their
// `*Present` pairs and byte results as `Uint8Array` copies (see members.mjs).
// Both `fatoority` and `fatoority/raw` import this module, so the classes
// behave the same through either entry point and instances returned anywhere
// share one prototype.
import * as generated from './generated/index.mjs';
import { FatooraError, fromBindingError } from './errors.mjs';
import { byteResults, optionalResults, properties } from './members.mjs';

const { BindingError } = generated;
const wrapped = Symbol('fatoority.wrapped');

function translate(error) {
  // Generated code throws `Error('BindingError: ...', { cause: BindingError })`.
  if (error instanceof Error && !(error instanceof FatooraError) && error.cause instanceof BindingError) {
    return fromBindingError(error.cause);
  }
  return error;
}

/**
 * Return `fn` under `name`, converting its result and throwing the typed errors
 * of errors.mjs with a stack that starts at the caller.
 */
function adapt(name, fn, convert) {
  const adapter = {
    // A method shorthand keeps `this` and the name, without a prototype.
    [name](...args) {
      try { return convert(fn.apply(this, args)); }
      catch (error) {
        const typed = translate(error);
        if (typed !== error) Error.captureStackTrace?.(typed, adapter); // V8 only
        throw typed;
      }
    },
  }[name];
  adapter[wrapped] = true;
  return adapter;
}

const same = (value) => value;

function adaptMembers(target, skip, shape = {}) {
  const { properties = [], optionals = {}, booleans = [] } = shape;
  // `x` is null when `xPresent` is false; `xPresent` itself is not public.
  const present = {};
  for (const [name, has] of Object.entries(optionals)) {
    present[name] = target[has];
    delete target[has];
  }
  for (const name of Object.getOwnPropertyNames(target)) {
    if (skip.has(name)) continue;
    const descriptor = Object.getOwnPropertyDescriptor(target, name);
    if (typeof descriptor.value !== 'function' || descriptor.value[wrapped]) continue; // getters, enum singletons
    const convert = booleans.includes(name) ? Boolean : same;
    const has = present[name];
    const value = descriptor.value;
    const fn = has ? function (...args) { return has.call(this) ? value.apply(this, args) : null; } : value;
    const adapted = adapt(name, fn, convert);
    if (properties.includes(name)) {
      // A getter needs no arguments, so the adapter serves as one directly.
      Object.defineProperty(target, name, { get: adapted, enumerable: false, configurable: true });
    } else {
      Object.defineProperty(target, name, { ...descriptor, value: adapted });
    }
  }
}

// Diplomat 0.16's JS backend returns plain `bool` results as the WASM integer
// 0/1 although the declarations say `boolean`. tests/runtime.test.mjs checks
// this list against every `(): boolean` instance method in the declarations.
export const integerBooleanMethods = {
  AllowanceCharge: ['hasVatRate'],
  BytesList: ['isEmpty'],
  InvoiceLineItem: ['hasVatRate', 'hasTaxTotal', 'hasGrossAmount'],
  PaymentMeans: ['hasAccount'],
  VatBreakdown: ['hasVatRate'],
  ZatcaValidationReport: ['isComplete', 'hasErrors', 'isValid'],
};

const staticSkip = new Set(['length', 'name', 'prototype']);
const prototypeSkip = new Set(['constructor']);

for (const [className, value] of Object.entries(generated)) {
  if (typeof value !== 'function' || value === BindingError) continue;
  if (typeof value.getAllEntries === 'function') { // an enum class, such as `InvoiceFlag.bit`
    adaptMembers(value.prototype, prototypeSkip, { properties: properties[className] ?? [] });
    continue;
  }
  // Diplomat names a Rust `new` constructor `new_`; the public name is `create`.
  // Rename it before adapting so `create` reports errors at the caller.
  const generatedNew = value.new_;
  if (typeof generatedNew === 'function') {
    if (Object.hasOwn(value, 'create')) throw new Error(`${className} already has create`);
    const create = { create(...args) { return generatedNew.apply(this, args); } }.create;
    Object.defineProperty(value, 'create', { value: create, writable: true, configurable: true });
    delete value.new_;
  }
  // Diplomat copies a `&[u8]` result into a `number[]`; the public method
  // (`Bytes.toUint8Array`) returns a `Uint8Array` copy instead.
  for (const [generatedName, name] of Object.entries(byteResults[className] ?? {})) {
    const slice = value.prototype[generatedName];
    if (typeof slice !== 'function') throw new Error(`${className}.${generatedName} is missing`);
    const copy = { [name]() { return Uint8Array.from(slice.call(this)); } }[name];
    Object.defineProperty(value.prototype, name, { value: copy, writable: true, configurable: true });
    delete value.prototype[generatedName];
  }
  adaptMembers(value, staticSkip);
  // Value getters become read-only properties and `*Present` pairs nullable strings
  // (members.mjs, generated by scripts/generate_js_facade.py).
  adaptMembers(value.prototype, prototypeSkip, {
    properties: properties[className], optionals: optionalResults[className],
    booleans: integerBooleanMethods[className],
  });
}
