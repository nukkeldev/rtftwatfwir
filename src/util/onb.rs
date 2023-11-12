use glam::Vec3A;

#[derive(Debug, Copy, Clone)]
pub struct ONB {
    axis: [Vec3A; 3],
}

impl ONB {
    pub fn new_from_w(n: &Vec3A) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 { Vec3A::Y } else { Vec3A::X };
        let v = w.cross(a).normalize();
        let u = w.cross(v);
        Self { axis: [u, v, w] }
    }

    pub fn local(&self, a: Vec3A) -> Vec3A {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }

    pub fn u(&self) -> Vec3A {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3A {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3A {
        self.axis[2]
    }
}
