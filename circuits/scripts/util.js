import { MerkleTree as FixedMerkleTree } from "fixed-merkle-tree";
import { default as vmtree } from 'vmtree-sdk';

export class MerkleTree extends FixedMerkleTree {
    constructor({ hasher, levels = 20, leaves = [], zero = 0 }) {
        super(levels, leaves, {
            hashFunction: (left, right) => hasher([left, right]),
            zeroElement: zero,
        });
    };
};

export function verifyMerkleProof({pathElements, pathIndices, leaf, root}) {
    pathElements.forEach((element, i) => {
        leaf = !pathIndices[i] ?
            vmtree.poseidon([leaf, element]) : vmtree.poseidon([element, leaf]);
    });
    return leaf == root;
}

export function stringify_nova_json(input) {
    return JSON.stringify(input, (_, v) => typeof v === "number" ? v.toString(): v, 4)
}

// from https://github.com/0xPARC/circom-ecdsa/blob/d87eb7068cb35c951187093abe966275c1839ead/test/bigint.test.ts#L12
export function bigint_to_array(n/*: number*/, k/*: number*/, x/*: bigint*/) {
    let mod = 1n;
    for (var idx = 0; idx < n; idx++) {
        mod = mod * 2n;
    }

    let ret = [];
    var x_temp = x;
    for (var idx = 0; idx < k; idx++) {
        ret.push(x_temp % mod);
        x_temp = x_temp / mod;
    }
    return ret;
}

// bigendian
// from https://github.com/0xPARC/circom-ecdsa/blob/d87eb7068cb35c951187093abe966275c1839ead/test/ecdsa.test.ts#L111
export function bigint_to_Uint8Array(x) {
    var ret = new Uint8Array(32);
    for (var idx = 31; idx >= 0; idx--) {
        ret[idx] = Number(x % 256n);
        x = x / 256n;
    }
    return ret;
}
