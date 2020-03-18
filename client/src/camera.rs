use base::math::*;
use std::f32::consts;

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Point3f, // PRP
    // VUV will be calculated on the fly
    // used to calculate the look_at_point
    pub theta: f32,
    pub phi: f32,
    pub aspect_ratio: f32,
    proj: Matrix4<f32>,
}

impl Camera {
    /// Create new Camera with given aspect ratio
    pub fn new(aspect_ratio: f32) -> Camera {
        Camera {
            position: Point3::new(15.0, 10.0, 50.0),
            phi: -0.27,
            theta: 2.6,
            aspect_ratio: aspect_ratio,
            proj: perspective(deg(60.0), aspect_ratio, 0.1, 3_000.0),
        }
    }

    /// returns a camera based on a vector to look at
    pub fn new_from_vector(pos: Point3f, look: Vector3f, aspect_ratio: f32) -> Self {
        let mut look_phi = ((look.x) / ((look.x * look.x + look.y * look.y).sqrt())).acos();
        if look.y < 0.0 {
            look_phi = 2.0 * consts::PI - look_phi;
        }

        let look_theta =
            (consts::PI / 2.0) - ((look.z) / ((look.x * look.x + look.y * look.y).sqrt())).atan();

        Camera {
            position: pos,
            phi: look_phi,
            theta: look_theta,
            aspect_ratio: aspect_ratio,
            proj: perspective(deg(60.0), aspect_ratio, 0.1, 3_000.0),
        }
    }

    /// Returns the projection matrix
    pub fn proj_matrix(&self) -> Matrix4<f32> {
        self.proj
    }

    pub fn set_proj_matrix(&mut self, proj: Matrix4<f32>) {
        self.proj = proj;
    }

    /// Returns view matrix
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.position, self.get_look_at_point(), Vector3::unit_z())
    }

    /// Calculates the look_at_point by adding the current position to the
    /// look_at_vector
    pub fn get_look_at_point(&self) -> Point3f {
        self.position + self.get_look_at_vector()
    }

    /// Calculates the look_at_vector by using theta and phi on the unit sphere
    pub fn get_look_at_vector(&self) -> Vector3f {
        Vector3f::new(
            self.theta.sin() * self.phi.cos(),
            self.theta.sin() * self.phi.sin(),
            self.theta.cos(),
        )
    }

    /// Internal function to move the position of the camera

    /// Will be called by the other functions (move_forwars etc)
    pub fn move_by(&mut self, pos_diff: Vector3f) {
        self.position += pos_diff;
    }

    /// Method to call when **forward movement** is needed

    /// `factor` is a factor to scale the movement speed

    /// `factor` has to be positive for foward movement
    pub fn move_forward(&mut self, factor: f32) {
        let mut lookatvector = self.get_look_at_vector();
        lookatvector.z = 0.0;
        lookatvector = lookatvector.normalize();
        lookatvector *= factor;
        self.move_by(lookatvector);
    }

    /// Method to call when **backward movement** is needed

    /// `factor` is a factor to scale the movement speed

    /// `factor` has to be positive for backward movement
    pub fn move_backward(&mut self, factor: f32) {
        let mut lookatvector = self.get_look_at_vector();
        lookatvector.z = 0.0;
        lookatvector = lookatvector.normalize();
        lookatvector *= -factor;
        self.move_by(lookatvector);
    }

    /// Method to call when **left movement** is needed

    /// `factor` is a factor to scale the movement speed

    /// `factor` has to be positive for left movement
    pub fn move_left(&mut self, factor: f32) {
        let mut lookatvector = self.get_look_at_vector();
        lookatvector.z = 0.0;
        lookatvector = lookatvector.normalize();
        // Get the orthogonal 2d-vector, which is 90 degrees to the left
        let mut move_dir = Vector3f::new(-lookatvector.y, lookatvector.x, 0.0);
        move_dir *= factor;
        self.move_by(move_dir);
    }

    /// Method to call when **right movement** is needed

    /// `factor` is a factor to scale the movement speed

    /// `factor` has to be positive for right movement
    pub fn move_right(&mut self, factor: f32) {
        let mut lookatvector = self.get_look_at_vector();
        lookatvector.z = 0.0;
        lookatvector = lookatvector.normalize();
        // Get the orthogonal 2d-vector, which is 90 degrees to the left
        let mut move_dir = Vector3f::new(lookatvector.y, -lookatvector.x, 0.0);
        move_dir *= factor;
        self.move_by(move_dir);
    }

    /// Method to call when **upward movement** is needed

    /// `factor` is a factor to scale the movement speed

    /// `factor` has to be positive for upward movement
    pub fn move_up(&mut self, factor: f32) {
        self.move_by(Vector3f::new(0.0, 0.0, factor));
    }

    /// Method to call when **downward movement** is needed

    /// `factor` is a factor to scale the movement speed

    /// `factor` has to be positive for downward movement
    pub fn move_down(&mut self, factor: f32) {
        self.move_by(Vector3f::new(0.0, 0.0, -factor));
    }

    /// Changes `theta` and `phi` to essentially change the direction the
    /// camera looks
    pub fn change_dir(&mut self, theta_diff: f32, phi_diff: f32) {
        if self.theta < 0.1 {
            if theta_diff > 0.0 {
                // all the way UP, theta will not go any lower
                self.theta += theta_diff;
            }
        } else if self.theta > consts::PI - 0.1 {
            if theta_diff < 0.0 {
                // all the way DOWN, theta will not go any higher
                self.theta += theta_diff;
            }
        } else {
            self.theta += theta_diff;
        }
        self.phi += phi_diff;
    }
}
