// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {GirlfriendBank} from "../src/girlfriendBank.sol";
import {SP1VerifierGateway} from "@sp1-contracts/src/SP1VerifierGateway.sol";

contract ProofTest is Test {
    GirlfriendBank bank;
    address verifier;

    function setUp() public {
        verifier = address(new SP1VerifierGateway(address(1)));

        bank = new GirlfriendBank(
            0,
            10,
            verifier,
            0x002e3cd0431437b0747be8442b444d9bad9303730a343819cd927d4613d3d213
        );
    }

    function proofTest() public {
        bank.newGirlfriend(
            abi.encode(
                bytes32(
                    0x0000000000000000000000000000000000000000000000000000000000000026
                )
            ),
            abi.encode(
                hex"c430ff7f21cbb1d6b74384091cfc137df263a47ef7a974a077d569439ee66576c05711841a50798e906e4e908bc84898380b5c2b32de3415df274eb04e3df3ab75c0c17b0a1b9d1d127c859c42f8192b81cfb86bb3096c406301d941b77657a41c3e0e592aebbda1bcdb027046a182534001267f6f2573ba7710f3cd8c6af4f9a74809a9272a4e0fb2ec2c4f93eeeb9e20f077db1e56a92e0c857a816107b4005c74e89d22163168f3e3e793a8ead0070165de6fa9fa3495810fb40936682b26e35dbaf6008653d31c286abe12be89dc5cfc248dbb2ecdb4d1966d5c85b6fdbd2b96908929203f51059bafc52629abca22e9e6ac4ba297d739a97d89f4b0d411bc5c9f740f6a449517bf3a218d1bbdd4c255f18dcaadc1a30caa54bbb34fb669ed49a02f09d2ea2440d31de496bb3b791584c09c633315e19d0db982cfd31d7ef570b46923afb712dcec029261fcc0703732d0fa3867c161e91aeacf7b9be9a17a75553d03bd1f9d7eb6dd12296f6383b519e5f06d4e30893feb9099c1bc7ab427f8a5b6161602583dd1c55f0a7572dba3d5b421251e28e3137b312b20e5946befcdfbb90780e30ce9d1cff39154dbe0ebd545c68e705b9ac15b033123b64ab9f916d61b09c5800234d0299027e5a04ce871e37b54a93fbf09c6caad9e775af7d2e58c071fdec6f502a247bad78081f7a3ad4f6d96f2dfa29b3afcedf42cd4115d6163db24d0cb0e09ab4aa83c8dd8d9d938cdda7beb0d0d15fdddf79b7799c7e6b8ff330f7face0d64dde35683c4699bbe84c2aa094078eca0ffb20bc33027db848f3d4029cac07f83940db067e5d64bbd2b32634ee1d650bb5348f761db9dfd2e96fc928af3b6bda66137621647276a5bc3506b25b311aa0a36caa3850a384f915e1221279ae675a528ba1186ca5a457bfa6a8a9ff3be2fcc9f0e35e47f7f956f767ff27090a241e956214f841d363c249c9fcf73fc22b0c5902e7efa2a1dd1e568e9329b0604dbca4a2f2d2063bcc4be907acfab52de2641db2ee800cba89d7a7b08e1f5aea978e5dc3daeee0aa6251dcfd49814295557e3520f2e4b99af8253edd482df94c0c4ff78c88b9b9689f18a0d217777ffbaa149df037f9132dee96e346970911b2e3d25f136885767ab777e49f28edd6eb6b944f110354e32f07a691ae6c009d3aaacad2293846f921073da165f930086a1202a668232fbade61c56e5a1d"
            ),
            address(this)
        );
    }
}

