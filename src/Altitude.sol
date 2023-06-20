// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IncrementalTreeData, IncrementalBinaryTree} from "./IncrementalBinaryTree.sol";
import {Ownable2Step} from "../lib/openzeppelin-contracts/contracts/access/Ownable2Step.sol";
import {ECDSA} from "../lib/openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
import {PoseidonT3} from "../lib/poseidon-solidity/contracts/PoseidonT3.sol";
import {PoseidonT5} from "../lib/poseidon-solidity/contracts/PoseidonT5.sol";

struct Force {
    uint hash;
    uint time;
}

struct OwnershipRequest {
    uint coin;
    uint block_number;
    uint time;
    bytes32 responseHash;
}

contract Altitude is Ownable2Step {
    string internal constant ERROR_MINT_EMPTY = "E1";
    string internal constant ERROR_FORCE_NO_COIN = "E2";
    string internal constant ERROR_SLASH_NO_FORCE = "E3";
    string internal constant ERROR_SLASH_NOT_OLD = "E4";
    string internal constant ERROR_DOUBLE_RESPONSE = "E5";
    string internal constant ERROR_NONMATCHING_RESPONSE = "E6";
    using IncrementalBinaryTree for IncrementalTreeData;

    uint internal constant ZERO = 0;
    uint internal constant DEPTH = 20;
    uint internal constant FORCE_WAIT = 1 days;
    uint internal constant REQUEST_WAIT = 1 days;

    IncrementalTreeData public mintTree;
    uint[] public stateHistory;
    uint public stateRoot;

    OwnershipRequest[] public ownershipRequests;

    Force[] public forcedInclusions;

    uint maxCoin = 0;

    constructor(address _owner) {
        _transferOwnership(_owner);
        mintTree.init(DEPTH, ZERO);
    }

    function mint(address receiver) external payable returns (uint root) {
        require(msg.value > 0, ERROR_MINT_EMPTY);
        root = mintTree.insert({
            leaf: PoseidonT3.hash([uint(uint160(receiver)), msg.value])
        });
    }

    function forceInclude(address receiver, uint[2] calldata leafCoins, uint highestCoinToSend, bytes memory signature) external {
        require(highestCoinToSend < maxCoin, ERROR_FORCE_NO_COIN);
        uint hash = PoseidonT5.hash([leafCoins[0], leafCoins[1], highestCoinToSend, uint(uint160(receiver))]);
        require(msg.sender == ECDSA.recover(bytes32(hash), signature));

        forcedInclusions.push(Force({
            hash: hash,
            time: block.timestamp
        }));
    }

    function slashForLateInclusion(uint i) external {
        require(forcedInclusions[i].time != 0, ERROR_SLASH_NO_FORCE);
        require (block.timestamp - forcedInclusions[i].time > FORCE_WAIT, ERROR_SLASH_NOT_OLD);

        // SLASH
    }

    function updateState() external onlyOwner {

    }

    function updateMint() external onlyOwner {

    }

    // any eth sent is part of the slashable stake
    receive() external payable {}

    function requestOwnerShipProof(uint coin, uint block_number) external {
        ownershipRequests.push(OwnershipRequest({
            coin: coin,
            block_number: block_number,
            time: block.timestamp,
            responseHash: 0
        }));
    }

    function slashForLateResponse(uint i) external {
        require(ownershipRequests[i].time != 0, ERROR_SLASH_NO_FORCE);
        require (block.timestamp - ownershipRequests[i].time > REQUEST_WAIT, ERROR_SLASH_NOT_OLD);

        // SLASH
    }

    function respondToOwnershipRequest(uint i, bool[DEPTH] calldata pathIndices, uint[DEPTH] calldata pathElements, uint lower_coin, uint upper_coin, uint owner) external {
        require(ownershipRequests[i].time != 0, ERROR_DOUBLE_RESPONSE);
        ownershipRequests[i].time = 0;
        ownershipRequests[i].responseHash = keccak256(abi.encodePacked(pathIndices, pathElements, lower_coin, upper_coin, owner));
    }

    function slashForInvalidOwnershipProof(uint i, bool[DEPTH] calldata pathIndices, uint[DEPTH] calldata pathElements, uint lower_coin, uint upper_coin, uint owner) external {
        require(keccak256(abi.encodePacked(pathIndices, pathElements, lower_coin, upper_coin, owner)) == ownershipRequests[i].responseHash, ERROR_NONMATCHING_RESPONSE);
        // TODO: require that the proof is valid for that `block_number`th block in stateHistory

        // Slash
    }

    function slashForBrokenPromise() external {
        // TODO: show that the promise was signed by the operator (does transferable ownership make this tricky?)
        // TODO: show that the promise contradicts a response in ownershipRequests

        // Slash
    }


}
