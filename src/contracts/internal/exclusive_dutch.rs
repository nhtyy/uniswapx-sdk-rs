use super::{
    common::OrderInfo,
    dutch::{DutchInput, DutchOutput},
};
use alloy_sol_types::sol;

sol! {
    struct ExclusiveDutchOrder {
        // generic order information
        OrderInfo info;
        // The time at which the DutchOutputs start decaying
        uint256 decayStartTime;
        // The time at which price becomes static
        uint256 decayEndTime;
        // The address who has exclusive rights to the order until decayStartTime
        address exclusiveFiller;
        // The amount in bps that a non-exclusive filler needs to improve the outputs by to be able to fill the order
        uint256 exclusivityOverrideBps;
        // The tokens that the swapper will provide when settling the order
        DutchInput input;
        // The tokens that must be received to satisfy the order
        DutchOutput[] outputs;
    }
}
