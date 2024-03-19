use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;

use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;

fn main() {
    let private_key: usize = 0x6C616D6264617370;
    let public_key = BLS12381Curve::generator().operate_with_self(private_key);

    println!("{:?}", public_key);
}
