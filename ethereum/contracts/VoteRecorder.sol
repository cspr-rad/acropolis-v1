// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;


contract VoteRecorder {

    event VoteRecorded (
        bytes indexed electionId,
        uint256 voteIndex,
        bytes voteReceipt
    );

    mapping(bytes electionId => uint256 votesIndex) public votesIndices;
    mapping(bytes electionId => mapping(uint256 voteCounter => bytes voteReceipt)) public voteReceipts;

    constructor() {}

    function vote(bytes calldata electionId, bytes calldata voteReceipt) external {
        voteReceipts[electionId][votesIndices[electionId]] = voteReceipt;
        emit VoteRecorded(electionId, votesIndices[electionId], voteReceipt);
        votesIndices[electionId] += 1;
    }

    function getVoteReceipt(bytes calldata electionId, uint256 voteIndex) external view returns (bytes memory) {
        return voteReceipts[electionId][voteIndex];
    }

    function getVotesTallied(bytes calldata electionId) external view returns (uint256) {
        return votesIndices[electionId];
    }
}
