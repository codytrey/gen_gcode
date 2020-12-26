use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_home_x_bl() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::BottomLeft};
        let p = m.home_point();
        assert_eq!(p.x, 0.0)
    }

    #[test]
    fn test_machine_home_y_bl() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::BottomLeft};
        let p = m.home_point();
        assert_eq!(p.y, 0.0)
    }

    #[test]
    fn test_machine_home_x_tl() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::TopLeft};
        let p = m.home_point();
        assert_eq!(p.x, 0.0)
    }

    #[test]
    fn test_machine_home_y_tl() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::TopLeft};
        let p = m.home_point();
        assert_eq!(p.y, 220.0)
    }

    #[test]
    fn test_machine_home_x_br() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::BottomRight};
        let p = m.home_point();
        assert_eq!(p.x, 220.0)
    }

    #[test]
    fn test_machine_home_y_br() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::BottomRight};
        let p = m.home_point();
        assert_eq!(p.y, 0.0)
    }

    #[test]
    fn test_machine_home_x_tr() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::TopRight};
        let p = m.home_point();
        assert_eq!(p.x, 220.0)
    }

    #[test]
    fn test_machine_home_y_tr() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::TopRight};
        let p = m.home_point();
        assert_eq!(p.y, 220.0)
    }

    #[test]
    fn test_machine_home_x_c() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::Center};
        let p = m.home_point();
        assert_eq!(p.x, 110.0)
    }

    #[test]
    fn test_machine_home_y_c() {
        let m = Machine {bed_x: 220.0, bed_y: 220.0, home_pos: HomePos::Center};
        let p = m.home_point();
        assert_eq!(p.y, 110.0)
    }

}

#[derive(Debug)]
pub struct Machine {
    bed_x: f32,
    bed_y: f32,
    home_pos: HomePos,
}

#[derive(Debug)]
pub enum HomePos {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
    Center,
}

trait HomePoint {
    fn home_point(&self) -> Point2d;
}

impl HomePoint for Machine {
    fn home_point(&self) -> Point2d {
        match self.home_pos {
            HomePos::BottomLeft => Point2d { x: 0.0, y: 0.0 },
            HomePos::BottomRight => Point2d { x: self.bed_x, y: 0.0 },
            HomePos::TopLeft => Point2d { x: 0.0, y: self.bed_y },
            HomePos::TopRight => Point2d { x: self.bed_x, y: self.bed_y },
            HomePos::Center => Point2d { x: self.bed_x/2., y: self.bed_y/2. },
        }
    }
}
