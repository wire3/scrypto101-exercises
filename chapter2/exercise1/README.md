# Chapter 2 - Exercise 1
In this exercise, you will get familiar with different Rust types, how to define structs that are compatible with the Radix ledger and how to use various loging instructions to output text when calling a method.

Follow the steps in [the code file](code/src/lib.rs) and don't hesitate to validate your answer by looking at the [solution file](solution/src/lib.rs).

## How to run your code with resim
1. Reset your ledger with: `resim reset`
1. Create a new default account: `resim new-account`
1. Create a owner badge: `resim new-simple-badge`
1. Publish your code: `resim publish ./code --owner-badge [owner_badge_NFAddress]`
1. Instantiate a new component: `resim call-function [package_address] Exercise1 instantiate_exercise [your_name]`
1. Call the `log_data` method: `resim call-method [component_address] log_data`. You should see some information in the returned logs.