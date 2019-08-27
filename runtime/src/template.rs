use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result, ensure};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
        Value get(get_value) config(): u64 = 42;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		// set current value
        pub fn set_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

            <Value<T>>::put(value);

			Self::deposit_event(RawEvent::SetValue(sender, value));

            Ok(())
        }

		// increase current_value
        pub fn increase_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

	        let current_value = Self::get_value();

            <Value<T>>::put(value + current_value);

			Self::deposit_event(RawEvent::IncreaseValue(sender, value));

            Ok(())
        }

		// decrease current value
        pub fn decrease_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

	        let current_value = Self::get_value();

			ensure!(current_value >= value, "Cannot decrease by more than current value");

            <Value<T>>::put(current_value - value);

			Self::deposit_event(RawEvent::DecreaseValue(sender, value));

            Ok(())
        }
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		DecreaseValue(AccountId, u64),
		IncreaseValue(AccountId, u64),
		SetValue(AccountId, u64),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn set_value_works() {
		with_externalities(&mut new_test_ext(), || {
			assert_ok!(Value::set_value(1, 42));
			assert_ok!(Value::set_value(1, 0));
			assert_eq!(Value::set_value(1, 42), Some(42));
		});
	}
	#[test]
	fn increase_value_works() {
		with_externalities(&mut new_test_ext(), || {
			let inc_value = 1337;
			Value::set_value(1, 42);
			assert_ok!(Value::increase_value(1, inc_value));
			assert_eq!(Value::increase_value(1, inc_value), Some(42 + inc_value));
		});
	}
	#[test]
	fn decrease_value_works() {
		with_externalities(&mut new_test_ext(), || {
			let dec_value = 12;
			Value::set_value(1, 42);
			assert_ok!(Value::decrease_value(1, dec_value));
			assert_eq!(Value::decrease_value(1, dec_value), Some(42 - dec_value));
		});
	}
}
