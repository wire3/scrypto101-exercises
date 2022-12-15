use scrypto::prelude::*;

// Step 4: Create a `Vegetable` enum with the following fields:
//   - Tomato
//   - Carrot
//   - Broccoli
// Don't forget to add a call the the `scrypto` macro with TypeId, Encode, Decode, Describe and Debug.
// This last one, Debug, is neccessary as we are going to display it's value.




blueprint! {
    struct Exercise1 {
        // Step 1: Add two variables here. 
        //   - One u64 named `instantiated_at`. This will contain the epoch at which the component was instantiated
        //   - One String named `instantiator_name`


        // Step 5: Add a variable of type Vegetable named `favorite_vegetable`


    }

    impl Exercise1 {

        // Step 2: add a String argument named `instantiator_name` to the following function
        pub fn instantiate_exercise(  ) -> ComponentAddress {
            Self {
                // Step 3: Assign a value to the `instantiated_at` and `instantiator_name` variables.
                // Help: You can get the current epoch with `Runtime::current_epoch()`


                // Step 6: Assign a value to the `favorite_vegetable` variable


            }
            .instantiate()
            .globalize()
        }

        // Step 7: Create a `log_data` method that does the following:
        //   - Display the instantiator_name and instantiated_at variables using the `error!` macro
        //   - Display the favorite vegetable using the `debug!` macro
        // Help: You can display formatted messages using the following notation: 
        //   "blah {} blah" -> for instantiator_name and instantiated_at
        //   "blah {:?} blah" -> for the favorite_vegeratable. This is different notation than the other variables because
        //     the enum has a Debug implementation but not a Display one

        
    }
}