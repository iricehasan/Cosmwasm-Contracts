# Cosmwasm Simple Option

This smart contract is written by following the tutorial: https://docs.cosmwasm.com/tutorials/simple-option/intro/ and it is a contract for simple option. It allows the holder the right to buy or sell an underlying asset at a strike price or before the expiration date. The application logic:

- The owner can post counter_offer on unexpired option to execute and get the collateral.
- The owner can release the collateral if it is expired.
- The ownership can be transferred to another address.

