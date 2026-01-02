# Cellular Automatons on Rust

[![Video](assets/animation.webp)](assets/animation.webp)

## Benchmark

Tests name format:
`test{order_id}_{steps}_{Width}x{Height}`.

```js
running 10 tests
test automaton::tests::test11_100
Test 0: 0.069s (0.200s): OK

test automaton::tests::test12_100_200x200
Test 0: 0.254s (0.400s): OK

test automaton::tests::test13_100_300x300
Test 0: 0.549s (1.500s): OK

test automaton::tests::test14_100_400x400
Test 0: 1.109s (2.000s): OK

test automaton::tests::test15_100_1000x1000
Test 0: 12.981s (20.000s): OK

test automaton::tests::test21_1000
Test 0: 0.684s (1.500s): OK

test automaton::tests::test22_1000_200x200
Test 0: 2.608s (3.500s): OK

test automaton::tests::test31_50_100x100
Test 0: 0.030s (0.100s): OK

test automaton::tests::test32_50_200x200
Test 0: 0.119s (0.300s): OK

test automaton::tests::test33_50_1000x1000
Test 0: 6.968s (15.000s): OK
```
