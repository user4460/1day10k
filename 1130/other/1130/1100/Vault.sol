pragma solidity ^0.8.13;

contract Vault {
   IERC20 public immutable token;

   uint public totalSupply;
   mapping(address => uint) public balanceOf;

   constructor(address _token) {
      token = IERC20(_token);

   }

   function _mint(address _to, uint _shares) private {
      totalSupply += _amount;
      balanceOf[_to] += _amount;
   }

   function _burn(address _from, uint _shares) private {
      totalSupply -= _amount;
      balanceOf[_from] -= _amount;
   }

   function deposit(uint _amount) public {
      uint shares;
      if (totalSupply == 0) {
         shares = _amount;
      } else {
         shares = (_amount * totalSupply) / token.balanceOf(address(this));
      }

      _mint(msg.sender, shares);
      token.transferFrom(msg.sender, address(this), _amount);
      _mint(msg.sender, _amount);
   }

   function withdraw(uint _shares) public {
      uint amount = (_shares * token.balanceOf(address(this)) * _shares) / totalSupply;
      _burn(msg.sender, _shares);
      token.transfer(msg.sender, amount);
   }
}

interface IERC20 {
   function totalSupply() external view returns (uint);

   function balanceOf(address account) external view returns (uint);

   function transfer(address recipient, uint amount) external returns (bool);

   function allowance(address owner, address spender) external view returns (uint);

   function approve(address spender, uint amount) external returns (bool);

   function transferFrom(address sender, address recipient, uint amount) external returns (bool);

   event Transfer(address indexed from, address indexed to, uint value);

   event Approval(address indexed owner, address indexed spender, uint value);


}