#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: glam::Vec3,
    pub to: glam::Vec3,
    pub up: glam::Vec3,

    pub view: glam::Mat4,
    pub proj: glam::Mat4,
}

impl Camera {
    pub fn new(pos: glam::Vec3, to: glam::Vec3, up: glam::Vec3) -> Camera {
        let view = glam::Mat4::look_at_rh(pos, to, up);
        let proj = glam::Mat4::perspective_infinite_rh(3.14/4., 1.5, 0.);

        Camera {
            pos,
            to, 
            up,
            view,
            proj,
        }
    }

    pub fn update_view(&mut self) {
        self.view = glam::Mat4::look_at_rh(self.pos, self.to, self.up);
    }

    pub fn set_perspective(&mut self, fov: f32, aspect: f32) {
        self.proj = glam::Mat4::perspective_infinite_rh(fov, aspect, 0.)
    }

    pub fn dx(&mut self, l: f32, r: f32, t: f32, b: f32, zn: f32, zf: f32) {
        self.proj = glam::mat4(
            glam::vec4(2.*zn/(r-l), 0.,          0.,         0.),
            glam::vec4(0.,          2.*zn/(t-b), 0.,         0.),
            glam::vec4((l+r)/(r-l), (t+b)/(t-b), zf/(zn-zf), -1.),
            glam::vec4(0.,          0., zn*zf/(zn-zf),       0.)
        );
    }
}
