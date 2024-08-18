// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC721} from "lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import {GirlfriendBank} from "./girlfriendBank.sol";

contract GirlfriendNFT is ERC721 {
    uint256 s_paidUntil;
    uint256 s_subscriptionPeriod;
    address immutable s_bank;
    uint256 immutable score;

    error OnlyBank();

    constructor(uint256 _subscriptionPeriod, address _owner, uint256 _score) ERC721("ZKGirlfriendNFT", "ZKGF") {
        s_paidUntil = block.number + _subscriptionPeriod;
        s_bank = msg.sender;
        _mint(_owner, 1);
        score = _score;
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        if(score > 30){
            return '{"image": "https://i.imgur.com/RA4Ds0m.png","name": "zkGirlfriend NFT"}';
        } else {
            return '{"image": "https://i.imgur.com/P9AbdFH.png","name": "zkGirlfriend NFT"}';
        }
    }

    function paySubscription() external {
        // if(msg.sender != s_bank){
        //     revert OnlyBank();
        // }
        // s_paidUntil += s_subscriptionPeriod;
    }

    function approve(address to, uint256 tokenId) public override {}

    function getApproved(uint256 tokenId) public view override returns (address) {
        return address(0);
    }

    function setApprovalForAll(address operator, bool approved) public override {}

    function transferFrom(address from, address to, uint256 tokenId) public override {}

    function safeTransferFrom(address from, address to, uint256 tokenId, bytes memory data) public override {}
}
