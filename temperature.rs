use chrono::NaiveTime;

/// Irradiance d'un point, en Watts par mètre carré, en fonction de la masse d'air
/// devant être traversée par les rayons du Soleil.
pub fn irradiance(air_mass: f64) -> f64 {
    1.353 * 0.7f64.powf(air_mass.powf(0.678)) * 1000.0
}

/// Masse d'air devant être traversée par les rayons du Soleil en fonction de l'angle
/// zénithal.
pub fn air_mass(zenith_angle: f64) -> f64 {
    assert!(0.0 <= zenith_angle && zenith_angle <= 90.0);
    1.0 / zenith_angle.to_radians().cos()
}

/// Angle zénithal en fonction de l'angle d'élévation.
pub fn zenith_angle(elevation_angle: f64) -> f64 {
    assert!(0.0 <= elevation_angle && elevation_angle <= 90.0);
    90.0 - elevation_angle
}

/// Angle d'élévation en fonction de l'angle de déclinaison, de la latitude et de l'angle horaire.
pub fn elevation_angle(declination_angle: f64, latitude: f64, hour_angle: f64) -> f64 {
    assert!(-23.45 <= declination_angle && declination_angle <= 23.45);
    assert!(-90.0 <= latitude && latitude <= 90.0);
    assert!(-180.0 <= hour_angle && hour_angle <= 180.0);
    (declination_angle.to_radians().sin() * latitude.to_radians().sin() +
     declination_angle.to_radians().cos() * latitude.to_radians().cos() *
     hour_angle.to_radians().cos())
        .asin()
        .to_degrees()
}

/// Angle de déclinaison en fonction du jour de l'année.
pub fn declination_angle(day_of_year: u32) -> f64 {
    assert!(1 <= day_of_year && day_of_year <= 366); // leap years

    (23.45f64.to_radians().sin() * ((360.0 / 365.0) * (day_of_year - 81) as f64).to_radians().sin())
        .asin()
        .to_degrees()
}

/// Angle horaire, en degrés, en fonction de l'heure solaire.
pub fn hour_angle(solar_time: f64) -> f64 {
    assert!(0.0 <= solar_time && solar_time <= 24.0);
    15.0 * (solar_time - 12.0)
}

/// Heure solaire en fonction de l'heure locale et du facteur de correction de l'heure.
pub fn solar_time(local_time: f64, time_correction_factor: f64) -> f64 {
    local_time + time_correction_factor / 60.0
}

/// Facteur de correction de l'heure, en minutes, en fonction de la longitude du point étudié,
/// de la longitude du méridien standard local et de l'équation du temps.
pub fn time_correction_factor(longitude: f64,
                              local_standard_meridian_longitude: f64,
                              equation_of_time: f64)
                              -> f64 {
    assert!(-15.0 <= equation_of_time && equation_of_time <= 20.0);
    4.0 * (longitude - local_standard_meridian_longitude) + equation_of_time
}

/// Longitude du méridien standard local en fonction du décalage horaire par rapport
/// au fuseau horaire GMT.
pub fn local_standard_meridian_longitude(gmt_offset: f64) -> f64 {
    15.0 * gmt_offset
}

/// Équation du temps en fonction du jour de l'année, en minutes.
pub fn equation_of_time(day_of_year: u32) -> f64 {
    let b = (360.0 / 365.0) * (day_of_year - 81) as f64;
    9.87 * (2.0 * b).to_radians().sin() - 7.53 * b.to_radians().cos() - 1.5 * b.to_radians().sin()
}
