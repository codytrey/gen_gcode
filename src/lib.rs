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
        let gcode = move_xy(p, false, None, None);
        assert_eq!("G0 X10 Y5\n", gcode);
    }

    #[test]
    fn test_arc_ij() {
        let p = Point2d { x: 125.0, y: 0.0 };
        let gcode = move_xy_arc_ij(Some(p), Some(62.5), None, None, false);
        assert_eq!("G2 X125 Y0 I62.5\n", gcode);
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
/// let gcode = move_xy(p, false, None, None);
/// assert_eq!("G0 X10 Y5", gcode);
/// ```
/// 
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy};
/// 
/// let p = Point2d { x: 10.0, y: 5.0 };
/// // move with extrude
/// let gcode = move_xy(p, true, None, None);
/// assert_eq!("G1 X10 Y5", gcode);
/// ```
/// 
pub fn move_xy(dest:Point2d, extrude: bool, feed_rate: Option<u32>, flow_rate: Option<f32>) -> String {
    let f_str: String;
    let e_str: String;
    if let Some(maybe_feed_rate) = feed_rate {
        f_str = format!(" F{}", maybe_feed_rate);
    } else {
        f_str = format!("");
    }
    
    if extrude {
        if let Some(maybe_flow_rate) = flow_rate {
            e_str = format!(" E{}", maybe_flow_rate);
        } else {
            e_str = format!("");
        }
        return format!("G1 X{x} Y{y}{e}{f}\n", x=dest.x, y=dest.y, e=e_str, f=f_str)
    } else {
        return format!("G0 X{x} Y{y}{f}\n", x=dest.x, y=dest.y, f=f_str)
    }

}

pub fn move_xyz(dest:Point3d, extrude: bool, feed_rate: Option<u32>, flow_rate: Option<f32>) -> String {
    let f_str: String;
    let e_str: String;
    if let Some(maybe_feed_rate) = feed_rate {
        f_str = format!(" F{}", maybe_feed_rate);
    } else {
        f_str = format!("");
    }
    
    if extrude {
        if let Some(maybe_flow_rate) = flow_rate {
            e_str = format!(" E{}", maybe_flow_rate);
        } else {
            e_str = format!("");
        }
        return format!("G1 X{x} Y{y} Z{z}{e}{f}\n", x=dest.x, y=dest.y, z=dest.z, e=e_str, f=f_str)
    } else {
        return format!("G0 X{x} Y{y} Z{z}{f}\n", x=dest.x, y=dest.y, z=dest.z, f=f_str)
    }

}

pub fn move_z(z: f32) -> String {
    return format!("G0 Z{}", z)
}

/// Returns a G2 or G3 command as a String
/// 
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy};
/// 
/// let p = Point2d { x: 125.0, y: 0.0 };
/// // Create a Clockwise 180 degree Arc starting at 0,0 ending at 125,0 with center point 62.5,0
/// let gcode = move_xy_arc_ij(Some(p), Some(62.5), None, None, false);
/// assert_eq!("G2 X125 Y0 I62.5", gcode);
/// ```
pub fn move_xy_arc_ij(dest: Option<Point2d>, x_offset: Option<f32>, y_offset: Option<f32>, flow_rate: Option<f32>, ccw: bool) -> String {
    let x_str: String;
    let y_str: String;
    let i_str: String;
    let j_str: String;
    let e_str: String;
    if let Some(maybe_dest) = dest {
        x_str = format!(" X{}", maybe_dest.x);
        y_str = format!(" Y{}", maybe_dest.y);
    } else {
        x_str = format!("");
        y_str = format!("");
    }
    if let Some(maybe_x_offset) = x_offset {
        i_str = format!(" I{}", maybe_x_offset);
    } else {
        i_str = format!("");
    }
    if let Some(maybe_y_offset) = y_offset {
        j_str = format!(" J{}", maybe_y_offset);
    } else {
        j_str = format!("");
    }
    if let Some(maybe_flow_rate) = flow_rate {
        e_str = format!(" E{}", maybe_flow_rate);
    } else {
        e_str = format!("");
    }
    if ccw {
        return format!("G3{x}{y}{i}{j}{e}\n", i=i_str, j=j_str, x=x_str, y=y_str, e=e_str);
    } else {
        return format!("G2{x}{y}{i}{j}{e}\n", i=i_str, j=j_str, x=x_str, y=y_str, e=e_str);
    }
}

/// Returns a G21 command as a String
///
/// Sets units to millimeters
pub fn use_millimeters() -> String {
    return format!("G21 ; set units to millimeters\n")
}

/// Returns a G20 command as a String
/// 
/// Sets units to inches
pub fn use_inches() -> String {
    return format!("G20 ; set units to inches\n")
}

/// Returns a G90 command as a String
/// 
/// sets all axes to absolute positioning (relative to home, ie. (0,0))
pub fn absolute_positioning() -> String {
    return format!("G90 ; Set all axes to absolute\n")
}

/// Returns a G91 command as a String
/// 
/// sets all axes to relative positioning (relative to nozzle/tool position)
pub fn relative_positioning() -> String {
    return format!("G91 ; Set all axes to relative\n")
}

/// Returns a G92 command to set the current nozzle/tool possition in the XY plane as a String
/// 
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy};
/// 
/// let p = Point2d { x: 125.0, y: 125.0 };
/// let gcode = set_pos_2d(p, None);
/// assert_eq!("G92 X125 Y125", gcode);
/// ```
pub fn set_pos_2d(pos: Point2d, extrude_pos: Option<f32>) -> String {
    let e_str: String;
    if let Some(maybe_extrude_pos) = extrude_pos {
        e_str = format!(" E{}", maybe_extrude_pos);
    } else {
        e_str = format!("");
    }
    return format!("G92 X{x} Y{y}{e}\n", x=pos.x, y=pos.y, e=e_str)
}

/// Returns a G92 command to set the current nozzle/tool possition in 3 dimentions (XYZ) as a String
/// 
/// # Examples
/// ```
/// extern crate gen_gcode;
/// use gen_gcode::{Point2d, move_xy};
/// 
/// let p = Point3d { x: 125.0, y: 125.0, z: 25.0};
/// let gcode = set_pos_3d(p, None);
/// assert_eq!("G92 X125 Y125 Z25", gcode);
/// ```
pub fn set_pos_3d(pos: Point3d, extrude_pos: Option<f32>) -> String {
    let e_str: String;
    if let Some(maybe_extrude_pos) = extrude_pos {
        e_str = format!(" E{}", maybe_extrude_pos);
    } else {
        e_str = format!("");
    }
    return format!("G92 X{x} Y{y} Z{z}{e}\n", x=pos.x, y=pos.y, z=pos.z, e=e_str)
}

pub fn reset_extruder(extrude_pos: f32) -> String {
    return format!("G92 E{}\n", extrude_pos)
}

/// Returns a G92.1 command to reset to machine's native possitioning offsets as a String
pub fn reset_pos() -> String {
    return format!("G92.1\n")
}

pub fn set_hotend_temp(temp: u16, hotend: Option<u8>) -> String {
    let t_str: String;
    if let Some(maybe_hotend) = hotend {
        t_str = format!(" T{}", maybe_hotend);
    } else {
        t_str = format!("");
    }
    return format!("M104 S{s}{t}\n", s=temp, t=t_str)
}

pub fn wait_hotend_temp(temp: u16, hotend: Option<u8>) -> String {
    let t_str: String;
    if let Some(maybe_hotend) = hotend {
        t_str = format!(" T{}", maybe_hotend);
    } else {
        t_str = format!("");
    }
    return format!("M109 S{s}{t}\n", s=temp, t=t_str)
}

pub fn set_fan_speed(speed: u8, fan: Option<u8>) -> String {
    let p_str: String;
    if let Some(maybe_fan) = fan {
        p_str = format!(" P{}", maybe_fan);
    } else {
        p_str = format!("");
    }
    return format!("M106 S{s}{p}\n", s=speed, p=p_str)
}

pub fn fan_off(fan: Option<u8>) -> String {
    let p_str: String;
    if let Some(maybe_fan) = fan {
        p_str = format!(" P{}", maybe_fan);
    } else {
        p_str = format!("");
    }
    return format!("M107{p}\n", p=p_str)
}

pub fn set_bed_temp(temp: u8) -> String {
    return format!("M140 S{}\n", temp)
}

pub fn wait_bed_temp(temp: u8) -> String {
    return format!("M190 S{}\n", temp)
}

pub fn set_chamber_temp(temp: u8) -> String {
    return format!("M141 S{}\n", temp)
}

pub fn wait_chamber_temp(temp: u8) -> String {
    return format!("M191 S{}\n", temp)
}

pub fn auto_home() -> String {
    return format!("G28\n")
}

pub fn absolute_extrution() -> String {
    return format!("M82")
}

pub fn relative_extrution() -> String {
    return format!("M83")
}