use std::fmt::{Display};

use image::RgbaImage;

pub struct Scene {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Box<dyn Light>>,
    pub viewport: Point,
    pub camera: Point
}

impl Scene{
    pub fn render(&self) -> RgbaImage {
        println!("Beginning Render...");
        let mut image = RgbaImage::new(1920, 1080);
        println!("1 / 3 | Generated blank image buffer... ");

        let total = 1920 * 1080;
        let mut num_pixels_completed = 0;
        for y in 0..1080{
            for x in 0..1920{
                let p = image.get_pixel_mut(x, y);

                // Following CGfS, the camera is fixed at 0, 0, 0,
                // and is facing parallel to the xz (horizontal) plane, i.e along (0, 0, 1)
                // positive y is up, positive x is right, positive z is away.

                // this origin is the top left of the viewport, which is therefore
                // centered at (0, 0, 1), with a width of 1.980 units and a height of 1.080 units.
                // I'm thinking of these units as meters.
                let viewport_origin = pt(-1.980/2., 1.080/2., 1.);

                // what world coordinate the 2D screen coordinate maps to
                let point_on_viewport = viewport_origin.add(&pt(1.980 * (x as f32 / 1920.), -1.080 * (y as f32 / 1080.), 0.));
                
                let mut nearest_itsct: Point = pt(999., 999., 999.);
                let mut itsct_normal = pt(0., 0., 0.);
                let mut color: Color = [0., 0., 0., 0.];
                let ray = Ray::new(pt(0., 0., 0.), point_on_viewport.sub(&pt(0., 0., 0.)));
                self.objects.iter().for_each(|sphere| {
                    let hits = sphere.itsct(&ray);
                    for i in 0..hits.len() {
                        let popped = hits.get(i).unwrap();
                        if popped.length_squared() < nearest_itsct.length_squared() {
                            nearest_itsct = popped.clone();
                            itsct_normal = popped.sub(&sphere.center);
                            color = sphere.color;
                        }
                    }
                });

                if nearest_itsct.x != 999.0 {
                    // compute lighting to determine what color to fill the pixel with.
                    // the absense of light means you can't see something,
                    // and having total light means you can see it in its full form,
                    // so totalLight is roughly between 0 and 1, and the color gets
                    // scaled by that value. Nothing stops it from going over 1 or under 0,
                    // which can abnormally brighten or darken objects, which can be useful.
                    // We add together all the light on the object to figure out how
                    // bright we should draw the color
                    let mut total_light: f64 = 0.0;
                    self.lights.iter().for_each(| light | {
                        total_light += light.intensity(itsct_normal,  nearest_itsct) as f64
                    });
                    
                    // convert from (0..1) to (0..255)
                    p.0 = color.map(|e| (e*total_light*255.0) as u8);
                    p.0[3] = 255;
                }else{
                    // background color/pattern
                    p.0 = [0, 0, 0, 255]
                }

                num_pixels_completed += 1;
                
                let progress_pct = num_pixels_completed as f32 / (total as f32) * 100.0;
                if progress_pct == progress_pct.floor(){
                    println!("2 / 3 | Casting rays... {:}%", num_pixels_completed as f32 / (total as f32) * 100.0);
                }
            }
        }
        image
    }
}

pub trait Light{
    fn intensity(&self, normal: Point, pos: Point) -> f32;
}

pub struct AmbientLight{
    pub intensity: f32
}
impl Light for AmbientLight{
    fn intensity(&self, _normal: Point, _pos: Point) -> f32 {
        return self.intensity;
    }
}

pub struct PointLight{
    pub intensity: f32,
    pub position: Point
}
impl Light for PointLight{
    fn intensity(&self, normal: Point, pos: Point) -> f32 {
        let light_vec = self.position.sub(&pos);
        let alignment = normal.dot(&light_vec);
        if alignment < 0. {
            return 0.;
        }
        let intensity = self.intensity*(alignment / light_vec.length_squared().powf(0.5));
        return intensity;
    }
}


pub struct DirectionalLight{
    pub intensity: f32,
    pub direction: Point,
    pub position: Point
}
impl Light for DirectionalLight{
    fn intensity(&self, normal: Point, _pos: Point) -> f32 {
        let light_vec = self.direction.scale(-1.);
        let alignment = normal.dot(&light_vec);
        if alignment < 0. {
            return 0.;
        }
        let intensity = self.intensity*(alignment / light_vec.length_squared().powf(0.5));
        return intensity;
    }
}

pub struct Sphere{
    center: Point,
    radius: f32,
    color: Color
}

type Color = [f64; 4];

#[derive(Clone, Copy, Debug)]
pub struct Point{
    x: f32,
    y: f32,
    z: f32
}
impl Point{
    fn dot(&self, other: &Point) -> f32{
        return self.x*other.x + self.y*other.y + self.z*other.z;
    }

    fn length_squared(&self) -> f32{
        return self.dot(self);
    }

    fn sub(&self, other: &Point) -> Point{
        return pt(self.x - other.x, self.y - other.y, self.z - other.z);
    }
    pub fn add(&self, other: &Point) -> Point{
        return pt(self.x + other.x, self.y + other.y, self.z + other.z);
    }

    fn scale(&self, r: f32) -> Point{
        return pt(r*self.x, r*self.y, r*self.z);
    }
}

impl Display for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({:}, {:}, {:})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
pub struct Ray{
    origin: Point,
    direction: Point,
}

impl Ray{
    fn new(origin: Point, direction: Point) -> Ray {
        return Ray{origin, direction};
    }
}

pub fn pt(x: f32, y: f32, z: f32) -> Point{
    return Point{x, y, z};
}

impl Sphere{
    fn itsct(&self, ray: &Ray) -> Vec<Point> {
        // the a, b, and c, are the coefficients
        // of the quadratic equation. Honestly,
        // I just believed the derivation from
        // https://gabrielgambetta.com/computer-graphics-from-scratch/02-basic-raytracing.html
        let a = ray.direction.length_squared();
        let co = &ray.origin.sub(&self.center);
        let b = 2.0 * co.dot(&ray.direction);
        let c = co.length_squared() - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0*a*c;

        if discriminant > 0.0 {
            let (t1, t2) = ((-b + discriminant) / (2.0*a), (-b - discriminant) / (2.0*a));
            let mut vec: Vec<Point> = vec![];
            if t1 > 0. {
                vec.push(ray.origin.add(&ray.direction.scale(t1)));
            }

            if t2 > 0. {
                vec.push(ray.origin.add(&ray.direction.scale(t2)));
            }

            return vec;
        }else if discriminant == 0.0 {
            let t = -b / (2.0*a);
            if t <= 0. {
                return vec![];
            }
            return vec![ray.origin.add(&ray.direction.scale(t))]
        }
        return vec![];
    }

    pub fn new(center: Point, radius: f32, color: Color) -> Sphere{
        return Sphere{
            center,
            radius,
            color
        }
    }
}