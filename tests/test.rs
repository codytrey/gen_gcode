use gen_gcode::*;

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
fn test_move_z() {
    let gcode = move_z(1.8);
    assert_eq!("G0 Z1.8\n", gcode);
}

mod test_consts {
    use gen_gcode::consts::Constants;

    #[test]
    fn test_auto_home() {
        let gcode = Constants::AutoHome.as_str_end();
        assert_eq!("G28\n", gcode);
    }

    #[test]
    fn test_absolute_extrution() {
        let gcode = Constants::AbsoluteExtrution.as_str_end();
        assert_eq!("M82\n", gcode);
    }

    #[test]
    fn test_relative_extrution() {
        let gcode = Constants::RelativeExtrution.as_str_end();
        assert_eq!("M83\n", gcode);
    }

    #[test]
    fn test_relative_positioning() {
        let gcode = Constants::RelativePos.as_str_end();
        assert_eq!("G91\n", gcode);
    }

    #[test]
    fn test_absolute_positioning() {
        let gcode = Constants::AbsolutePos.as_str_end();
        assert_eq!("G90\n", gcode);
    }

    #[test]
    fn test_use_inches() {
        let gcode = Constants::UseInches.as_str_end();
        assert_eq!("G20\n", gcode);
    }

    #[test]
    fn test_use_millimeters() {
        let gcode = Constants::UseMillimeters.as_str_end();
        assert_eq!("G21\n", gcode);
    }

    #[test]
    fn test_rest_pos() {
        let gcode = Constants::ResetPos.as_str_end();
        assert_eq!("G92.1\n", gcode);
    }
}

mod test_consts_no_end {
    use gen_gcode::consts::Constants;

    #[test]
    fn test_auto_home() {
        let gcode = Constants::AutoHome.as_str();
        assert_eq!("G28", gcode);
    }

    #[test]
    fn test_absolute_extrution() {
        let gcode = Constants::AbsoluteExtrution.as_str();
        assert_eq!("M82", gcode);
    }

    #[test]
    fn test_relative_extrution() {
        let gcode = Constants::RelativeExtrution.as_str();
        assert_eq!("M83", gcode);
    }

    #[test]
    fn test_relative_positioning() {
        let gcode = Constants::RelativePos.as_str();
        assert_eq!("G91", gcode);
    }

    #[test]
    fn test_absolute_positioning() {
        let gcode = Constants::AbsolutePos.as_str();
        assert_eq!("G90", gcode);
    }

    #[test]
    fn test_use_inches() {
        let gcode = Constants::UseInches.as_str();
        assert_eq!("G20", gcode);
    }

    #[test]
    fn test_use_millimeters() {
        let gcode = Constants::UseMillimeters.as_str();
        assert_eq!("G21", gcode);
    }

    #[test]
    fn test_rest_pos() {
        let gcode = Constants::ResetPos.as_str();
        assert_eq!("G92.1", gcode);
    }
}
