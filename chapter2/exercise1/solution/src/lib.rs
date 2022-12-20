use scrypto::prelude::*;

#[scrypto(TypeId, Encode, Decode, Debug, Describe)]
enum Vegetable {
    Tomato,
    Carrot,
    Broccoli
}

blueprint! {
    struct Exercise1 {
        instantiated_at: u64,
        instantiator_name: String,
        favorite_vegetable: Vegetable
    }

    impl Exercise1 {

        pub fn instantiate_exercise(instantiator_name: String) -> ComponentAddress {
            Self {
                instantiated_at: Runtime::current_epoch(),
                instantiator_name: instantiator_name,
                favorite_vegetable: Vegetable::Broccoli
            }
            .instantiate()
            .globalize()
        }

        pub fn log_data(&self) {
            error!("Instantiated by {} at epoch {}", self.instantiator_name, self.instantiated_at);
            debug!("The favorite vegetable is: {:?}", self.favorite_vegetable);
        }
    }
}