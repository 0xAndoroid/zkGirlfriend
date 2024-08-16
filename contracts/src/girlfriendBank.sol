import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract GirlfriendBank{
    ERC721[] s_nfts;

    uint256 immutable paymentAmount;

    constructor(uint256 _paymentAmount){
        paymentAmount = _paymentAmount;
    } 

    //newGirlfriend

    //subscription

}