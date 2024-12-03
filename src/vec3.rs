use rand::Rng;
use nalgebra::{vector, SVector, Unit};

use crate::NEG_ONE_ONE_CLOSED;

pub fn near_zero(vec: SVector<f64, 3>) -> bool {
    let s = 1e-8;
    vec[0].abs() < s && vec[1].abs() < s && vec[2].abs() < s
}

fn random_within_unit_sphere() -> SVector<f64, 3> {
    let mut rand = rand::thread_rng();
    loop {
        let p = SVector::<f64, 3>::from_distribution(&*NEG_ONE_ONE_CLOSED, &mut rand);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Unit<SVector<f64, 3>> {
    Unit::new_normalize(random_within_unit_sphere())
}

pub fn random_within(min: f64, max: f64) -> SVector<f64, 3> {
    let mut rand = rand::thread_rng();
    vector![
        rand.gen_range(min..max),
        rand.gen_range(min..max),
        rand.gen_range(min..max),
    ]
}

pub fn random() -> SVector<f64, 3> {
    let mut rand = rand::thread_rng();
    SVector::<f64, 3>::from_distribution(&rand::distributions::Standard, &mut rand)
}

pub fn reflect(v: SVector<f64, 3>, n: SVector<f64, 3>) -> SVector<f64, 3> {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(uv: SVector<f64, 3>, n: SVector<f64, 3>, etai_over_etat: f64) -> SVector<f64, 3> {
    let cos_theta = (-uv).dot(&n).min(1.0);
    let perp = etai_over_etat * (uv + cos_theta * n);
    let parallel = -(1.0 - perp.magnitude_squared()).abs().sqrt() * n;

    perp + parallel
}

pub fn mul(lhs: SVector<f64, 3>, rhs: SVector<f64, 3>) -> SVector<f64, 3> {
    vector!(lhs[0] * rhs[0], lhs[1] * rhs[1], lhs[2] * rhs[2])
}
