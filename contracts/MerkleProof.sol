// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract MerkleProof {
    function verifyData(
        bytes32[] memory proof,
        bytes32 root,
        bytes32 leaf
    ) public pure returns (bool) {
        return processProof(proof, leaf) == root;
    }

    function processProof(bytes32[] memory proof, bytes32 leaf)
        internal
        pure
        returns (bytes32)
    {
        bytes32 computedHash = leaf;
        for (uint256 i = 0; i < proof.length; i++) {
            computedHash = _hashPair(computedHash, proof[i]);
        }
        return computedHash;
    }

    // From: https://mirror.xyz/haruxe.eth/Gg7UG4hctOHyteVeRX7w1Ac9m1gAoCs8uuiWx3WwVz4
    function _hashPair(bytes32 a, bytes32 b) private pure returns (bytes32) {
        return a < b ? keccak256(abi.encodePacked(a, b)) : 
                       keccak256(abi.encodePacked(b, a));
    }
}