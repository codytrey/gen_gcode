//! Generate G-Code with funcational operation describing motion of the machine that the created gcode should produce

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_move_xy() {
        let p = Point2d { x: 10.0, y: 5.0 };
        let gcode = move_xy(p, None, None);
        assert_eq!("G0 X10 Y5\n", gcode);
    }

    #[test]
    fn test_move_xy_with_f() {
        let p = Point2d { x: 10.0, y: 5.0 };
        let gcode = move_xy(p, Some(400), None);
        assert_eq!("G0 X10 Y5 F400\n", gcode);
    }

    #[test]
    fn test_move_xy_with_e() {
        let p = Point2d { x: 10.0, y: 5.0 };
        let gcode = move_xy(p, None, Some(5.));
        assert_eq!("G1 X10 Y5 E5\n", gcode);
    }

    #[test]
    fn test_move_xy_with_f_and_e() {
        let p = Point2d { x: 10.0, y: 5.0 };
        let gcode = move_xy(p, Some(400), Some(5.));
        assert_eq!("G1 X10 Y5 E5 F400\n", gcode);
    }

    #[test]
    fn test_move_xyz() {
        let p = Point3d { x: 10.0, y: 5.0, z: 0.2 };
        let gcode = move_xyz(p, None, None);
        assert_eq!("G0 X10 Y5 Z0.2\n", gcode);
    }

    #[test]
    fn test_move_xyz_with_f() {
        let p = Point3d { x: 10.0, y: 5.0, z: 0.17 };
        let gcode = move_xyz(p, Some(400), None);
        assert_eq!("G0 X10 Y5 Z0.17 F400\n", gcode);
    }

    #[test]
    fn test_move_xyz_with_e() {
        let p = Point3d { x: 10.0, y: 5.0, z: 0.17 };
        let gcode = move_xyz(p, None, Some(5.));
        assert_eq!("G1 X10 Y5 Z0.17 E5\n", gcode);
    }

    #[test]
    fn test_move_xyz_with_f_and_e() {
        let p = Point3d { x: 10.0, y: 5.0, z: 0.17 };
        let gcode = move_xyz(p, Some(400), Some(5.));
        assert_eq!("G1 X10 Y5 Z0.17 E5 F400\n", gcode);
    }

    #[test]
    fn test_arc_ij_i() {
        let p = Point2d { x: 125.0, y: 0.0 };
        let gcode = move_xy_arc_ij(Some(p), Some(62.5), None, None, false);
        assert_eq!("G2 X125 Y0 I62.5\n", gcode);
    }

    #[test]
    fn test_arc_ij_j() {
        let p = Point2d { x: 125.0, y: 0.0 };
        let gcode = move_xy_arc_ij(Some(p), None, Some(62.5), None, false);
        assert_eq!("G2 X125 Y0 J62.5\n", gcode);
    }

    #[test]
    fn test_arc_ij_full_circle() {
        let p = Point2d { x: 220.0, y: 110.0 };
        // Move to the point (220,110)
        let _ = move_xy(p, None, None);
        // Make a counter clockwise circle with centerpoint at (110,110)
        let gcode = move_xy_arc_ij(None, Some(110.0), Some(110.0), Some(920.0), true);
        assert_eq!("G3 I110 J110 E920\n", gcode);
    }

    #[test]
    fn test_set_pos_2d() {
        let p = Point2d { x: 125.0, y: 125.0 };
        let gcode = set_pos_2d(p, None);
        assert_eq!("G92 X125 Y125\n", gcode);
    }

    #[test]
    fn test_set_pos_2d_with_e() {
        let p = Point2d { x: 125.0, y: 125.0 };
        let gcode = set_pos_2d(p, Some(90.0));
        assert_eq!("G92 X125 Y125 E90\n", gcode);
    }

    #[test]
    fn test_set_pos_3d() {
        let p = Point3d { x: 125.0, y: 125.0, z: 25.0};
        let gcode = set_pos_3d(p, None);
        assert_eq!("G92 X125 Y125 Z25\n", gcode);
    }

    #[test]
    fn test_set_pos_3d_with_e() {
        let p = Point3d { x: 125.0, y: 125.0, z: 25.0 };
        let gcode = set_pos_3d(p, Some(90.0));
        assert_eq!("G92 X125 Y125 Z25 E90\n", gcode);
    }

    #[test]
    fn test_reset_extruder() {
        let gcode = reset_extruder(0.0);
        assert_eq!("G92 E0\n", gcode);
    }

    #[test]
    fn test_rest_pos() {
        let gcode = reset_pos();
        assert_eq!("G92.1\n", gcode);
    }

    #[test]
    fn test_set_hotend_temp() {
        let gcode = set_hotend_temp(210, None);
        assert_eq!("M104 S210\n", gcode);
    }

    #[test]
    fn test_set_hotend_temp_non_default_tool() {
        let gcode = set_hotend_temp(210, Some(2));
        assert_eq!("M104 S210 T2\n", gcode);
    }

    #[test]
    fn test_wait_hotend_temp() {
        let gcode = wait_hotend_temp(210, None);
        assert_eq!("M109 S210\n", gcode);
    }

    #[test]
    fn test_wait_hotend_temp_non_default_tool() {
        let gcode = wait_hotend_temp(210, Some(2));
        assert_eq!("M109 S210 T2\n", gcode);
    }

    #[test]
    fn test_set_fan_speed() {
        //set default fan to half speed
        let gcode = set_fan_speed(128, None);
        assert_eq!("M106 S128\n", gcode);
    }

    #[test]
    fn test_set_fan_speed_atl_fan() {
        //set alternate fan to full speed
        let gcode = set_fan_speed(u8::MAX, Some(1));
        assert_eq!("M106 S255 P1\n", gcode);
    }

    #[test]
    fn test_fan_off() {
        let gcode = fan_off(None);
        assert_eq!("M107\n", gcode);
    }

    #[test]
    fn test_fan_off_alt_fan() {
        let gcode = fan_off(Some(3));
        assert_eq!("M107 P3\n", gcode);
    }

    #[test]
    fn test_test_bed_temp() {
        let gcode = set_bed_temp(210);
        assert_eq!("M140 S210\n", gcode);
    }

    #[test]
    fn test_wait_bed_temp() {
        let gcode = wait_bed_temp(210);
        assert_eq!("M190 S210\n", gcode);
    }

    #[test]
    fn test_test_chamber_temp() {
        let gcode = set_chamber_temp(50);
        assert_eq!("M141 S50\n", gcode);
    }

    #[test]
    fn test_wait_chamber_temp() {
        let gcode = wait_chamber_temp(50);
        assert_eq!("M191 S50\n", gcode);
    }

    #[test]
    fn test_auto_home() {
        let gcode = auto_home();
        assert_eq!("G28\n", gcode);
    }

    #[test]
    fn test_absolute_extrution() {
        let gcode = absolute_extrution();
        assert_eq!("M82\n", gcode);
    }

    #[test]
    fn test_relative_extrution() {
        let gcode = relative_extrution();
        assert_eq!("M83\n", gcode);
    }

    #[test]
    fn test_move_z() {
        let gcode = move_z(1.8);
        assert_eq!("G0 Z1.8\n", gcode);
    }

    #[test]
    fn test_relative_positioning() {
        let gcode = relative_positioning();
        assert_eq!("G91\n", gcode);
    }

    #[test]
    fn test_absolute_positioning() {
        let gcode = absolute_positioning();
        assert_eq!("G90\n", gcode);
    }

    #[test]
    fn test_use_inches() {
        let gcode = use_inches();
        assert_eq!("G20\n", gcode);
    }

    #[test]
    fn test_use_millimeters() {
        let gcode = use_millimeters();
        assert_eq!("G21\n", gcode);
    }

}


/// Defines a 2 dimentional point in the XY catersian coordanant system
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::Point2d;
///
/// let p1 = Point2d { x: 0.0, y: 0.0 };
/// let p2 = Point2d { x: 10.0, y: 0.0 };
/// let p3 = Point2d { x: 10.0, y: 10.0 };
/// let p4 = Point2d { x: 0.0, y: 10.0 };
/// let square: Vec<Point2d> = vec!(p1, p2, p3, p4);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

/// Defines a 3 dimentional point in the XYZ catersian coordanant system
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::Point3d;
///
/// let p1 = Point3d { x: 0.0, y: 0.0, z: 0.0 };
/// let p2 = Point3d { x: 10.0, y: 0.0, z: 0.0 };
/// let p3 = Point3d { x: 10.0, y: 10.0, z: 0.0 };
/// let p4 = Point3d { x: 0.0, y: 10.0, z: 0.0 };
/// let p5 = Point3d { x: 0.0, y: 10.0, z: 10.0 };
/// let p6 = Point3d { x: 10.0, y: 10.0, z: 10.0 };
/// let p7 = Point3d { x: 10.0, y: 0.0, z: 10.0 };
/// let p8 = Point3d { x: 0.0, y: 0.0, z: 10.0 };
/// let cube: Vec<Point3d> = vec!(p1, p2, p3, p4, p5, p6, p7, p8);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Returns a G1 or G0 command as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy};
///
/// let p = Point2d { x: 10.0, y: 5.0 };
/// // move without extruding
/// let gcode = move_xy(p, None, None);
/// assert_eq!("G0 X10 Y5\n", gcode);
/// ```
///
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy};
///
/// let p = Point2d { x: 10.0, y: 5.0 };
/// // move with extrude
/// let gcode = move_xy(p, None, Some(5.0));
/// assert_eq!("G1 X10 Y5 E5\n", gcode);
/// ```
///
pub fn move_xy(dest: Point2d, feed_rate: Option<u32>, flow_rate: Option<f32>) -> String {
    let f_str = match feed_rate {
        Some(feed_rate) => format!(" F{}", feed_rate),
        None => String::new(),
    };

    if let Some(maybe_flow_rate) = flow_rate {
        let e_str = format!(" E{}", maybe_flow_rate);
        return format!(
            "G1 X{x} Y{y}{e_str}{f_str}\n",
            x = dest.x,
            y = dest.y,
        );
    }

    format!("G0 X{x} Y{y}{f_str}\n", x = dest.x, y = dest.y)
}

/// Takes a [Point3d] as input, returns a G1 or G0 command to move in 3 dimentionsReturns as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point3d, move_xyz};
///
/// let p = Point3d { x: 10.0, y: 5.0, z: 15.0 };
/// // move without extruding
/// let gcode = move_xyz(p, None, None);
/// assert_eq!("G0 X10 Y5 Z15\n", gcode);
/// ```
///
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point3d, move_xyz};
///
/// let p = Point3d { x: 10.0, y: 5.0, z: 0.2 };
/// // move with extrude
/// let gcode = move_xyz(p, None, Some(5.0));
/// assert_eq!("G1 X10 Y5 Z0.2 E5\n", gcode);
/// ```
///
pub fn move_xyz(dest: Point3d, feed_rate: Option<u32>, flow_rate: Option<f32>) -> String {
    let f_str = match feed_rate {
        Some(feed_rate) => format!(" F{}", feed_rate),
        None => String::new(),
    };

    if let Some(flow_rate) = flow_rate {
        let e_str = format!(" E{}", flow_rate);
        return format!(
            "G1 X{x} Y{y} Z{z}{e_str}{f_str}\n",
            x = dest.x,
            y = dest.y,
            z = dest.z
        );
    }

    format!(
        "G0 X{x} Y{y} Z{z}{f_str}\n",
        x = dest.x,
        y = dest.y,
        z = dest.z
    )
}

/// Takes an [f32] value as a location on the Z axis to move to, Returns a G0 command
/// Useful in layerchanges and z-hops as this function does not take arguments to extrude.
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::move_z;
///
/// let gcode = move_z(1.8);
/// assert_eq!("G0 Z1.8\n", gcode);
/// ```
pub fn move_z(z: f32) -> String {
    format!("G0 Z{}\n", z)
}

/// Returns a G2 or G3 command as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy_arc_ij};
///
/// let p = Point2d { x: 125.0, y: 0.0 };
/// // Create a Clockwise 180 degree Arc starting at 0,0 ending at 125,0 with center point 62.5,0
/// let gcode = move_xy_arc_ij(Some(p), Some(62.5), None, None, false);
/// assert_eq!("G2 X125 Y0 I62.5\n", gcode);
/// ```
///
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy, move_xy_arc_ij};
///
/// let p = Point2d { x: 220.0, y: 110.0 };
/// // Move to the point (220,110)
/// let _ = move_xy(p, None, None);
/// // Make a counter clockwise circle with centerpoint at (110,110)
/// let gcode = move_xy_arc_ij(None, Some(110.0), Some(110.0), Some(920.0), true);
/// assert_eq!("G3 I110 J110 E920\n", gcode);
/// ```
pub fn move_xy_arc_ij(
    dest: Option<Point2d>,
    x_offset: Option<f32>,
    y_offset: Option<f32>,
    flow_rate: Option<f32>,
    ccw: bool,
) -> String {
    let (x_str, y_str) = match dest {
        Some(dest) => (format!(" X{}", dest.x), format!(" Y{}", dest.y)),
        None => (String::new(), String::new()),
    };

    let i_str = match x_offset {
        Some(x_offset) => format!(" I{}", x_offset),
        None => String::new(),
    };

    let j_str = match y_offset {
        Some(y_offset) => format!(" J{}", y_offset),
        None => String::new(),
    };

    let e_str = match flow_rate {
        Some(flow_rate) => format!(" E{}", flow_rate),
        None => String::new(),
    };

    if ccw {
        return format!(
            "G3{x}{y}{i}{j}{e}\n",
            i = i_str,
            j = j_str,
            x = x_str,
            y = y_str,
            e = e_str
        );
    }

    format!(
        "G2{x}{y}{i}{j}{e}\n",
        i = i_str,
        j = j_str,
        x = x_str,
        y = y_str,
        e = e_str
    )
}

/// Returns a G21 command as a String
///
/// Sets units to millimeters
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::use_millimeters;
///
/// let gcode = use_millimeters();
/// assert_eq!("G21\n", gcode);
/// ```
pub fn use_millimeters() -> String {
    "G21\n".to_string()
}

/// Returns a G20 command as a String
///
/// Sets units to inches
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::use_inches;
///
/// let gcode = use_inches();
/// assert_eq!("G20\n", gcode);
/// ```
pub fn use_inches() -> String {
    "G20\n".to_string()
}

/// Returns a G90 command as a String
///
/// sets all axes to absolute positioning (relative to home, ie. (0,0))
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::absolute_positioning;
///
/// let gcode = absolute_positioning();
/// assert_eq!("G90\n", gcode);
/// ```
pub fn absolute_positioning() -> String {
    "G90\n".to_string()
}

/// Returns a G91 command as a String
///
/// sets all axes to relative positioning (relative to nozzle/tool position)
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::relative_positioning;
///
/// let gcode = relative_positioning();
/// assert_eq!("G91\n", gcode);
/// ```
pub fn relative_positioning() -> String {
    "G91\n".to_string()
}

/// Returns a G92 command to set the current nozzle/tool possition in the XY plane as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, set_pos_2d};
///
/// let p = Point2d { x: 125.0, y: 125.0 };
/// let gcode = set_pos_2d(p, None);
/// assert_eq!("G92 X125 Y125\n", gcode);
/// ```
pub fn set_pos_2d(pos: Point2d, extrude_pos: Option<f32>) -> String {
    let e_str = match extrude_pos {
        Some(extrude_pos) => format!(" E{}", extrude_pos),
        None => String::new(),
    };

    format!("G92 X{x} Y{y}{e_str}\n", x = pos.x, y = pos.y)
}

/// Returns a G92 command to set the current nozzle/tool possition in 3 dimentions (XYZ) as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point3d, set_pos_3d};
///
/// let p = Point3d { x: 125.0, y: 125.0, z: 25.0};
/// let gcode = set_pos_3d(p, None);
/// assert_eq!("G92 X125 Y125 Z25\n", gcode);
/// ```
pub fn set_pos_3d(pos: Point3d, extrude_pos: Option<f32>) -> String {
    let e_str = match extrude_pos {
        Some(extrude_pos) => format!(" E{}", extrude_pos),
        None => String::new(),
    };

    format!(
        "G92 X{x} Y{y} Z{z}{e}\n",
        x = pos.x,
        y = pos.y,
        z = pos.z,
        e = e_str
    )
}

/// Returns a G92 command to set the extruder possition (E axis) as a string
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::reset_extruder;
///
/// let gcode = reset_extruder(0.0);
/// assert_eq!("G92 E0\n", gcode);
/// ```
pub fn reset_extruder(extrude_pos: f32) -> String {
    format!("G92 E{}\n", extrude_pos)
}

/// Returns a G92.1 command to reset to machine's native possitioning offsets as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::reset_pos;
///
/// let gcode = reset_pos();
/// assert_eq!("G92.1\n", gcode);
/// ```
pub fn reset_pos() -> String {
    "G92.1\n".to_string()
}

/// Returns a M104 command to set target hotend temp as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::set_hotend_temp;
///
/// let gcode = set_hotend_temp(210, None);
/// assert_eq!("M104 S210\n", gcode);
/// ```
///
/// To specify an extruder other than default (last active):
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::set_hotend_temp;
/// // some(2) is the extruder index on the printer
/// let gcode = set_hotend_temp(210, Some(2));
/// assert_eq!("M104 S210 T2\n", gcode);
/// ```
pub fn set_hotend_temp(temp: u16, hotend: Option<u8>) -> String {
    let t_str = match hotend {
        Some(hotend) => format!(" T{}", hotend), 
        None => String::new()
    };

    format!("M104 S{s}{t}\n", s = temp, t = t_str)
}

/// Returns a M109 command to set target hotend temp to wait to reach as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::wait_hotend_temp;
///
/// let gcode = wait_hotend_temp(210, None);
/// assert_eq!("M109 S210\n", gcode);
/// ```
///
/// To specify an extruder other than default (last active):
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::wait_hotend_temp;
/// // some(2) is the extruder index on the printer
/// let gcode = wait_hotend_temp(210, Some(2));
/// assert_eq!("M109 S210 T2\n", gcode);
/// ```
pub fn wait_hotend_temp(temp: u16, hotend: Option<u8>) -> String {
    let t_str = match hotend {
        Some(hotend) => format!(" T{}", hotend),
        None => String::new(),
    };

    format!("M109 S{s}{t}\n", s = temp, t = t_str)
}

/// Returns a M106 command to set the fan speed, with optional fan index, as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::set_fan_speed;
///
/// //set default fan to half speed
/// let gcode = set_fan_speed(128, None);
/// assert_eq!("M106 S128\n", gcode);
/// ```
///
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::set_fan_speed;
///
/// //set alternate fan to full speed
/// let gcode = set_fan_speed(u8::MAX, Some(1));
/// assert_eq!("M106 S255 P1\n", gcode);
/// ```
pub fn set_fan_speed(speed: u8, fan: Option<u8>) -> String {
    let p_str = match fan {
        Some(maybe_fan) => format!(" P{}", maybe_fan),
        None => String::new(),
    };

    format!("M106 S{s}{p}\n", s = speed, p = p_str)
}

/// Returns a M107 command to disable the fan, with optional fan index, as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::fan_off;
///
/// let gcode = fan_off(None);
/// assert_eq!("M107\n", gcode);
/// ```
///
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::fan_off;
///
/// let gcode = fan_off(Some(3));
/// assert_eq!("M107 P3\n", gcode);
/// ```
pub fn fan_off(fan: Option<u8>) -> String {
    let p_str = match fan {
        Some(maybe_fan) => format!(" P{}", maybe_fan),
        None => String::new(),
    };

    format!("M107{p}\n", p = p_str)
}

/// Returns a M140 command to set bed hotend temp as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::set_bed_temp;
///
/// let gcode = set_bed_temp(210);
/// assert_eq!("M140 S210\n", gcode);
/// ```
pub fn set_bed_temp(temp: u8) -> String {
    format!("M140 S{}\n", temp)
}

/// Returns a M190 command to set target bed temp to wait to reach as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::wait_bed_temp;
///
/// let gcode = wait_bed_temp(210);
/// assert_eq!("M190 S210\n", gcode);
/// ```
pub fn wait_bed_temp(temp: u8) -> String {
    format!("M190 S{}\n", temp)
}

/// Returns a M141 command to set target chamber temp as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::set_chamber_temp;
///
/// let gcode = set_chamber_temp(50);
/// assert_eq!("M141 S50\n", gcode);
/// ```
pub fn set_chamber_temp(temp: u8) -> String {
    format!("M141 S{}\n", temp)
}

/// Returns a M191 command to set target chamber temp to wait to reach as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::wait_chamber_temp;
///
/// let gcode = wait_chamber_temp(50);
/// assert_eq!("M191 S50\n", gcode);
/// ```
pub fn wait_chamber_temp(temp: u8) -> String {
    format!("M191 S{}\n", temp)
}

/// Returns a G28 command to trigger autohome procedure, using default parameters set in machine firmware, as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::auto_home;
///
/// let gcode = auto_home();
/// assert_eq!("G28\n", gcode);
/// ```
pub fn auto_home() -> String {
    "G28\n".to_string()
}

/// Returns a M82 command to set the extruder axis to absolute mode, independant of other axes, as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::absolute_extrution;
///
/// let gcode = absolute_extrution();
/// assert_eq!("M82\n", gcode);
pub fn absolute_extrution() -> String {
    "M82\n".to_string()
}

/// Returns a M83 command to set the extruder axis to relative mode, independant of other axes, as a String
///
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::relative_extrution;
///
/// let gcode = relative_extrution();
/// assert_eq!("M83\n", gcode);
pub fn relative_extrution() -> String {
    "M83\n".to_string()
}
