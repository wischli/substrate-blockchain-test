use support::{decl_storage, decl_module, StorageValue, dispatch::Result, ensure};
use system::ensure_signed;
// use std;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Template {
        Value get(get_value): u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		// set current value
        fn set_value(origin, value: u64) -> Result {
            ensure_signed(origin)?;

			// ensure!(value <= std::u64::MAX, "Cannot set value that exceeds u64 max value");

            <Value<T>>::put(value);

            Ok(())
        }

		// increase current_value
        fn increase_value(origin, value: u64) -> Result {
            ensure_signed(origin)?;

	        let current_value = Self::get_value();

        	// ensure!(std::u64::MAX - value == current_value, "New value is too big");

            <Value<T>>::put(value + current_value);

            Ok(())
        }

		// decrease current value
        fn decrease_value(origin, value: u64) -> Result {
            ensure_signed(origin)?;

	        let current_value = Self::get_value();

        	ensure!(current_value >= value, "New value is too negative");

            <Value<T>>::put(current_value - value);

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use support::assert_ok;

    #[test]
	fn set_value_works() {
        assert_ok!(Value::set_value(1, 42));
        assert_ok!(Value::set_value(1, 0));
		assert_eq!(Value::set_value(1, 42), Some(42));
	}

    #[test]
	fn increase_value_works() {
		let inc_value = 1337;
		Value::set_value(1, 42);
		assert_ok!(Value::increase_value(1, inc_value));
		assert_eq!(Value::increase_value(1, inc_value), Some(42 + inc_value));
	}

    #[test]
	fn decrease_value_works() {
		let dec_value = 12;
		Value::set_value(1, 42);
		assert_ok!(Value::increase_value(1, dec_value));
		assert_eq!(Value::increase_value(1, dec_value), Some(42 - dec_value));
	}
}