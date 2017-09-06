mod cp_multiplier;

pub struct Level {
    val: f64,
}

const MIN_LEVEL: f64 = 1.;
const MAX_LEVEL: f64 = 39.;

impl Level {
    pub fn new<T: Into<f64> + Copy>(level: T) -> Option<Level> {
        let val: f64 = level.into();

        if val < MIN_LEVEL || val > MAX_LEVEL || (val.fract() != 0.5f64 && val.fract() != 0.0f64) {
            return None;
        }

        Some(Level { val })
    }

    pub fn value(&self) -> f64 {
        self.val
    }

    pub fn cp_multiplier(&self) -> f64 {
        let &(_, cp_multiplier) = cp_multiplier::CP_MULTIPLIER
            .iter()
            .find(|&&(level, _)| level == self.value())
            .unwrap();

        cp_multiplier
    }

    pub fn min() -> Level {
        Level { val: MIN_LEVEL }
    }

    pub fn max() -> Level {
        Level { val: MAX_LEVEL }
    }
}

#[cfg(test)]
mod tests {
    use Level;

    #[test]
    fn level_construction() {
        let good = Level::new(34.5).unwrap();
        assert_eq!(34.5, good.value());

        let too_low = Level::new(0.5);
        assert!(too_low.is_none());

        let too_high = Level::new(40.5);
        assert!(too_high.is_none());

        let wrong_fractional = Level::new(11.1);
        assert!(wrong_fractional.is_none());
    }

    #[test]
    fn cp_multiplier() {
        let level = Level::new(10.).unwrap();
        assert_eq!(0.42250001, level.cp_multiplier());

        let level = Level::new(37.5).unwrap();
        assert_eq!(0.776064962, level.cp_multiplier());
    }
}
