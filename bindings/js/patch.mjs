// Internal: adapts the generated classes in place. Both `fatoority` and
// `fatoority/raw` import this module, so the classes behave the same through
// either entry point and instances returned anywhere share one prototype.
import * as generated from './generated/index.mjs';
import { FatooraError, fromBindingError } from './errors.mjs';

const { BindingError } = generated;
const wrapped = Symbol('fatoority.wrapped');

function translate(error) {
  // Generated code throws `Error('BindingError: ...', { cause: BindingError })`.
  if (error instanceof Error && !(error instanceof FatooraError) && error.cause instanceof BindingError) {
    return fromBindingError(error.cause);
  }
  return error;
}

function wrap(fn) {
  if (fn[wrapped]) return fn;
  const wrapper = {
    // A method shorthand keeps `this` and the original name, without a prototype.
    [fn.name](...args) {
      try { return fn.apply(this, args); }
      catch (error) {
        const typed = translate(error);
        // Start the stack at the caller, not inside this adapter (V8 only).
        if (typed !== error) Error.captureStackTrace?.(typed, wrapper);
        throw typed;
      }
    },
  }[fn.name];
  wrapper[wrapped] = true;
  return wrapper;
}

function wrapMethods(target, skip) {
  for (const name of Object.getOwnPropertyNames(target)) {
    if (skip.has(name)) continue;
    const descriptor = Object.getOwnPropertyDescriptor(target, name);
    if (typeof descriptor.value !== 'function') continue; // getters and enum singletons
    Object.defineProperty(target, name, { ...descriptor, value: wrap(descriptor.value) });
  }
}

const staticSkip = new Set(['length', 'name', 'prototype']);
const prototypeSkip = new Set(['constructor']);

for (const value of Object.values(generated)) {
  // Enum classes (with `getAllEntries`) and BindingError never throw a BindingError.
  if (typeof value !== 'function' || value === BindingError || typeof value.getAllEntries === 'function') continue;
  // Diplomat names a Rust `new` constructor `new_`; the public name is `create`.
  // Define it before wrapping so both names report errors at the caller.
  const generatedNew = value.new_;
  if (typeof generatedNew === 'function' && !Object.hasOwn(value, 'create')) {
    const create = { create(...args) { return generatedNew.apply(this, args); } }.create;
    Object.defineProperty(value, 'create', { value: create, writable: true, configurable: true });
  }
  wrapMethods(value, staticSkip);
  wrapMethods(value.prototype, prototypeSkip);
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
for (const [name, methods] of Object.entries(integerBooleanMethods)) {
  const prototype = generated[name].prototype;
  for (const method of methods) {
    const integer = prototype[method];
    const boolean = { [method]() { return Boolean(integer.call(this)); } }[method];
    Object.defineProperty(prototype, method, { value: boolean, writable: true, configurable: true });
  }
}
