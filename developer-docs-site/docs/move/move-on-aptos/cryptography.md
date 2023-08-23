---
title: "Cryptography"
slug: "cryptography"
---

# Cryptography in Move

The Aptos adapter for Move has introduced many useful cryptographic primitives.
The **key guiding principles** behind these cryptographic extensions are:

1. **Minimize gas costs** for Move developers by implementing key primitives as [Move native functions](./move/book/functions#native-functions) (e.g., see the [`aptos_std::bls12381`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/bls12381.move) module for BLS signatures over BLS12-381 elliptic curves) 
2. When native functions are not available, **empower developers** to implement their own cryptographic primitives on top of abstract cryptographic building blocks such as _finite fields_ and _Abelian groups_ (e.g., via the [`aptos_std::crypto_algebra`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/crypto_algebra.move) module).

An overview of these features and (some of) the applications they enable follows below. 
For a detailed view, see the [cryptography Move modules code](https://github.com/aptos-labs/aptos-core/tree/main/aptos-move/framework/aptos-stdlib/sources/cryptography).

## Cryptographic hash functions in Move

Developers can now use more cryptographic hash functions in Move via the [`aptos_std::aptos_hash`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/hash.move) module:

 1. Keccak256
 2. SHA2-512
 3. SHA3-512
 4. RIPEMD160
 5. Blake2b-256

Some of these functions can be used for interoperability with other chains (e.g., verifying Ethereum Merkle proofs via [`aptos_std::aptos_hash::keccak256`](https://github.com/aptos-labs/aptos-core/blob/137acee4c6dddb1c86398dce25b041d78a3028d3/aptos-move/framework/aptos-stdlib/sources/hash.move#L35)).
Others, have lower gas costs, such as [`aptos_std::aptos_hash::blake2b_256`](https://github.com/aptos-labs/aptos-core/blob/137acee4c6dddb1c86398dce25b041d78a3028d3/aptos-move/framework/aptos-stdlib/sources/hash.move#L69).
In general, a wider variety of hash functions give developers additional freedom in terms of both security and interoperability with other off-chain cryptographic system.

## Digital signature verification in Move

Developers can now use a *type-safe* API for verifying many types of digital signatures in Move:

 1. **Ed25519** signatures in [`aptos_std::ed25519`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/ed25519.move)
 2. “Naive” $t$-out-of-$n$ **Ed25519** **multi**-signatures in [`aptos_std::multi_ed25519`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/multi_ed25519.move) (via simple concatentation of Ed25519 signatures)
 3. **ECDSA** signatures over secp256k1 in [`aptos_std::secp256k1`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/secp256k1.move)
 4. **BLS** signatures, **multi**-signatures, **aggregate** signatures and **threshold** signatures over BLS12-381 in [`aptos_std::bls12831`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/bls12381.move)

The signature modules above can be used to build smart contract-based wallets, secure claiming mechanisms for airdrops, or any digital-signature-based access-control mechanism for dapps.  

## Elliptic curve arithmetic in Move

While the [hash function](#cryptographic-hash-functions-in-move) and [digital signature](#digital-signature-verification-in-move) modules should provide enough functionality for most applications, some applications will require more powerful cryptography.
Normally, developers of such applications would have to wait until their desired crytographic functionality is implemented efficiently as a [Move native function](./move/book/functions#native-functions) in the [Aptos Move framework](/reference/move).
Instead, we expose basic building blocks that developers can use to implement their own cryptographic primitives directly in the Move language and do so **efficiently**.  

Specifically, we currently expose low-level arithmetic operations on two popular elliptic curve groups and their associated finite fields:

 1. Ristretto255, via [`aptos_std::ristretto255`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/ristretto255.move)
 2. BLS12-381, via [`aptos_std::crypto_algebra`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/crypto_algebra.move) 
    and [`aptos_std::bls12381_algebra`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/bls12381_algebra.move)

These modules support low-level operations such as:

 * scalar multiplication of elliptic curve points
 * multi-scalar multiplications (MSMs)
 * pairings
 * scalar addition, multiplication, inversion
 * hashing to a scalar or to a point
 * and many more

Examples of powerful applications that can be built on top include:  

 1. **Validity rollups**; see the [`groth16` zkSNARK verifier example](#groth16-zksnark-verifier)
 2. **Randomness-based games**; see the [`drand` verifier example](#verifying-randomness-from-the-drand-beacon)
 3. **Privacy-preserving applications**; see the [`veiled_coin` example](#veiled-coins) 

### Ristretto255 arithmetic in Move

The [`aptos_std::ristretto255`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/ristretto255.move) module provides support for elliptic curve arithmetic on the popular [Ristretto255 curve](https://ristretto.group/).
One of the main advantages of Ristretto255 is that it is a prime order group (unlike the Edwards 25519 curve), which obviates small-subgroup attacks on higher-level cryptosystems built on top of it.
Furthermore, Ristretto255 serialization is canonical and deserialization only accepts canonical encodings, which obviates malleability issues in higher-level protocols.

This module has proven useful for implementing several cryptographic primitives:

 1. **Zero-knowledge $\Sigma$-protocols**, as demonstrated in the [`veiled_coin` example](#veiled-coins).
 2. **ElGamal** encryption; see [`aptos_std::ristretto255_elgamal`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/ristretto255_elgamal.move)
 3. **Pedersen** commitments; see [`aptos_std::ristretto255_pedersen`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/ristretto255_pedersen.move)
 4. **Bulletproofs** ZK range proofs[^bulletproofs]; see [`aptos_std::ristretto255_bulletproofs`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/ristretto255_bulletproofs.move)

Need ideas for a cryptosystem to build on top of `ristretto255`?
A popular primitive that you could easily build would be the [schnorrkel](https://github.com/w3f/schnorrkel) signature scheme, which is a hardended version of Schnorr signatures over Ristretto255 groups.

### Generic elliptic curve arithmetic in Move

What is better than one curve? More curves!

The [`aptos_std::crypto_algebra`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/crypto_algebra.move) provides elliptic curve arithmetic operations for **any** supported elliptic curve, including pairing-friendly curves.
As a consequence, Move developers can implement a cryptosystem generically over **any** curve that is or will be supported in the future.
Compared to fixing a particular curve in the code (e.g., by implementing against the [Ristretto255 module](#ristretto255-arithmetic-in-move)), this approach provides more flexibility and lowers development time when migrating to a different curve.

Although currently the `crypto_algebra` module only supports arithmetic over BLS12-381 curves (via the marker types declared in [`aptos_std::bls12381_algebra`](https://github.com/aptos-labs/aptos-core/blob/main/aptos-move/framework/aptos-stdlib/sources/cryptography/bls12381_algebra.move)), more curves will be supported into the future (e.g., BN254, Ristretto255, BLS12-377, BW6-761, secp256k1, secp256r1).

As an example, a Move developer can implement the popular Boneh-Lynn-Shacham (BLS) signature scheme generically over **any** curve by using [type arguments](../../../move/book/functions#type-parameters) for the curve type in their implementation:

```move
use std::option;
use aptos_std::crypto_algebra::{eq, pairing, one, deserialize, hash_to};

/// Example of a BLS signature verification function that works over any pairing-friendly
/// group triple `G1`, `G2`, `Gt` where signatures are in `G1` and PKs in `G2. Points are
/// serialized using the format in `FormatG1` and `FormatG2` and the hashing method is 
/// `HashMethod`.
/// 
/// WARNING: This example is type-unsafe and probably not a great fit for production code.
public fun bls_verify_sig<G1, G2, Gt, FormatG1, FormatG2, HashMethod>(
    dst:        vector<u8>,
    signature:  vector<u8>,
    message:    vector<u8>,
    public_key: vector<u8>): bool
{
    let pk   = option::extract(&mut deserialize<G2, FormatG2>(&public_key));
    let sig  = option::extract(&mut deserialize<G1, FormatG1>(&signature));
    let hash = hash_to<G1, HashMethod>(&dst, &message);
    
    // Checks if $e(H(m), pk) = e(sig, g_2)$, where $g_2$ generates $\mathbb{G}_2$
    return eq(
        &pairing<G1, G2, Gt>(&msg_hash, &pk), 
        &pairing<G1, G2, Gt>(&sig, &one<G2>())
    );
}
```

Using the `bls_verify_sig` _generic_ function from above, developers can verify BLS signatures over **any** of the supported (pairing-friendly) curves.
For example, one can verify BLS signatures over BLS12-381 curves by calling the function above with the BLS12-381 marker types as its type arguments:

```move
use aptos_std::bls12381_algebra::{
    G1, G2, Gt, FormatG1Compr, FormatG2Compr, HashG1XmdSha256SswuRo
};

// Aborts with code 1 if the BLS signature over the BLS12-381 curve fails to verify. 
assert(
    bls_verify_sig<G1, G2, Gt, FormatG1Compr, FormatG2Compr, HashG1XmdSha256SswuRo>(
        dst, signature, message, public_key
    ),
    1
);
```

For more use cases of the `crypto_algebra` module, check out some Move examples:

1. [Verifying Groth16 zkSNARK proofs](#groth16-zksnark-verifier) over **any** curve 
2. [Verifying randomness from the `drand` beacon](#verifying-randomness-from-the-drand-beacon)

## Building powerful cryptographic applications in Move

### Veiled coins

The [`veiled_coin` example](https://github.com/aptos-labs/aptos-core/tree/main/aptos-move/move-examples/veiled_coin/sources) demonstrates how to use [the Ristretto255 modules from above](#ristretto255-arithmetic-in-move) to add a reasonable layer of confidentiality to coin balances and transactions.

Specifically, users can **veil** their balance, keeping it hidden from everyone, including validators.
Furthermore, a user can send a **veiled transaction** that hides the transaction amount from everybody, including validators.
An important caveat is that veiled transactions do NOT hide the identities of the sender or the recipient.

**WARNING:** This module is educational. It is **NOT** production-ready. Using it could likely lead to loss of funds.

### Groth16 zkSNARK verifier

The [`groth16` example](https://github.com/aptos-labs/aptos-core/tree/main/aptos-move/move-examples/groth16_example/sources) demonstrates how to verify Groth16 zkSNARK proofs[^groth16], which are the shortest and fastest-to-verify zero-knowledge proofs for arbitrary computations.
Importantly, as explained [above](#generic-elliptic-curve-arithmetic-in-move), this implementation is *generic* over **any** curve, making it very easy for Move developers to use it with their favorite (supported) curves.

**WARNING:** This code has not been audited. If using it in a production system, proceed at your own risk.

### Verifying randomness from the `drand` beacon

The [`drand` example](https://github.com/aptos-labs/aptos-core/tree/main/aptos-move/move-examples/drand/sources) shows how to verify public randomness from the [drand](https://drand.love) randomness beacon.
This randomness can be used in games or any other chance-based smart contract.
We give a simple example of a lottery implemented on top of `drand` randomness in [`lottery.move`](https://github.com/aptos-labs/aptos-core/tree/main/aptos-move/move-examples/drand/sources/lottery.move).

**WARNING:** This code has not been audited. If using it in a production system, proceed at your own risk.

Another application that can be built on top of `drand` is time-lock encryption[^tlock], which allows users to encrypt information such that it can only be decrypted in a future block.
We do not currently have an implementation but the reader is encouraged to write one!

[^bulletproofs]: _bulletproofs:_ **Bulletproofs: Short Proofs for Confidential Transactions and More**; by B. Bünz and J. Bootle and D. Boneh and A. Poelstra and P. Wuille and G. Maxwell; in 2018 IEEE Symposium on Security and Privacy
[^groth16]: _groth16:_ **On the Size of Pairing-Based Non-interactive Arguments**; by Groth, Jens; in EUROCRYPT 2016
[^tlock]: _tlock:_ tlock: Practical Timelock Encryption from Threshold BLS; by Nicolas Gailly and Kelsey Melissaris and Yolan Romailler; in Cryptology ePrint Archive, Paper 2023/189; [https://eprint.iacr.org/2023/189](https://eprint.iacr.org/2023/189)