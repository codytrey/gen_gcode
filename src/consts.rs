pub enum Constants {
    /// Gcode `G21` - sets units to millimeters.
    UseMillimeters,

    /// Gcode `G20` - sets units to inches.
    UseInches,

    /// Gcode `G90` - sets axes to absolute position.
    AbsolutePos,

    /// Gcode `G91` - sets axes to relative position.
    RelativePos,

    /// Gcode `G92.1` - resets to machine's native positioning offsets.
    ResetPos,

    /// Gcode `G28` - triggers autohome procedure, using default parameters set in machine
    /// firmware.
    AutoHome,

    /// Gcode `M82` - sets extruder axis to absolute mode (independant of other axis).
    AbsoluteExtrution,

    /// Gcode `M83` - sets extruder axis to relative mode (independant of other axis).
    RelativeExtrution,
}

impl Constants {
    /// Converts enum elements to str as gcode instruction.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UseMillimeters    => "G21",
            Self::UseInches         => "G20",
            Self::AbsolutePos       => "G90",
            Self::RelativePos       => "G91",
            Self::ResetPos          => "G92.1",
            Self::AutoHome          => "G28",
            Self::AbsoluteExtrution => "M82",
            Self::RelativeExtrution => "M83",
        }
    }

    /// Similar to `Constants::as_str` but with '\n' at end.
    pub fn as_str_end(&self) -> &'static str {
        match self {
            Self::UseMillimeters    => "G21\n",
            Self::UseInches         => "G20\n",
            Self::AbsolutePos       => "G90\n",
            Self::RelativePos       => "G91\n",
            Self::ResetPos          => "G92.1\n",
            Self::AutoHome          => "G28\n",
            Self::AbsoluteExtrution => "M82\n",
            Self::RelativeExtrution => "M83\n",
        }
    }
}
