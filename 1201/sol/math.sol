pragma solidity ^0.8.0;

contract Math {
    function add(uint256 a, uint256 b) public pure returns (uint256) {
        return a + b;
    }

    function div(uint256 a, uint256 b) public pure returns (uint256) {
        return a / b;
    }

    function mul(uint256 a, uint256 b) public pure returns (uint256) {
         return a * b;
    }

    function sub(uint256 a, uint256 b) public pure returns (uint256) {
             return a - b;
    }
      
}

