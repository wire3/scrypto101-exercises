# Chapter 4 - Exercise 1
In this exercise, you are going to write a blueprint that defines the logic of a component allowing people to stake their XRD into. Before being able to stake and withdraw their XRD, members will have to register to get a member NFT badge. This badge will be used to store the amount of XRD the member currently have staked.

Follow the steps in [the code file](code/src/lib.rs) to get started and don't hesitate to validate your answer by looking at the [solution file](solution/src/lib.rs).

## How to test the solution with resim
1. Reset your ledger with: `resim reset`
1. Create a new default account: `resim new-account`
1. Publish your code: `resim publish ./code`
1. Instantiate a new component: `resim call-function [package_address] Exercise1 instantiate_exercise` -> Save the returned component address and second resource address. This last address represents the member NFT.
1. Call the `become_member` method: `resim call-method [component_address] become_member`
1. Stake 25 XRD by calling `resim call-method [component_address] stake_xrd 25,resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety 1,[member_badge_address]`
1. Withdraw the staked XRD by calling: `resim call-method [component_address] withdraw 1,[member_badge_address]`