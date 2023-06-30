extern crate p256;
extern crate rand;

use std::convert::TryInto;

use crate::p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::elliptic_curve::Field;
use p256::elliptic_curve::group::GroupEncoding;
use p256::NistP256;
use rand::RngCore;
use p256::ProjectivePoint;
use p256::elliptic_curve::SecretKey;
use p256::EncodedPoint;

// Structure for representing an EC point
pub struct ECPoint {
    pub point: ProjectivePoint,
}

// Function to generate a random projective point
fn generate_random_point() -> ProjectivePoint {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let scalar_bytes: &[u8; 32] = &bytes;
    let scalar = p256::Scalar::from_bytes_reduced((&scalar_bytes[..]).try_into().unwrap());
    ProjectivePoint::generator() * scalar
}

// Function to multiply an EC point by a scalar
fn multiply_ec_point(point: &ECPoint, scalar: &p256::Scalar) -> ECPoint {
    ECPoint {
        point: point.point * scalar,
    }
}

// Function to add two EC points
fn add_ec_points(a: &ECPoint, b: &ECPoint) -> ECPoint {
    ECPoint {
        point: a.point + b.point,
    }
}

// Function to double an EC point
fn double_ec_point(a: &ECPoint) -> ECPoint {
    ECPoint {
        point: a.point.double(),
    }
}

fn main() {
    // Generate private keys
    let priv_key1: SecretKey<NistP256> = SecretKey::random(&mut rand::thread_rng());
    let priv_key2: SecretKey<NistP256> = SecretKey::random(&mut rand::thread_rng());

    // Generate public keys
    let scalar1 = priv_key1.to_secret_scalar();
    let scalar2 = priv_key2.to_secret_scalar();
    let pub_key1 = multiply_ec_point(&ECPoint { point: ProjectivePoint::generator() }, &scalar1);
    let pub_key2 = multiply_ec_point(&ECPoint { point: ProjectivePoint::generator() }, &scalar2);

    // Exchange public keys
    let received_pub_key1 = pub_key2;
    let received_pub_key2 = pub_key1;

    // Compute shared secrets
    let shared_secret1 = multiply_ec_point(&received_pub_key1, &scalar1);
    let shared_secret2 = multiply_ec_point(&received_pub_key2, &scalar2);

    // Convert shared secrets to affine points
    let shared_secret1_affine = shared_secret1.point.to_affine();
    let shared_secret2_affine = shared_secret2.point.to_affine();

    // Get X-coordinates of shared secrets
    let binding = shared_secret1_affine.to_encoded_point(false);
    let shared_secret1_x = binding.x();
    let shared_secret2_x = binding.x();

    // Compare shared secrets
    if shared_secret1_x == shared_secret2_x {
        println!("Shared secret: {:?}", shared_secret1_x);
    } else {
        println!("Shared secrets do not match.");
    }
}
