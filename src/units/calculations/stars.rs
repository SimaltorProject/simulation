pub fn temperature(mass: f64) -> f64 {
	if mass < 1.5 {
		5778.0 * (luminosity(mass).sqrt() / radius(mass))
	} else {
		10f64.powf(-0.17 * mass.log10().powf(2.0) + 0.888 * mass.log10() + 3.671)
	}
}

pub fn radius(mass: f64) -> f64 {
	if mass <= 1.5 {
		0.438 * mass.powf(2.0) + 0.479 * mass + 0.075
	} else {
		luminosity(mass).sqrt() / (temperature(mass) / 5778.0).powf(2.0)
	}
}

pub fn luminosity(mass: f64) -> f64 {
	match mass {
		x if x <= 0.179 => todo!(),
		x if x <= 0.45 => 10.0f64.powf(2.028 * x.log10() - 0.976),
		x if x <= 0.72 => 10.0f64.powf(4.572 * x.log10() - 0.102),
		x if x <= 1.05 => 10.0f64.powf(5.743 * x.log10() - 0.007),
		x if x <= 2.40 => 10.0f64.powf(4.392 * x.log10() + 0.010),
		x if x <= 7.00 => 10.0f64.powf(3.967 * x.log10() + 0.093),
		x if x <= 31.0 => 10.0f64.powf(2.865 * x.log10() + 1.105),
		_ => todo!(),
	}
}
