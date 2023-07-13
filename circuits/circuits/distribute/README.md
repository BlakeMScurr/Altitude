# Circuits

As well as being used to enforce the state transition function, ZKPs are used to handle distribution after shutdown.
The distribution circuits take a set of claims over coins and output the rightful final owners given those claims.

## Algorithm

Distribution involves 3 parts:
    - Filter out any claim that is invalid (i.e., did not occur in the state history)
    - Resolve any conflicts between claims
    - Prove that there are no conflicting claims in the final output

For conceptual simplicity and good prover time, we will implement each part as a separate Nova circuit.

### Filtering

Claims are statments like `X owns coins [a,b] in block Y` along with a purported Merkle proof.
The set of claims are represented as an accumulator, in particular a keccak hash chain.
The state history is represented as a Merkle root, which is known onchain, and passed as a public input to the circuit. It must be passed unchanged through each iteration of the circuit.
There is also an index public input, which is required to start at `0` and end at `claim_count`, which is also known onchain.
There is a `filtered_count` input that starts at `0`.

The circuit processes one claim per iteration/fold. The claim accumulator starts at 0, one claim is added per iteration, and it must end with the right onchain value.
If the claim exists in the state history, the `filtered_count` variable is incremented and the claim is added to a filtered poseidon merkle tree at position `filtered_count`. Otherwise, `filtered_count` stays the same and nothing is changed.

If all the checks pass, then this circuit outputs a `filtered_tree` of size `filtered_count`, where every claim in the tree is valid.

### Challenge Resolution

The challenge resolution circuit is also in Nova and runs iteratively. The initial input is the `filtered_tree` which is a poseidon Merkle tree, and each iteration takes two contradictory claims in the tree, resolves the contradiction, and updates the tree.

The first claim is called the winner and the second is the loser. The winner's block number must be greater than the loser's. The winner's leaf remains unedited. If the winner's coins are a superset of the loser's (i.e., $[0,5]$ beats $[2,3]$), the loser's leaf is deleted, and the `resolved_count` is decremented. If the loser's coins are split (i.e., $[2,3]$ beats $[0,5]$), the loser's leaf is deleted, but two new leaves are added and the `resolved_count` is incremented. If the loser's coins are truncated (i.e., $[1,5]$ vs $[1,3]$), the loser's leaf is deleted, a new leaf is added and the `resolved_count` remains the same.

Now we have a `resolved_tree` that contains all valid winning claims, but may contain additional claims.

### Proving Disjointness

The disjointness circuit takes the `resolved_tree` as input, and a new `final_tree` as advice. Its goal is to show that the `final_tree` is sorted by coin order, fully disjoint, and only contains elements from the `resolved_tree`. This proves that the final tree only contains the true winners, and, importantly, it can be deterministically calculated with onchain data, so everyone can calculate the merkle proofs they need. The `final_tree` should use keccak for gas efficiency.

The circuit iterates over every element in the `final_tree` and makes sure that its `end_coin` is lower than the `start_coin` of the next claim (unless it's the final claim, in which case there is no next claim). It also makes sure that the element exists in the `resolved_tree`.

Since the `start_coin[i] < end_coin[i]` for every valid claim (if the protocol operated correctly while running!!), and `end_coin[i] < start_coin[i+1]` from our ordering check, we know that our claims are strictly ordered. This means that every claim is different. Since we have proved that the `final_tree` contains at least `resolve_count` distinct elements that also exist in the `resolved_tree`, we know that `final` is a superset of `resolved`. To prove that the sets are strictly equal, we have to prove that all other elements are zero.

We prove that the remaining elements are zero by showing that the `resolved_count`th element is 0, and that the `resolved_count/2`th element is `h(0, 0)` etc up the tree. This doesn't necessarily fit nicely into an iteration of the disjointness circuit, but could be done in its own proof or even onchain.

## Properties

It's critical that the final result contains exactly the winning valid claims in order. The result must be fully deterministic, so that anyone can generate the tree and appropriate proofs. The proof must also be unhaltable in the sense that adversarial claims won't make it impossible to generate the proof.