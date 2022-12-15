use scrypto::prelude::*;

blueprint! {
    struct Exercise1 {
        // Step 3: Add two variables to the Struct of the blueprint.
        //   - a Vault named `bananas`
        //   - a ResourceAddress named `apples_resource_address`


        
    }

    impl Exercise1 {

        pub fn instantiate_exercise() -> ComponentAddress {

            // Step 1: Create a new fungible resource with a name "Banana" and symbol "BNN".
            // Make the resource burnable with an `allow_all` rule. Don't forget to make this flag unchangeable
            // by specifying `deny_all` in the second parameter.
            // Make the initial supply 1000 and store it inside a `bananas` variable.



            // Step 2: Create a new fungible resource with a name "Apple" and symbol "APP".
            // Make the resource mintable with an `allow_all` rule. Don't forget to make this flag unchangeable
            // by specifying `deny_all` in the second parameter.
            // Initialize the resource with no initial supply and store the ResourceAddress in a `apples` variable.



            Self {
                // Step 4: Initialize the two variables.
                //  - bananas will be a Vault containing the `bananas` bucket we just created.
                //  - apples_resource_address will simply be the `apples` variables we got in step 2.


            }
            .instantiate()
            .globalize()
        }

        // Step 5: Write a `mint_apple` method that returns a Bucket.
        // Inside, borrow the apple resource manager, mint a token and return it.



        // Step 6: Write a `get_banana` method that returns a Bucket.
        // Inside, take a single token from the `bananas` vault and return it.



        // Step 7: Create a `burn_banana` method that accepts a `banana` Bucket.
        // Inside, burn the bucket passed as argument.


    }
}