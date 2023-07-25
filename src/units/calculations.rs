pub(crate) mod stars;

pub(crate) fn _roche_limit(r_body: f64, m_body: f64, m_satelite: f64) -> f64 {
	r_body * ((2.0 * m_body) / m_satelite).powf(1.0 / 3.0)
}
