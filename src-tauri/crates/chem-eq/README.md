# `chem-eq`

`chem-eq` parses chemical equations into elements, mol ratio,
direction of reaction and more. Chemical equations can be balanced
with the crate feature `balance`. There are serde implementations behind
feature `serde`.

Get started by looking at the docs or examples. An example application can be found [here](https://github.com/BeaconBrigade/balance-tui.git)

## v0.3.0

This release includes many changes from bug fixes, to improved error types.
Most of the breaking changes involve modified error types. There are added
methods to directly parse elements and compounds. Additional changes include
documentation improvements, more methods on `Compound` and `Equation` to make
your life easier and more test cases.
