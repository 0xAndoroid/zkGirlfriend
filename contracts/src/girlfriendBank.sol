// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {GirlfriendNFT} from "./girlfriendNFT.sol";
import {ISP1Verifier} from "../lib/sp1-contracts/contracts/src/ISP1Verifier.sol";

contract GirlfriendBank{
    address[] s_nfts;
    mapping(address => uint256) s_addressToArrayIndex;
    uint256 s_subscriptionPeriod;

    uint256 immutable paymentAmount;
    address immutable SP1_VERIFIER;
    bytes32 immutable VERIFICATION_KEY;

    error notEnoughPayment();

    modifier checkPaymentAmount() {
        if(msg.value < paymentAmount){
            revert notEnoughPayment();
        }
        _;
    }

    constructor(uint256 _paymentAmount, uint256 _subscriptionPeriod, address _sp1verifier, bytes32 _verificationKey){
        paymentAmount = _paymentAmount;
        s_subscriptionPeriod = _subscriptionPeriod;
        SP1_VERIFIER = _sp1verifier;
        VERIFICATION_KEY = _verificationKey;
    } 

    function newGirlfriend(bytes calldata _publicValues, bytes calldata _proofBytes) external payable checkPaymentAmount {
        _verifyProof(_publicValues, _proofBytes);
        GirlfriendNFT temp = new GirlfriendNFT(s_subscriptionPeriod, msg.sender);
        s_addressToArrayIndex[msg.sender] = s_nfts.length;
        s_nfts.push(address(temp));
    }

    function paySubscription() external payable checkPaymentAmount {
        GirlfriendNFT(s_nfts[s_addressToArrayIndex[msg.sender]]).paySubscription();
    }

    function _verifyProof(bytes calldata _publicValues, bytes calldata _proofBytes) internal view {
        ISP1Verifier(SP1_VERIFIER).verifyProof(VERIFICATION_KEY, _publicValues, _proofBytes);
    }
}