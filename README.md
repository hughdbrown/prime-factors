# Purpose
Implement a method to convert an integer to a list of prime factors.

# Testing
Test this from cargo:
```
> cargo test
running 16 tests
test tests::factors_10 ... ok
test tests::factors_2_3_5 ... ok
test tests::factors_160 ... ok
test tests::factors_2 ... ok
test tests::factors_4 ... ok
test tests::factors_3 ... ok
test tests::factors_5 ... ok
test tests::factors_6 ... ok
test tests::factors_60 ... ok
test tests::test_find_factors_sieve ... ok
test tests::factors_general ... ok
test tests::factors_12345678901234 ... ok
test tests::factors_64374108854777 ... ok
test tests::factors_312680865509917 ... ok
test tests::factors_6795742697625173 ... ok
test tests::factors_44711100255155897 ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.68s
```

# main
The main entry point tests the raw factorization code against code that starts with a vector of primes.
The course text suggests this optimization is worthwhile, at least when the amortized cost of creation
of the primes is considered.

Here are the results for this test case when using a release build:
```
Build sieve: 439.330042ms seconds

find_factors: 67.381086ms seconds
[45269999, 987654103]
Product: 44711100255155897

find_factors_sieve: 24.847313ms seconds
[45269999, 987654103]
Product: 44711100255155897
```
