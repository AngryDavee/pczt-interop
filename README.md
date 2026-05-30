# pczt-interop (proof of concept)

A small, runnable proof of concept for cross-implementation interoperability testing of
[PCZT](https://github.com/zcash/librustzcash/tree/main/pczt) (Partially Created Zcash
Transactions). PCZT is the format that lets one tool build a Zcash transaction, another
sign it, and another finalize it. It is the basis for hardware wallets, multisig, and
air-gapped signing.

Several teams now implement PCZT independently (OneKey, Ledger, Hito, Keystone, and the
reference implementation in `librustzcash`), but there is no shared way to confirm they
interoperate: the `pczt` crate has a single end-to-end test where `librustzcash` plays
every role at once, and `zcash-test-vectors` contains no PCZT vectors.

This repo is the seed of a fuller test suite (proposed as a Zcash Community Grant). It
demonstrates the approach on real code.

## What it does

Running it (`cargo run`) does four things:

1. **Creator role.** Builds a base PCZT (v5 transaction, no spends or outputs) with the
   `pczt` crate's `Creator`.
2. **Round-trip check.** Serializes the PCZT, parses it back, and serializes again,
   asserting the bytes are identical. Byte-stable serialization is the core property
   every implementation must agree on to interoperate.
3. **Combiner role (a role boundary).** Two separate parties each hold the base PCZT, and
   the `Combiner` role merges them into one. This is the seed of the full
   "create on A, sign on B, finalize on C" harness.
4. **Test vector.** Writes a canonical vector (`vectors/empty_v5.pczt` plus a JSON
   manifest with hex and metadata) of the kind that would be contributed to
   `zcash-test-vectors`.

## Run it

```bash
cargo run
```

Expected output ends with the inspector summary, a successful combine, and the written
vector files. Example:

```
[1] round-trip OK: 101 bytes, byte-stable
[2] inspector
    tx_version          : 5
    ...
[3] combiner OK: merged two parties' PCZTs into 101 bytes
[4] wrote vectors/empty_v5.pczt and vectors/empty_v5.json
```

## Test it

```bash
cargo test
```

Covers byte-stable round-trip, the Combiner merge, and rejection of non-PCZT input.

## Scope and honesty

This is a proof of concept, not the finished suite. It uses empty (anchor-zero) bundles so
it builds and runs without proving keys or a node. The fuller grant work adds: vectors for
real transparent, Sapling, and Orchard cases; a `PcztRole` adapter trait so independent
implementations can be tested against each other (including sign-only hardware devices via
exported PCZT files); and per-role diagnostics. See the grant application for milestones.

## License

MIT OR Apache-2.0.
