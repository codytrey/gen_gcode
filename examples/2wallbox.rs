// examples/2wallbox.rs
extern crate gen_gcode;

use gen_gcode::*;
use gen_gcode::consts::Constants;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let print_area_x = 220;
    let print_area_y = 220;
    let print_area_z = 250;
    let nozzle_temp = 210;
    let bed_temp = 80;
    let nozzle_size = 0.4;
    let layer_hight = 0.2;
    let boxlength = 40.0;
    let wall_thickness = 0.8;
    let num_walls = &wall_thickness/&nozzle_size;
    let bottom_thickness = 0.8;
    let num_bottom_layers = &bottom_thickness/&layer_hight;
    let top_thickness = 0.4;
    let num_top_layers = &top_thickness/&layer_hight;
    let init_layer_hight = 0.17;
    let extrude_per_travel = 0.024;

    let move_feed_rate = Some(3000);
    let print_feed_rate = Some(300);

    let mut file = File::create("foo.gcode").unwrap();
    file.write(wait_bed_temp(bed_temp).as_bytes()).expect("could not write to file");
    file.write(wait_hotend_temp(nozzle_temp, None).as_bytes()).expect("could not write to file");
    file.write(Constants::AbsoluteExtrution.as_str().as_bytes()).expect("could not write to file");
    file.write(Constants::AutoHome.as_str().as_bytes()).expect("could not write to file");
    file.write(reset_extruder(0.0).as_bytes()).expect("could not write to file");
    file.write(move_xyz(Point3d { x: 0.0, y: 0.0, z: 2.0 }, move_feed_rate, None).as_bytes()).expect("could not write to file");

    let layers_z = gen_layer_heights(init_layer_hight, boxlength, layer_hight);

    for l in layers_z {
        let start_point: Point3d = calc_start_point(print_area_x, print_area_y, boxlength, boxlength, l);
        file.write(move_xyz(start_point, move_feed_rate, None).as_bytes()).expect("could not write to file");
        let permim_points = gen_2_perimiters(start_point, nozzle_size, num_walls, boxlength, boxlength);
        let mut e_dest = 0.0;
        for p in permim_points {
            e_dest += boxlength * extrude_per_travel;
            file.write(move_xyz(p, print_feed_rate, Some(e_dest)).as_bytes()).expect("could not write to file");
        }
    }
    // let start_point: Point3d = calc_start_point(print_area_x, print_area_y, boxlength, boxlength, init_layer_hight);
    // file.write(move_xyz(start_point, move_feed_rate, None).as_bytes());
    // let layers_z = gen_layer_heights(init_layer_hight, boxlength, layer_hight);
    // let permim_points = gen_2_perimiters(start_point, nozzle_size, num_walls, boxlength, boxlength);
    // let mut e_dest = 0.0;
    // for p in permim_points {
    //     e_dest += boxlength * extrude_per_travel;
    //     file.write(move_xyz(p, print_feed_rate, Some(e_dest)).as_bytes());
    // }

}

fn calc_start_point(bed_x: u8, bed_y: u8, print_x: f32, print_y: f32, first_layer_z: f32) -> Point3d {
    let init_x = (bed_x as f32/2.0) - (print_x/2.0);
    let init_y = (bed_y as f32/2.0) - (print_y/2.0);
    return Point3d { x: init_x as f32, y: init_y as f32, z: first_layer_z}
}

fn gen_2_perimiters(start_point: Point3d, nozzle_size: f32, num_walls: f32, x_dim: f32, y_dim: f32) -> Vec<Point3d> {
    let mut points: Vec<Point3d> = Vec::new();
    
    let mut tmp_point = Point3d { x: start_point.x, y: start_point.y, z: start_point.z };
    // points.push(tmp_point);
    for n in 0..num_walls as u8 {
        let offset = n as f32 * nozzle_size;
        println!("Offset: {:#?}", offset);
        println!("x-left: {:#?}", start_point.x+offset);
        println!("x-right: {:#?}", start_point.x+x_dim-offset);
        println!("y-top: {:#?}", start_point.y+y_dim-offset);
        println!("y-bottem: {:#?}", start_point.y+offset);
        tmp_point = Point3d { x: tmp_point.x, y: start_point.y+y_dim-offset, z: tmp_point.z };
        points.push(tmp_point);
        tmp_point = Point3d { x: start_point.x+x_dim-offset, y: tmp_point.y, z: tmp_point.z };
        points.push(tmp_point);
        tmp_point = Point3d { x: tmp_point.x, y: start_point.y+offset, z: tmp_point.z };
        points.push(tmp_point);
        tmp_point = Point3d { x: start_point.x+offset, y: tmp_point.y, z: tmp_point.z };
        points.push(tmp_point);
    }

    return points
}

fn gen_layer_heights(first_layer_z: f32, last_layer_z: f32, z_height: f32) -> Vec<f32> {
    let mut z_heights: Vec<f32> = Vec::new();

    let first_layer_z_micron = (first_layer_z * 1000.0) as u32;
    let last_layer_z_micron = (last_layer_z * 1000.0) as u32;
    let z_height_micron = (z_height * 1000.0) as usize;
    for x in (first_layer_z_micron..last_layer_z_micron).step_by(z_height_micron) {
        z_heights.push(x as f32 / 1000.0);
    }
    return z_heights
}

fn layer_change(cur_layer_end: Point3d, new_layer_start: Point3d, layer_hight: f32) -> Vec<String> {
    let mut layer_change_gcode: Vec<String> = Vec::new();

    layer_change_gcode.push(move_z(cur_layer_end.z + layer_hight * 1.25));
    layer_change_gcode.push(move_xy(Point2d { x: new_layer_start.x, y: new_layer_start.y }, None, None));
    layer_change_gcode.push(move_z(new_layer_start.z));

    return layer_change_gcode
}
