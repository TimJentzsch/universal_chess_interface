/// An engine option that can be set by the GUI.
pub struct EngineOption {
    /// The name of the option.
    pub name: String,

    /// The type of the option.
    pub r#type: OptionType,
}

impl EngineOption {
    /// Create a new engine option.
    pub fn new<N>(name: N, r#type: OptionType) -> Self
    where
        N: Into<String>,
    {
        Self {
            name: name.into(),
            r#type,
        }
    }

    /// Create a new option for the size of the hashtable.
    ///
    /// All values must be in MB.
    ///
    /// # Panics
    ///
    /// This function panics if
    ///
    /// - `min_mb` > `max_mb`
    /// - `default_mb` < `min_mb`
    /// - `default_mb` > `max_mb`
    pub fn hash(min_mb: usize, max_mb: usize, default_mb: usize) -> Self {
        Self::new(
            "Hash",
            OptionType::Spin(SpinOption::new(min_mb, max_mb, default_mb)),
        )
    }
}

/// The type of the engine option.
pub enum OptionType {
    /// A checkbox that can take the values `true` or `false`.
    Check(CheckOption),

    /// A spin wheel that can be an integer in a certain range.
    Spin(SpinOption),

    /// A combo box that can have different predefined strings as value.
    Combo(ComboOption),

    /// A button that can be pressed to send a command to the engine.
    Button,

    /// A text field that has a string as a value.
    String(StringOption),
}

/// A checkbox that can take the values `true` or `false`.
pub struct CheckOption {
    /// The default value for the checkbox.
    pub default: bool,
}

/// A spin wheel that can be an integer in a certain range.
pub struct SpinOption {
    /// The default value for the spin wheel.
    pub default: usize,

    /// The minimum value of the spin wheel.
    pub min: usize,

    /// The maximum value of the spin wheel.
    pub max: usize,
}

impl SpinOption {
    /// Create a new spin option.
    ///
    /// The `min` must be smaller or equal to `max`.
    /// The `default` must be between `min` and `max`.
    pub fn new(min: usize, max: usize, default: usize) -> Self {
        assert!(min <= max, "The min {min} bigger than the max {max}");
        assert!(
            min <= default && default <= max,
            "The default {default} must be between the min {min} and the max {max}"
        );

        Self { min, max, default }
    }
}

/// A combo box that can have different predefined strings as value.
pub struct ComboOption {
    /// The default value for the combo box.
    pub default: String,

    /// The possible values the combo box can have.
    pub values: Vec<String>,
}

/// A text field that has a string as a value.
pub struct StringOption {
    /// The default value of the text field.
    pub default: String,
}
