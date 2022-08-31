use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok,BoundedVec};

#[test]
fn create_claim_test() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];

		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1,frame_system::Pallet::<Test>::block_number()))
		)
	});
}

#[test]
fn create_claim_test_when_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];

		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_noop!{
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::ProofAlreadyExist
		}
	});
}

#[test]
fn revoke_claim_test() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let sender = Origin::signed(1);
		
		assert_ok!(PoeModule::create_claim(sender.clone(), claim.clone()));

		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_ok!(PoeModule::revoke_claim(sender.clone(), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			None
		)
	});
}

#[test]
fn revoke_claim_test_when_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];

        assert_noop!{
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        }
    });
}

#[test]
fn revoke_claim_test_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!{
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        }
    });
}

#[test]
fn transfer_claim_test() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let sender = Origin::signed(1);
		
		assert_ok!(PoeModule::create_claim(sender.clone(), claim.clone()));

		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1,frame_system::Pallet::<Test>::block_number()))
		);

		assert_ok!(PoeModule::transfer_claim(sender.clone(), claim.clone(), 2));

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2,frame_system::Pallet::<Test>::block_number()))
		)
	});
}

#[test]
fn transfer_claim_failed_when_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        let claim_temp = vec![2, 3];

        assert_noop!{
            PoeModule::transfer_claim(Origin::signed(1), claim_temp.clone(), 2),
            Error::<Test>::ClaimNotExist
        }
    });
}

#[test]
fn transfer_claim_test_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!{
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 2),
            Error::<Test>::NotClaimOwner
		}
    });
}