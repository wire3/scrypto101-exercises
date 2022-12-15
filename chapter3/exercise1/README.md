# Chapter 3 - Exercise 1
In this exercise, you are going to create a component that instantiate two resources with different flags and that provides methods to interact with those tokens.

Follow the steps in [the code file](code/src/lib.rs) and don't hesitate to validate your answer by looking at the [solution file](solution/src/lib.rs).

## How to run your code with resim
1. Reset your ledger with: `resim reset`
1. Create a new default account: `resim new-account` -> Store the address of the account somewhere.
1. Create an owner badge: `resim new-simple-badge`
1. Publish your code: `resim publish ./code`
1. Instantiate a new component: `resim call-function [package_address] Exercise1 instantiate_exercise` -> Save the returned component address and first resource address. This first resource address represents the banana resource.
1. Call the `mint_apple` method: `resim call-method [component_address] mint_apple`
1. Take a look at the resources in your account: `resim show [account_address]`
1. Call the `get_banana` method: `resim call-method [component_address] get_banana`
1. Burn the banana with: `resim call-method [component_address] burn_banana 1,[banana_resource_address]`