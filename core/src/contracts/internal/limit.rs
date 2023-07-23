use super::super::common::{InputToken, OrderInfo, OutputToken};
use alloy_sol_types::sol;

sol! {
    /// @dev External struct used to specify simple limit orders
    struct LimitOrder {
        // generic order information
        OrderInfo info;
        // The tokens that the swapper will provide when settling the order
        InputToken input;
        // The tokens that must be received to satisfy the order
        OutputToken[] outputs;
    }
}
