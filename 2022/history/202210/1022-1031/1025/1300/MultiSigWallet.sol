pragma solidity ^0.8.13;

contract MultiSigWallet {
   event Deposit(address indexed sender, uint amount, uint balance);
   event SubmitTransaction(
      address indexed owner, 
      uint indexed txIndex, 
      address indexed to, 
      uint value, 
      bytes data
      );
   event ConfirmTransaction(address indexed owner, uint indexed txIndex);
   event RevokeConfirmation(address indexed owner, uint indexed txIndex);
   event ExecuteTransaction(address indexed owner, uint indexed txIndex);

   address[] public owners;
   mapping(address => bool) public isOwner;
   uint public numConfirmationsRequired;

   struct Transaction {
      address to;
      uint value;
      bytes data;
      bool executed;
      uint numConfirmations;
   }

   mapping(uint => mapping(address => bool)) public isConfirmed;

   Transaction[] public transactions;

   modifier onlyOwner() {
      require(isOwner[msg.sender], "Not owner");
      _;
   }
      
   modifier txExists(uint _txIndex) {
      require(_txIndex < transactions.length, "Tx does not exist");
      _;
   }

   modifier notConfirmed(uint _txIndex) {
      require(!isConfirmed[_txIndex][msg.sender], "Tx already confirmed");
      _;
   }
      
   constructor(address[] memory _owners, uint _numConfirmationsRequired) {
      require(_owners.length > 0, "Owners required");
      require(_numConfirmationsRequired > 0 && _numConfirmationsRequired <= _owners.length, "Invalid number of required confirmations");

      for (uint i = 0; i < _owners.length; i++) {
         address owner = _owners[i];
         require(owner != address(0), "Invalid owner");
         require(!isOwner[owner], "Owner not unique");

         isOwner[owner] = true;
         owners.push(owner);
      }

      numConfirmationsRequired = _numConfirmationsRequired;
   }
   
   receive() external payable {
      emit Deposit(msg.sender, msg.value, address(this).balance);
   }

   function submitTransaction(address _to, uint _value, bytes memory _data) public onlyOwner {
      uint txIndex = transactions.length;
      transactions.push(Transaction({
         to: _to,
         value: _value,
         data: _data,
         executed: false,
         numConfirmations: 0
      }));

      emit SubmitTransaction(msg.sender, txIndex, _to, _value, _data);
   }

   function confirmTransaction(uint _txIndex) public onlyOwner txExists(_txIndex) notConfirmed(_txIndex) {
      Transaction storage transaction = transactions[_txIndex];
      transaction.numConfirmations += 1;
      isConfirmed[_txIndex][msg.sender] = true;

      emit ConfirmTransaction(msg.sender, _txIndex);
   }

   function executeTransaction(uint _txIndex) public onlyOwner txExists(_txIndex) {
      Transaction storage transaction = transactions[_txIndex];
      require(!transaction.executed, "Tx already executed");
      require(transaction.numConfirmations >= numConfirmationsRequired, "Tx not confirmed");

      transaction.executed = true;
      (bool success, ) = transaction.to.call{value: transaction.value}(transaction.data);
      require(success, "Tx failed");

      emit ExecuteTransaction(msg.sender, _txIndex);
   }

   function revokeConfirmation(uint _txIndex) public onlyOwner txExists(_txIndex) notConfirmed(_txIndex) {
      Transaction storage transaction = transactions[_txIndex];
      transaction.numConfirmations -= 1;
      isConfirmed[_txIndex][msg.sender] = false;

      emit RevokeConfirmation(msg.sender, _txIndex);
   }

   function getOwners() public view returns (address[] memory) {
      return owners;
   }

   function getTransactionCount(bool _pending, bool _executed) public view returns (uint) {
      uint count = 0;
      for (uint i = 0; i < transactions.length; i++) {
         if (_pending && !transactions[i].executed || _executed && transactions[i].executed) {
            count += 1;
         }
      }
      return count;
   }

   function getTransaction(uint _txIndex) public view txExists(_txIndex)
   returns (
      address to, 
      uint value, 
      bytes memory data, 
      bool executed, 
      uint numConfirmations)
       {
      Transaction storage transaction = transactions[_txIndex];
      return (
         transaction.to, 
         transaction.value, 
         transaction.data, 
         transaction.executed, 
         transaction.numConfirmations);
   }


}