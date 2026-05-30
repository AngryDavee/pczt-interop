//! PCZT interop proof of concept.
//!
//! This is a small, runnable demonstration of the approach behind the grant
//! application "PCZT Interoperability Test Suite". It does four things:
//!
//!   1. Creates a base PCZT using the Creator role from the `pczt` crate.
//!   2. Round-trips it (serialize -> parse -> re-serialize) and checks that the
//!      bytes are stable. Byte-stable serialization is the core property every
//!      PCZT implementation must agree on to interoperate.
//!   3. Demonstrates a role boundary: two separate parties each hold the base
//!      PCZT, and the Combiner role merges them into one. This is the seed of
//!      the "create on A, sign on B, finalize on C" harness.
//!   4. Emits a canonical test vector (raw bytes + a JSON manifest) of the kind
//!      that would be contributed to `zcash-test-vectors`, which currently has
//!      no PCZT vectors.
//!
//! Run with: `cargo run`

use std::fs;

use pczt::Pczt;
use pczt::roles::combiner::Combiner;
use pczt::roles::creator::Creator;

/// NU5 consensus branch ID (used here only as a fixed, deterministic value for
/// the vector; the Creator just stores it).
const NU5_BRANCH_ID: u32 = 0xC2D6_D0B4;

/// SLIP-44 coin type for ZEC.
const ZEC_COIN_TYPE: u32 = 133;

/// Build a base PCZT with no spends or outputs, using the Creator role.
fn make_base_pczt(expiry_height: u32) -> Pczt {
    Creator::new(
        NU5_BRANCH_ID,
        expiry_height,
        ZEC_COIN_TYPE,
        [0u8; 32], // sapling anchor (empty for this PoC)
        [0u8; 32], // orchard anchor (empty for this PoC)
    )
    .build()
}

fn main() {
    println!("== PCZT interop proof of concept ==\n");

    // 1. Creator role: build the base PCZT.
    let pczt = make_base_pczt(0);

    // 2. Round-trip and check byte-stability.
    let bytes = pczt.serialize();
    let parsed = Pczt::parse(&bytes).expect("a PCZT we just serialized should parse");
    let bytes_again = parsed.serialize();
    assert_eq!(
        bytes, bytes_again,
        "serialization is not byte-stable across a round-trip"
    );
    println!("[1] round-trip OK: {} bytes, byte-stable", bytes.len());

    // 3. Inspector: print the global fields and bundle counts.
    let g = pczt.global();
    println!("\n[2] inspector");
    println!("    tx_version          : {}", g.tx_version());
    println!("    version_group_id    : {:#010x}", g.version_group_id());
    println!("    consensus_branch_id : {:#010x}", g.consensus_branch_id());
    println!("    expiry_height       : {}", g.expiry_height());
    println!("    coin_type           : {}", ZEC_COIN_TYPE);
    println!("    transparent inputs  : {}", pczt.transparent().inputs().len());
    println!("    transparent outputs : {}", pczt.transparent().outputs().len());
    println!("    sapling spends      : {}", pczt.sapling().spends().len());
    println!("    sapling outputs     : {}", pczt.sapling().outputs().len());

    // 4. Combiner role: two parties holding the same base PCZT, merged into one.
    let party_a = make_base_pczt(0);
    let party_b = make_base_pczt(0);
    let combined = Combiner::new(vec![party_a, party_b])
        .combine()
        .expect("two copies of the same base PCZT should combine");
    let combined_bytes = combined.serialize();
    println!(
        "\n[3] combiner OK: merged two parties' PCZTs into {} bytes",
        combined_bytes.len()
    );

    // 5. Emit a canonical test vector.
    fs::create_dir_all("vectors").expect("create vectors dir");
    fs::write("vectors/empty_v5.pczt", &bytes).expect("write vector bytes");
    let manifest = serde_json::json!({
        "name": "empty_v5",
        "description": "Base PCZT from the Creator role: v5 transaction, no spends or outputs",
        "pczt_version": 1,
        "tx_version": g.tx_version(),
        "consensus_branch_id": format!("{:#010x}", g.consensus_branch_id()),
        "coin_type": ZEC_COIN_TYPE,
        "size_bytes": bytes.len(),
        "hex": hex::encode(&bytes),
    });
    fs::write(
        "vectors/empty_v5.json",
        serde_json::to_string_pretty(&manifest).expect("serialize manifest"),
    )
    .expect("write manifest");
    println!("\n[4] wrote vectors/empty_v5.pczt and vectors/empty_v5.json");

    println!("\nDone.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_is_byte_stable() {
        let pczt = make_base_pczt(0);
        let bytes = pczt.serialize();
        let parsed = Pczt::parse(&bytes).expect("a serialized PCZT should parse");
        assert_eq!(
            bytes,
            parsed.serialize(),
            "serialization must be byte-stable across a round-trip"
        );
    }

    #[test]
    fn combiner_merges_identical_base_pczts() {
        let a = make_base_pczt(0);
        let b = make_base_pczt(0);
        let combined = Combiner::new(vec![a, b])
            .combine()
            .expect("two identical base PCZTs should combine");
        let bytes = combined.serialize();
        assert!(Pczt::parse(&bytes).is_ok(), "combined PCZT should parse");
    }

    #[test]
    fn rejects_non_pczt_bytes() {
        assert!(
            Pczt::parse(b"not a pczt at all").is_err(),
            "parser must reject input without the PCZT magic bytes"
        );
    }
}
