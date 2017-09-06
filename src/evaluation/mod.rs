use std::cmp::Eq;

mod cp_multiplier;

#[derive(Debug)]
/// Represents a Pokemon's level.
pub struct Level {
    val: f64,
}

const MIN_LEVEL: f64 = 1.;
const MAX_LEVEL: f64 = 39.;

impl Level {
    /// Create a new `Level` from the given parameter. Returns `None` if the given value is not a valid
    /// level.
    ///
    /// A value is a valid `Level` if:
    ///
    /// * 1 <= `value` <= 39
    /// * The fractional part of `value` is `0` or `0.5`
    ///
    /// ## Example:
    ///
    /// ```
    /// use pokemon_go_data::Level;
    ///
    /// let level = Level::new(20.5).unwrap();
    /// assert_eq!(level.value(), 20.5);
    ///
    /// assert!(Level::new(20.4).is_none());
    /// ```
    pub fn new<T: Into<f64> + Copy>(level: T) -> Option<Level> {
        let val: f64 = level.into();

        if val < MIN_LEVEL || val > MAX_LEVEL || (val.fract() != 0.5f64 && val.fract() != 0.0f64) {
            return None;
        }

        Some(Level { val })
    }

    /// The internal wrapped value.
    pub fn value(&self) -> f64 {
        self.val
    }

    /// Returns the combat power multiplier corresponding to this level. This value is used to calculate
    /// a Pokemon's combat power from its base stats.
    pub fn cp_multiplier(&self) -> f64 {
        let &(_, cp_multiplier) = cp_multiplier::CP_MULTIPLIER
            .iter()
            .find(|&&(level, _)| level == self.value())
            .unwrap();

        cp_multiplier
    }

    /// Returns the lowest possible Pokemon level.
    ///
    /// ```
    /// use pokemon_go_data::Level;
    ///
    /// assert_eq!(Level::min().value(), 1.0);
    /// ```
    pub fn min() -> Level {
        Level { val: MIN_LEVEL }
    }

    /// Returns the highest possible Pokemon level.
    ///
    /// ```
    /// use pokemon_go_data::Level;
    ///
    /// assert_eq!(Level::max().value(), 39.0);
    /// ```
    pub fn max() -> Level {
        Level { val: MAX_LEVEL }
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Level) -> bool {
        self.value() == other.value()
    }
}

impl Eq for Level { }

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
