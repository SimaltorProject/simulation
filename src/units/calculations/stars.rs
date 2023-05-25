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

pub fn color(temperature: f64) -> [f32; 3] {
	let temperature = temperature / 100.0;
	let red = if temperature <= 66.0 {
		1.0
	} else {
		let mut r = temperature - 60.0;
		r = 329.698727446 * r.powf(-0.1332047592);
		r = r.clamp(0.0, 255.0);
		r / 255.0
	};

	let green = if temperature <= 66.0 {
		let mut g = 99.4708025861 * temperature.ln() - 161.1195681661;
		g = g.clamp(0.0, 255.0);
		g / 255.0
	} else {
		let mut g = temperature - 60.0;
		g = 288.1221695283 * g.powf(-0.0755148492);
		g = g.clamp(0.0, 255.0);
		g / 255.0
	};

	let blue = if temperature >= 66.0 {
		1.0
	} else {
		if temperature <= 19.0 {
			0.0
		} else {
			let mut b = temperature - 10.0;
			b = 138.5177312231 * b.ln() - 305.0447927307;
			b = b.clamp(0.0, 255.0);
			b / 255.0
		}
	};

	[red as f32, green as f32, blue as f32]
}
