#!/usr/bin/env coffee

> ava:test
  ../index.js > helloWorld asyncAdd

test 'hello world', (t)=>
  t.is(helloWorld([1,2,4]),3)
  return

test 'async add', (t)=>
  t.is(await asyncAdd(10,Buffer.from([1,3])),12)
  return
