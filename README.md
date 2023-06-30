# ECDH (Elliptic Curve Diffie-Hellman)

Here is a description of each function in the provided code:

1. `generate_random_point()`: This function generates a random projective point on the elliptic curve. It uses the `rand` crate to generate random bytes, converts them into a scalar, and multiplies the generator point of the elliptic curve by the scalar to obtain the random point.

2. `multiply_ec_point(point: &ECPoint, scalar: &p256::Scalar) -> ECPoint`: This function takes an EC point (`point`) and a scalar value (`scalar`) and returns the result of multiplying the point by the scalar. It performs scalar multiplication using the `*` operator on the `point`'s underlying `ProjectivePoint` and the `scalar`.

3. `add_ec_points(a: &ECPoint, b: &ECPoint) -> ECPoint`: This function takes two EC points (`a` and `b`) and returns their sum. It adds the underlying `ProjectivePoint` of `a` and `b` using the `+` operator and wraps the result in a new `ECPoint` struct.

4. `double_ec_point(a: &ECPoint) -> ECPoint`: This function takes an EC point (`a`) and returns its doubling. It doubles the underlying `ProjectivePoint` of `a` using the `double()` method provided by the `p256` crate and wraps the result in a new `ECPoint` struct.

The `main()` function demonstrates the usage of these functions to perform the ECDH protocol. It generates two random private keys (`priv_key1` and `priv_key2`) using the `SecretKey::random()` method. Then, it computes the corresponding public keys (`pub_key1` and `pub_key2`) by multiplying the generator point by the respective private keys. The public keys are exchanged between the participants.

Next, the shared secrets (`shared_secret1` and `shared_secret2`) are computed by multiplying the received public keys with the respective private keys. These shared secrets are then converted to affine points (`shared_secret1_affine` and `shared_secret2_affine`).

Finally, the X-coordinates of the shared secrets are extracted and compared. If they match, the shared secret is printed. Otherwise, a message indicating that the shared secrets do not match is printed.

Result:
![image](https://github.com/soffije/ECDH/assets/93443981/dd02881b-1a14-4474-bbc1-e9a906b048b0)
