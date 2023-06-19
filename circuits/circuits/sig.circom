pragma circom 2.1.5;

// TODO: do we need to check that all registers of r,s fit in n bits?
// Note: The n-bit check is done for pubkey in FlattenPubkey circuit.
template VerifySignature(n, k) {
    signal input r[k];
    signal input s[k];
    signal input msghash[k];
    signal input pubkey[2][k];

    signal input leaf_coins[2];
    signal input receiver;
    signal input signer;

    signal output is_valid;

    // message hash check
    {
        signal msghash_computed <== Poseidon(3)(inputs <== [leaf_coins[0], leaf_coins[1], receiver]);

        var cumulativeM = 0;
        var nPow = 1;
        for (var i=0; i<k; i++) {
            _ <== Num2Bits(n)(in <== msghash[i]);
            cumulativeM += msghash[i]*nPow;
            nPow *= n;
        }

        cumulativeM === msghash_computed;
    }

    signal result <== ECDSAVerifyNoPubkeyCheck(n,k)(
        r <== r,
        s <== s,
        msghash <== msghash,
        pubkey <== pubkey
    );

    // result === 1;

    signal signer_computed <== PubkeyToAddress()(
        pubkeyBits <== FlattenPubkey(n, k)(chunkedPubkey <== pubkey)
    );

    signal is_signer_valid <== IsZero()(in <== signer_computed - signer);

    is_valid <== result * is_signer_valid;
}
