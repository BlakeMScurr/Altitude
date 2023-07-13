import { expect }  from 'chai';
import { default as vmtree } from 'vmtree-sdk';
import { MerkleTree, stringify_nova_json, verifyMerkleProof } from './util.js';
import { ethers } from "ethers";

// This file generates and validates the input data for the filter circuit
// 1) Generate mock onchain data:
//      a/ We construct a state history
//      b/ We take a set of claims and keccak them together into a hash chain, remembering all intermediate values
// 2) Build inputs
//      a/ Initialise a filtered tree
//      b/ We unwind the hash chain:
//          i. Add to the filtered tree if claim exists in history
//          ii. Set inputs

// 1) a/ Construct state history

// State history is a helper class for constructing a mock history of the states of an ArcPay Validium,
// and helper methods to make claims and construct the inputs for distribution proofs 
class StateHistory {
    constructor(address_list, state_history) {
        this.historyTree = new MerkleTree({ hasher: vmtree.poseidon, levels: 3, zero: 0 });
        this.state_history = state_history;
        this.address_list = address_list;
        this.state_trees = state_history.map((state, i) => {
            let stateTree = new MerkleTree({ hasher: vmtree.poseidon, levels: 3, zero: 0 });
            state.forEach((entry, j) => {
                stateTree.update(j, vmtree.poseidon([address_list[entry.address_index], entry.first_coin, entry.last_coin]));
            })
            this.historyTree.update(i, stateTree.root);
            return stateTree;
        })
    }

    claim(block_number, state_index) {
        let entry = this.state_history[block_number][state_index];
        let { pathElements, pathIndices } = this.state_trees[block_number].path(state_index);

        return {
            address: this.address_list[entry.address_index],
            first_coin: entry.first_coin,
            last_coin: entry.last_coin,
            block_number: block_number, 
            state_pathElements: pathElements, 
            state_pathIndices: pathIndices, 
            state_index: state_index, // TODO: only have one of state_index and state_pathIndices 
        }
    }

    claim_is_valid(claim) {
        let proof = {
            leaf: vmtree.poseidon([claim.address, claim.first_coin, claim.last_coin]),
            root: this.state_trees[claim.block_number].root,
            pathElements: claim.state_pathElements,
            pathIndices: claim.state_pathIndices
        }
        return verifyMerkleProof(proof)
    }
}

const history = new StateHistory(
    [
        1234,
        5678,
        9101,
    ],
    [
        [
            {
                address_index: 0,
                first_coin: 0,
                last_coin: 10,
            },
            {
                address_index: 1,
                first_coin: 11,
                last_coin: 15,
            },
            {
                address_index: 2,
                first_coin: 16,
                last_coin: 20,
            },
        ],
        [
            // all coins have moved to the first user
            {
                address_index: 0,
                first_coin: 0,
                last_coin: 10,
            },
            {
                address_index: 0,
                first_coin: 11,
                last_coin: 15,
            },
            {
                address_index: 0,
                first_coin: 16,
                last_coin: 20,
            },
        ]
    ]
)

const claims = [
    // invalid (arbitrary nonsense data)
    {
        address: history.address_list[2],
        first_coin: 1234567,
        last_coin: 12345677,
        block_number: 1, // Note, this is restricted to be less than 2 ** (history_depth = 3) by the claim contract
        state_pathElements: history.state_trees[0].path(1).pathElements, 
        state_pathIndices: history.state_trees[0].path(1).pathIndices,
        state_index: 1,
    },
    history.claim(0, 0), // Valid and old, but not outdated
    history.claim(0, 1), // Valid, old, and outdated
    history.claim(1,1), // Valid, new, and supersedes previous claim
    history.claim(0, 2), // Valid and new
]

expect(claims.map((claim) => history.claim_is_valid(claim))).to.deep.equal([false, true, true, true, true])

// 1) b/ Construct claim chain
function build_hash_chain() {
    
}


console.log(ethers.keccak256("0x12"))
