# Chapter 5 - Exercise 1
You are tasked with writing a transaction manifest that interacts with the blueprint you wrote in the example 1 of Chapter 4. You are going to write a manifest that calls the `become_member` method and `stake_xrd` method all in a single transaction. Start by publishing the blueprint on the local ledger and instantiating a component with:

1. `resim reset`
2. `resim new-account` -> Save the account address somewhere
3. `resim new-simple-badge`
4. `resim publish ./code --owner-badge [owner_badge_NFAddress]`
5. `resim call-function [package_address] Exercise1 instantiate_exercise` -> Save the component address and the member badge address somewhere

You are now ready to fill the `your_manifest.rtm` file. Open this file to see the steps to follow.

You can test your manifest file with `resim run your_manifest.rtm`.

Feel free to look at the `solution.rtm` file for the solution.
