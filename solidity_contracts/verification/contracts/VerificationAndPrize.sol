// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;
import "./IZKVerifyAttestation.sol";
import "./IAddressLogger.sol";

contract VerificationAndPrize {
    // zkVerify contract on Arbitrum
    address public constant ZKVERIFY =
        0x82941a739E74eBFaC72D0d0f8E81B1Dac2f586D5;
    address public address_logger;

    event ProofVerified(
        bytes32 indexed leaf,
        uint256 indexed attestationId,
        uint256 index
    );
    event PayoutsProcessed(uint256 winnersCount);
    event VerificationFailed(
        bytes32 indexed leaf,
        uint256 indexed attestationId,
        string reason
    );

    constructor(address _address_logger) {
        address_logger = _address_logger;
    }

    function verifyWinnersAndProcess(
        bytes32 _leaf,
        uint256 _attestationId,
        bytes32[] calldata _merklePath,
        uint256 _leafCount,
        uint256 _index,
        bool[] memory winners
    ) public {
        bool verified = IZkVerifyAttestation(ZKVERIFY).verifyProofAttestation(
            _attestationId,
            _leaf,
            _merklePath,
            _leafCount,
            _index
        );

        if (!verified) {
            emit VerificationFailed(
                _leaf,
                _attestationId,
                "Proof verification failed"
            );
            revert("Invalid proof");
        }

        emit ProofVerified(_leaf, _attestationId, _index);

        IAddressLogger(address_logger).processPayouts(winners);
        emit PayoutsProcessed(winners.length);
    }
}
