#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
import test from 'ava';

import {
  helloWorld,
  asyncAdd
} from '../index.js';

test('hello world', (t) => {
  t.is(helloWorld([1, 2, 4]), 3);
});

test('async add', async(t) => {
  t.is((await asyncAdd(10, Buffer.from([1, 3]))), 12);
});
