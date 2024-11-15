// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/console.sol";

uint256 constant CONTEXT_TRANSIENT_STORAGE_LOCATION = 0;

contract BatchExecutor {
    address public immutable OWNER;

    constructor(address _owner) {
        OWNER = _owner;
    }

    // Struct used to call a contract dynamically and insert the result into the callData of another contract call.
    struct DynamicCall {
        address to;
        bytes data;
        uint64 offset;
        uint64 length;
        uint64 resOffset;
    }

    struct FallbackData {
        bytes[] multicallData;
        bytes returnData;
    }

    // BATCH CALL FUNCTIONS

    function batchCall(bytes[] memory data) external payable {
        require(msg.sender == OWNER);
        _multicall(data);
    }

    /* 
    Storing the previous context is necessary for nested flash loans scenarios
    For example for the following scenario: 

    1. Aave Flash Loan
    │
    └─> Aave Callback
       │
       └─> Uniswap Flash Loan  // Nested operation
           │
           └─> Uniswap Callback
           │
           └─> Back to Aave Callback
           │
           └─> Complete Aave Flash Loan

    The corresponding context flow is:

    Initial Context: Empty
    ├─> Store Aave Context
    │   ├─> Aave Callback
    │   ├─> Store Uniswap Context (save Aave as prevContext)
    │   │   ├─> Uniswap Callback
    │   │   └─> Restore Aave Context
    │   └─> Complete Aave Operation
    └─> Restore Initial Context        

    */
    function singlecall(
        address target,
        uint256 value,
        bytes32 context,
        bytes memory callData,
        DynamicCall[] calldata dynamicCalls
    ) external payable {
        require(msg.sender == address(this));

        for (uint256 i; i < dynamicCalls.length; ++i) {
            DynamicCall calldata dynamicCall = dynamicCalls[i];

            (bool success, bytes memory resData) = dynamicCall.to.staticcall(dynamicCall.data);
            if (!success) _revert(resData);

            uint64 offset = dynamicCall.offset;
            uint64 length = dynamicCall.length;
            uint64 resOffset = dynamicCall.resOffset;

            assembly ("memory-safe") {
                mcopy(add(callData, add(32, offset)), add(resData, add(32, resOffset)), length)
            }
        }

        bytes32 prevContext = _tload(CONTEXT_TRANSIENT_STORAGE_LOCATION);

        _tstore(CONTEXT_TRANSIENT_STORAGE_LOCATION, context);

        (bool success, bytes memory returnData) = target.call{value: value}(callData);
        if (!success) _revert(returnData);

        _tstore(CONTEXT_TRANSIENT_STORAGE_LOCATION, prevContext);
    }

    /*
    The data index is used to retrieve the callback data from the corresponding flash loan. 
    For example, in the case of aave v3 the callback function is `executeOperation(address[],uint256[],uint256[],address,bytes)`.

    Calldata Layout for executeOperation(address[],uint256[],uint256[],address,bytes):

    Position (bytes) | Content
    ----------------|----------------------------------------------------------
    [0:4]           | Function selector
    [4:36]          | Offset to assets[] (param 1)          \
    [36:68]         | Offset to amounts[] (param 2)          |
    [68:100]        | Offset to premiums[] (param 3)         } Parameter Offsets
    [100:132]       | initiator address (param 4)            |
    [132:164]       | Offset to params (param 5) ←-----------/
                    |
    [Offset1]       | assets[] length
    [Offset1+32]    | assets[0]
    [Offset1+64]    | assets[1]
    ...             |
                    |
    [Offset2]       | amounts[] length
    [Offset2+32]    | amounts[0]
    [Offset2+64]    | amounts[1]
    ...             |
                    |
    [Offset3]       | premiums[] length
    [Offset3+32]    | premiums[0]
    [Offset3+64]    | premiums[1]
    ...             |
                    |
    [Offset5]       | params length
    [Offset5+32]    | Our encoded data:
                    | ┌────────────────────────────────┐
                    | │ (multicallData, returnData)    │
                    | │ - multicallData: bytes[]       │
                    | │ - returnData: bytes            │
                    | └────────────────────────────────┘

    The callback data is the last parameter passed to the callback function so data index is 4.
    So 
    - offset = 4 + 32 * 4 = 140. 
    - callback data length is found from the 32 bytes after the offset
    - callback data is the data after the callback data length
    Then: 
    - fallback data length = calldataload(offset)
    - calldatacopy(fallback data, offset, length)
    - mstore(0x40, add(fallback data, length))
    */
    fallback(bytes calldata) external payable returns (bytes memory returnData) {
        bytes32 context = _tload(CONTEXT_TRANSIENT_STORAGE_LOCATION);
        require(msg.sender == address(uint160(uint256(context))));

        uint256 dataIndex = uint256(context >> 160);

        bytes memory fallbackData;
        assembly ("memory-safe") {
            let offset := add(4, calldataload(add(4, mul(32, dataIndex))))
            let length := calldataload(offset)

            fallbackData := mload(0x40)

            calldatacopy(fallbackData, offset, add(32, length))

            mstore(0x40, add(fallbackData, add(32, length)))
        }

        bytes[] memory multicallData;
        (multicallData, returnData) = abi.decode(fallbackData, (bytes[], bytes));

        _multicall(multicallData);
    }

    function _multicall(bytes[] memory data) internal {
        for (uint256 i; i < data.length; ++i) {
            (bool success, bytes memory returnData) = address(this).call(data[i]);
            if (!success) _revert(returnData);
        }
    }

    function _tload(uint256 tloc) internal view returns (bytes32 value) {
        assembly ("memory-safe") {
            value := tload(tloc)
        }
    }

    function _tstore(uint256 tloc, bytes32 value) internal {
        assembly ("memory-safe") {
            tstore(tloc, value)
        }
    }

    receive() external payable {}

    /// @dev Bubbles up the revert reason / custom error encoded in `returnData`.
    /// @dev Assumes `returnData` is the return data of any kind of failing CALL to a contract.
    function _revert(bytes memory returnData) internal pure {
        uint256 length = returnData.length;
        require(length > 0);

        assembly ("memory-safe") {
            revert(add(32, returnData), length)
        }
    }
}
