pragma circom 2.1.5;

include "./node_modules/circomlib/circuits/poseidon.circom"; // TODO: consider Poseidon2

// if s == 0 returns [in[0], in[1]]
// if s == 1 returns [in[1], in[0]]
template DualMux() {
    signal input in[2];
    signal input s;
    signal output out[2];

    s * (1 - s) === 0;
    out[0] <== (in[1] - in[0])*s + in[0];
    out[1] <== (in[0] - in[1])*s + in[1];
}

// Verifies that merkle proof is correct for given merkle root and a leaf
// pathIndices input is an array of 0/1 selectors telling whether given pathElement is on the left or right side of merkle path
// Note: this outputs a Merkle root, but doesn't check it against anything - that must be done by the caller
template CheckMerkleProof(levels) {
    signal input leaf;
    signal input pathElements[levels];
    signal input pathIndices[levels];
    signal output root;

    component selectors[levels];
    component hashers[levels];

    for (var i = 0; i < levels; i++) {
        selectors[i] = DualMux();
        selectors[i].in[0] <== i == 0 ? leaf : hashers[i - 1].out;
        selectors[i].in[1] <== pathElements[i];
        selectors[i].s <== pathIndices[i];

        hashers[i] = Poseidon(2);
        hashers[i].inputs[0] <== selectors[i].out[0];
        hashers[i].inputs[1] <== selectors[i].out[1];
    }

    root <== hashers[levels - 1].out;
}

template CheckMerkleProofStrict(levels) {
    signal input leaf;
    signal input pathElements[levels];
    signal input pathIndices[levels];
    signal input root;

    signal calculated_root <== CheckMerkleProof(levels)(
        leaf,
        pathIndices,
        pathElements
    );

    root === calculated_root;
}

template UpdateLeaf(levels) {
    signal input old_leaf;
    signal input new_leaf;
    signal input pathElements[levels];
    signal input pathIndices[levels];
    signal input old_root;
    signal output new_root;

    component mpcs[2];
    for (var i = 0; i < 2; i++) {
        mpcs[i] = CheckMerkleProof(levels);
        for (var j = 0; j < levels; j++) {
            mpcs[i].pathElements[j] <== pathElements[j];
            mpcs[i].pathIndices[j] <== pathIndices[j];
        }
    }
    // Makes sure the old leaf was in the old root at the specified position
    mpcs[0].leaf <== old_leaf;
    mpcs[0].root === old_root;

    // Makes sure the new leaf is in the new root at the specified position with no other elements in the tree changed
    mpcs[1].leaf <== new_leaf;
    new_root <== mpcs[1].root;
}
