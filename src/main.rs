use std::f32::consts::PI;
use std::{thread, time};

fn main() {
    let mut a: f32 = 0.0; // rotation on the x-axis I think
    let mut b: f32 = 0.0; // and this one's for z-axis rotation

    let width = 80;
    let height = 40;

    // clear the terminal at startup
    print!("\x1b[2J");

    loop {
        // screen buffer and depth tracking
        let mut output = vec![' '; width * height];
        let mut zbuffer = vec![0.0; width * height]; // renamed from z_buffer for consistency

        // theta goes around the tube, phi spins the whole thing
        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let mut phi = 0.0;
            while phi < 2.0 * PI {
                // precalculating trig values - saves time in the loop
                let sin_theta = theta.sin();
                let cos_theta = theta.cos();
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();
                let sin_a = a.sin();
                let cos_a = a.cos();
                let sin_b = b.sin();
                let cos_b = b.cos();

                // Torus shape parameters
                // TODO: maybe make these configurable later?
                let r1 = 1.0; // tube radius
                let r2 = 2.0; // donut radius
                let circle_x = r2 + r1 * cos_theta;
                let circle_y = r1 * sin_theta;

                // 3D rotation calculations (honestly still not 100% sure about all this math)
                let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi)
                    - circle_y * cos_a * sin_b;
                let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi)
                    + circle_y * cos_a * cos_b;
                let z = 5.0 + cos_a * circle_x * sin_phi + circle_y * sin_a;

                let ooz = 1.0 / z; // perspective division thing

                // project to screen coordinates
                let k1 = 35.0; // chosen by trial and error, looks good
                let xp = (width as f32 / 2.0 + k1 * ooz * x) as usize;
                let yp = (height as f32 / 2.0 - k1 * ooz * y / 2.0) as usize; // divide by 2 because terminal chars aren't square

                // lighting calculation - dot product with light direction
                let luminance =
                    cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                        + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

                // only draw if facing towards us (positive luminance)
                if luminance > 0.0 {
                    let idx = xp + yp * width;
                    if idx < width * height && ooz > zbuffer[idx] {
                        zbuffer[idx] = ooz;
                        // characters from darkest to brightest
                        let chars = ".,-~:;=!*#$@";
                        let char_idx = (luminance * 8.0) as usize;
                        output[idx] = chars.chars().nth(char_idx).unwrap_or('.');
                    }
                }

                phi += 0.02; // step size for phi
            }
            theta += 0.07; // step size for theta
        }

        // move cursor back to top and render
        print!("\x1b[H");
        for j in 0..height {
            for i in 0..width {
                print!("{}", output[i + j * width]);
            }
            println!();
        }

        // increment rotation angles
        a += 0.04;
        b += 0.02;

        // sleep to control frame rate
        thread::sleep(time::Duration::from_millis(30));
    }
}
