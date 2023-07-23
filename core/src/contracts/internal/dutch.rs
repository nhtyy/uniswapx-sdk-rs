use super::super::common::OrderInfo;
use alloy_sol_types::sol;

sol! {
    /// @dev An amount of output tokens that decreases linearly over time
    struct DutchOutput {
        // The ERC20 token address (or native ETH address)
        address token;
        // The amount of tokens at the start of the time period
        uint256 startAmount;
        // The amount of tokens at the end of the time period
        uint256 endAmount;
        // The address who must receive the tokens to satisfy the order
        address recipient;
    }

    /// @dev An amount of input tokens that increases linearly over time
    struct DutchInput {
        // The ERC20 token address
        address token;
        // The amount of tokens at the start of the time period
        uint256 startAmount;
        // The amount of tokens at the end of the time period
        uint256 endAmount;
    }

    struct DutchOrder {
        // generic order information
        OrderInfo info;
        // The time at which the DutchOutputs start decaying
        uint256 decayStartTime;
        // The time at which price becomes static
        uint256 decayEndTime;
        // The tokens that the swapper will provide when settling the order
        DutchInput input;
        // The tokens that must be received to satisfy the order
        DutchOutput[] outputs;
    }
}
