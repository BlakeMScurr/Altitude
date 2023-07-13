import { expect }  from 'chai';
import { default as vmtree } from 'vmtree-sdk';
import { MerkleTree, stringify_nova_json, verifyMerkleProof } from './util.js';

const stateTree = new MerkleTree({ hasher: vmtree.poseidon, levels: 3, zero: 0 });
const mintTree = new MerkleTree({ hasher: vmtree.poseidon, levels: 3, zero: 0 });

// recipient secret keys = ['0xb4b0bf302506d14eba9970593921a0bd219a10ebf66a0367851a278f9a8c3d08', '0xb81676dc516f1e4dcec657669e30d31e4454e67aa98574eca670b4509878290c']
const requests = [{recipient: 802933809494131860455082925493303288586736066170n, leaf_coins:[0, 10]}, {recipient: 253486562210967126009990789802080859110172940592n, leaf_coins: [11, 15]}]
requests.forEach((v) => v.leaf = vmtree.poseidon([v.recipient, v.leaf_coins[0], v.leaf_coins[1]]))

// fill up mint tree
requests.forEach((request, i) => {
    mintTree.update(i, request.leaf)
})
let step_in = [mintTree.root.toString(), stateTree.root.toString()];

// update state tree while emptying mint tree and generating circuit inputs
let inputs = requests.map((request, i) => {
    stateTree.update(i, 0); // initialises the leaf
    
    const {pathIndices: pi, pathElements: pe} = mintTree.path(i);
    expect(verifyMerkleProof({
        leaf: request.leaf,
        root: mintTree.root,
        pathElements: pe,
        pathIndices: pi
    })).to.be.true;
    
    const { pathIndices: si, pathElements: se } = stateTree.path(i);
    const input = vmtree.utils.stringifyBigInts({
        sender: 0,
        recipient: request.recipient,
        leaf_coins: request.leaf_coins,
        mintPathElements: pe,
        mintPathIndices: pi,
        pathElements: se,
        pathIndices: si
    });
    
    // prepare trees for next mint
    mintTree.update(i,0);
    stateTree.update(i, request.leaf);
    return input;
})

console.log(stringify_nova_json({
    step_in: step_in,
    private_inputs: inputs
}))
