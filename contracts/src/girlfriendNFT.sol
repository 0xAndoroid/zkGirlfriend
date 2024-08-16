import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract GirlfriendNFT is ERC721 {
    constructor() ERC721("ZKGirlfriendNFT", "ZKGF") {}

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        
    }
}