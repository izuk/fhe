/// file: main.rs
use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // Set up the keys.
    let sk_rlwe = RLWESecretKey::new(&RLWE128_2048_1);
    let sk_lwe = sk_rlwe.to_lwe_secret_key();
    // HOW DO YOU CHOOSE THESE PARAMS?
    //let bsk = LWEBSK::new(&sk_lwe, &sk_rlwe, 5, 4);

    // HOW DO YOU CHOOSE THESE PARAMS?
    let encoder = Encoder::new(-10., 10., 10, 7)?;

    let input: Vec<f64> = vec![-6.276, 4.3, 7.78];
    let ciphertext = VectorLWE::encode_encrypt(&sk_lwe, &input, &encoder)?;

    let weights: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 6.0, 9.0],
    ];
    let bias: Vec<f64> = vec![1.0, 1.0, 1.0];

    let x1 = matmul(&weights, &ciphertext, 9.0, 7)?;
    let x2 = add(&bias, &x1)?;
    //let x3 = x2.bootstrap_nth(&bsk, 0)?;
    let outputs: Vec<f64> = x2.decrypt_decode(&sk_lwe)?;
    println!("{:?}", outputs);
    Ok(())
}

fn add(x: &Vec<f64>, y: &VectorLWE) -> Result<VectorLWE, CryptoAPIError> {
    return y.add_constant_static_encoder(x);
}

fn dot(x: &Vec<f64>, y: &VectorLWE, max: f64, padding: usize) -> Result<VectorLWE, CryptoAPIError> {
    let v = y.mul_constant_with_padding(&x, max, padding)?;
    return v.sum_with_padding();
}

fn matmul(a: &Vec<Vec<f64>>, y: &VectorLWE, max: f64, padding: usize) -> Result<VectorLWE, CryptoAPIError> {
    let mut res = VectorLWE::zero(2048, y.nb_ciphertexts)?;
    for (i, x) in a.iter().enumerate() {
        let v = dot(x, y, max, padding)?;
        res.copy_in_nth_nth_inplace(i, &v, 0)?;
    }
    return Ok(res);
}
