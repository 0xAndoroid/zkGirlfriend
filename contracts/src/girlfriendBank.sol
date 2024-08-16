import {GirlfriendNFT} from "./girlfriendNFT.sol";

contract GirlfriendBank{
    address[] s_nfts;
    mapping(address => uint256) s_addressToArrayIndex;
    uint256 s_subscriptionPeriod;

    uint256 immutable paymentAmount;

    error notEnoughPayment();

    modifier checkPaymentAmount() {
        if(msg.value < paymentAmount){
            revert notEnoughPayment();
        }
        _;
    }

    constructor(uint256 _paymentAmount, uint256 _subscriptionPeriod){
        paymentAmount = _paymentAmount;
        s_subscriptionPeriod = _subscriptionPeriod;
    } 

    function newGirlfriend() external payable checkPaymentAmount {
        GirlfriendNFT temp = new GirlfriendNFT(s_subscriptionPeriod, msg.sender);
        s_addressToArrayIndex[msg.sender] = s_nfts.length;
        s_nfts.push(address(temp));
    }

    function paySubscription() external payable checkPaymentAmount {
        GirlfriendNFT(s_nfts[s_addressToArrayIndex[msg.sender]]).paySubscription();
    }

}