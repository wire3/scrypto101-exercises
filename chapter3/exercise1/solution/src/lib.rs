use scrypto::prelude::*;

blueprint! {
    struct Exercise1 {
        bananas: Vault,
        apples_resource_address: ResourceAddress
    }

    impl Exercise1 {

        pub fn instantiate_exercise() -> ComponentAddress {

            let bananas: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Banana")
                .metadata("symbol", "BNN")
                .burnable(rule!(allow_all), LOCKED)
                .initial_supply(1000);

            let apples: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "Apple")
                .metadata("symbol", "APP")
                .mintable(rule!(allow_all), LOCKED)
                .no_initial_supply();

            Self {
                bananas: Vault::with_bucket(bananas),
                apples_resource_address: apples
            }
            .instantiate()
            .globalize()
        }

        pub fn mint_apple(&self) -> Bucket {
            borrow_resource_manager!(self.apples_resource_address).mint(1)
        }

        pub fn get_banana(&mut self) -> Bucket {
            self.bananas.take(1)
        }

        pub fn burn_banana(&self, banana: Bucket) {
            banana.burn();
        }
    }
}