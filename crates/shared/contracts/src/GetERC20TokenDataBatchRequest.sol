//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20} from "./IERC20.sol";
/**
 * @dev This contract is not meant to be deployed. Instead, use a static call with the
 *      deployment bytecode as payload.
 */
contract GetERC20TokenDataBatchRequest {
    struct TokenData {
        address tokenAddress;
        string symbol;
        uint8 decimals;
        uint256 totalSupply;
    }

    constructor(address[] memory tokens) {
        TokenData[] memory allTokenData = new TokenData[](tokens.length);

        for (uint256 i = 0; i < tokens.length; ++i) {
            address tokenAddress = tokens[i];

            if (codeSizeIsZero(tokenAddress)) continue;

            TokenData memory tokenData;

            // Set token address
            tokenData.tokenAddress = tokenAddress;

            // Get symbol
            (bool symbolSuccess, bytes memory symbolData) =
                tokenAddress.call{gas: 20000}(abi.encodeWithSignature("symbol()"));

            if (symbolSuccess && symbolData.length > 0) {
                tokenData.symbol = abi.decode(symbolData, (string));
            } else {
                continue;
            }

            // Get decimals
            (bool decimalsSuccess, bytes memory decimalsData) =
                tokenAddress.call{gas: 20000}(abi.encodeWithSignature("decimals()"));

            if (decimalsSuccess) {
                uint256 decimals;
                if (decimalsData.length == 32) {
                    (decimals) = abi.decode(decimalsData, (uint256));
                    if (decimals == 0 || decimals > 255) {
                        continue;
                    } else {
                        tokenData.decimals = uint8(decimals);
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }

            // Get total supply
            (bool totalSupplySuccess, bytes memory totalSupplyData) =
                tokenAddress.call{gas: 20000}(abi.encodeWithSignature("totalSupply()"));

            if (totalSupplySuccess && totalSupplyData.length == 32) {
                tokenData.totalSupply = abi.decode(totalSupplyData, (uint256));
            } else {
                continue;
            }

            allTokenData[i] = tokenData;
        }

        // ensure abi encoding, not needed here but increase reusability for different return types
        // note: abi.encode add a first 32 bytes word with the address of the original data
        bytes memory _abiEncodedData = abi.encode(allTokenData);

        assembly {
            // Return from the start of the data (discarding the original data address)
            // up to the end of the memory used
            let dataStart := add(_abiEncodedData, 0x20)
            return(dataStart, sub(msize(), dataStart))
        }
    }

    function codeSizeIsZero(address target) internal view returns (bool) {
        return target.code.length == 0;
    }
}
