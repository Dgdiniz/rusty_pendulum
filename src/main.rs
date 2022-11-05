//Douglas Diniz - Manual do Código

//We are using the speedy2d crate for this example.
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    //We need this window object to create a window.
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    //We need to create a window helper to handle the window events.
    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 400.0),
    };

    //Run the loop.
    window.run_loop(win);
}

//This is the window handler.
//It handles the window events and have the objects that we want to draw and the logic.
struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum,
}

//We need to implement the WindowHandler trait for our window handler.
impl WindowHandler for MyWindowHandler {
    //The draw function is called every frame.
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        //We need to clear the screen every frame.
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        //We need to draw the first pendulum.
        self.p.update();
        self.p.draw(graphics);

        //We need to draw the first pendulum.
        self.p2.update();
        self.p2.draw(graphics);

        //Draw the frame.
        helper.request_redraw();
    }
}

//This is the pendulum struct.
struct Pendulum {
    //This vector is the position of the pendulum.
    origin: vector::Vector,

    //This vector is the position of the ball.
    position: vector::Vector,

    //This is the angle of the pendulum.
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, //The length of the pendulum.
    m: f32, //The mass of the ball.
    g: f32, //The gravity.
}

//We need to implement the pendulum struct.
impl Pendulum {
    //This is the constructor of the pendulum.
    //It takes the x and y position of the pendulum and the length of the pendulum.
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        //We need to return the pendulum.
        Pendulum {
            //We need to set the origin of the pendulum.
            origin: vector::Vector::new(x, y),

            //We'll set the position when we update the pendulum.
            //For now we'll set it to a default value.
            position: vector::Vector::new(0.0, 0.0),

            angle: 1.0,                //We'll set the angle to 1.0 radian.
            angular_velocity: 0.0,     //The pendulum is not moving in the beginning.
            angular_acceleration: 0.0, //The pendulum is not accelerating in the beginning.

            r: r,
            m: 1.0, //The mass of the ball is 1.0 for this example.
            g: 0.5, //The gravity is 0.5 for this example, but play with it.
        }
    }

    //This function updates the pendulum every frame.
    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration.
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus the angular acceleration.
        self.angular_velocity += self.angular_acceleration;

        //The angle is the angle plus the angular velocity.
        self.angle += self.angular_velocity;

        //The posisition is the polar coordinates translated to cartesian coordinates.
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        //The final position of the ball in the canvas is the origin of the
        //pendulum plus the position vector.
        self.position.add(&self.origin);
    }

    //This function draws the pendulum. It takes the graphics object as a parameter.
    fn draw(&self, graphics: &mut Graphics2D) {
        //We need to draw the line of the pendulum first.
        //It takes the start and end position of the line, the width of the line and the color.
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        //We need to draw the ball of the pendulum.
        //It takes the position of the ball, the radius of the ball and the color.
        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

//This is a module that contains the vector struct.
pub mod vector {
    //This is the vector struct that we use for the position of the pendulum and the ball.
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    //The vector implementation.
    impl Vector {
        //The constructor of the vector.
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y } //We return a new vector.
        }

        //This function adds a vector to the current vector.
        //We modify the current vector.
        //Another option would be to return a new vector.
        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        //This function sets the x and y values of the vector.
        pub fn set(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }
    }
}
