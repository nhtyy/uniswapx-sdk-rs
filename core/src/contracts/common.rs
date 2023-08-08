use alloy_sol_types::sol;

sol! {
    /// @dev generic order information
    ///  should be included as the first field in any concrete order types
    struct OrderInfo {
        // The address of the reactor that this order is targeting
        // Note that this must be included in every order so the swapper
        // signature commits to the specific reactor that they trust to fill their order properly
        address reactor;
        // The address of the user which created the order
        // Note that this must be included so that order hashes are unique by swapper
        address swapper;
        // The nonce of the order, allowing for signature replay protection and cancellation
        uint256 nonce;
        // The timestamp after which this order is no longer valid
        uint256 deadline;
        // Custom validation contract
        address additionalValidationContract;
        // Encoded validation params for additionalValidationContract
        bytes additionalValidationData;
    }

    /// @dev tokens that need to be sent from the swapper in order to satisfy an order
    struct InputToken {
        address token;
        uint256 amount;
        // Needed for dutch decaying inputs
        uint256 maxAmount;
    }

    /// @dev tokens that need to be received by the recipient in order to satisfy an order
    struct OutputToken {
        address token;
        uint256 amount;
        address recipient;
    }

    /// @dev generic concrete order that specifies exact tokens which need to be sent and received
    struct ResolvedOrder {
        OrderInfo info;
        InputToken input;
        OutputToken[] outputs;
        bytes sig;
        bytes32 hash;
    }
}
