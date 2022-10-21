use scrypto::prelude::*;

// Step 1: Define the Struct that will be used to store the
// members non fungible metadata. Name the struct `MemberData` and add a Decimal field
// named `amount_staked` that is **mutable**.



blueprint! {
    struct Exercise1 {
        // Step 4: Define the variables that the instantiated component
        // will have access to. We need a vault to store the staked XRD,
        // a Vault to store the member_manager badge and a ResourceAddress to store the address
        // of the member badges


    }

    impl Exercise1 {
        pub fn instantiate_exercise() -> ComponentAddress {
            // Step 2: Create a new fungible resource with a divisibility of 0 and an initial supply of 1.
            // This will be the badge that is allowed to mint member badges. Store the token in a `member_manager_badge` variable.


            // Step 3: Create a non-fungible resource that is `mintable` and `updateable_non_fungible_data`
            // by someone showing ownership of the `member_manager_badge`. Initialize the resource with no initial supply and store the result
            // in a `member_badge` variable.


            Self {
                // Step 5: Assign a value for the three variables we defined in the struct.


            }
            .instantiate()
            .globalize()
        }

        // Step 6: Create a `become_member` method that returns a Bucket.
        // Inside, instantiate a new MemberData struct with a value of 0 for the amount_staked.
        // Then put a proof of the `manager_badge` on the AuthZone and mint a new member non fungible resource 
        // with random ID and return it.


        // Step 7: Complete the `stake_xrd` method. Start by adding a Bucket parameter named `xrd`
        // and a Proof parameter named `member_proof`.
        pub fn stake_xrd(&mut self) {
            
            // Step 8: Validate the passed proof to make sure its resource address is the same as the one stored on the component's state.
            // Store the ValidatedProof in a variable.


            // Step 9: Store the amount of passed XRD in an `amount` variable
            // and put the xrd bucket inside the component's `xrd_vault`.


            // Step 10: Get the ID and data of the passed-in proof


            // Step 11: Add the amount of new XRD token staked to the `amount_staked`
            // field of the NFT metadata and update the data on the ledger (by calling the ResourceManager::update_non_fungible_data method).


        }

        // Step 12: Complete the `withdraw` method. Start by adding a Proof parameter named `member_proof`
        // just like with the `stake_xrd` method
        pub fn withdraw(&mut self) -> Bucket {
            // Step 13: validate the proof and store the ValidatedProof in a variable


            
            // Step 14: Use the ValidatedProof to get the ID of the member and the data attached to
            // its NFT



            // Step 15: Store the value of the `amount_staked` field of the NFT's data in a variable



            // Step 16: Set the NFT's data `amount_staked` value to 0 and propagate the changes to the ledger.



            // Step 17: Take the amount of tokens that was staked from the `xrd_vault` and return it.


        }
    }
}