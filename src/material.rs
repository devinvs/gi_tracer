use crate::vector::Vec3;
use crate::geometry::Ray;
use crate::world::World;

const KA: f32 = 0.9;
const KE: f32 = 8.0;

pub struct Color;
impl Color {
    #[allow(non_snake_case)]
    pub fn RGB(r: u8, g: u8, b: u8) -> Vec3<f32> {
        Vec3::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0
        )
    }
}

#[derive(Debug)]
pub struct Light {
    pub pos: Vec3<f32>,
    pub color: Vec3<f32>
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Phong(Vec3<f32>, f32, f32),    // color, kd, ks
    CookTorrance(Vec3<f32>, f32, f32, f32),   // color, f0, roughness, k
}

impl Material {
    pub fn shade(
        &self,
        vin: &Ray,
        dist: f32,
        normal: &Vec3<f32>,
        world: &World
    ) -> Vec3<f32> {
        match self {
            Material::Phong(o_color, kd, ks) => {
                // Ambient
                let mut color = *o_color * KA;

                let v = vin.origin + vin.dir*dist;

                for l in world.lights.iter() {
                    let s = l.pos-v;
                    let s_norm = s.normalized();

                    // Check if light is visible
                    let ray = Ray::new(
                        v,
                        s
                    );

                    // If we collide with something first don't add this light
                    if let Some(p_to_o) = world.intersect(&ray) {
                        let p_to_l = s.mag();
                        if p_to_o.1 < p_to_l {
                            continue;
                        }
                    }

                    // Diffuse Light
                    color += l.color * *o_color * s_norm.dot(normal).max(0.0) * *kd;

                    // Specular Light
                    let r = (v-l.pos).normalized().reflect(normal);
                    let v = -vin.dir;
                    let spec_angle = r.dot(&v).max(0.0);
                    let specular = spec_angle.powf(KE);
                    color += l.color * specular * *ks;
                }

                color
            }
            Material::CookTorrance(o_color, f0, roughness, k) => {
                // Ambient
                let mut color = *o_color * KA;

                let v = vin.origin + vin.dir*dist;

                for l in world.lights.iter() {
                    let s = l.pos-v;
                    let s_norm = s.normalized();

                    let n_dot_l = normal.dot(&s_norm).max(0.0);
                    if n_dot_l <= 0.0 {
                        continue;
                    }

                    // Check if light is visible
                    let ray = Ray::new(
                        v,
                        s
                    );

                    if let Some(p_to_o) = world.intersect(&ray) {
                        let p_to_l = s.mag();
                        if p_to_o.1 < p_to_l {
                            continue;
                        }
                    }

                    let v = -vin.dir;

                    let h = (s_norm+v).normalized();
                    let n_dot_h = normal.dot(&h).max(0.0);
                    let n_dot_v = normal.dot(&v).max(0.0);
                    let v_dot_h = v.dot(&h).max(0.0);

                    if n_dot_v <= 0.0 {continue;}

                    // Fresnel reflectance
                    let f = (1.0-v_dot_h).powi(5) * (1.0 - f0) + f0;

                    // microfacet distribution by beckman
                    let m_squared = roughness * roughness;
                    let r1 = 1.0 / (4.0 * m_squared * n_dot_h.powi(4));
                    let r2 = (n_dot_h * n_dot_h -1.0) / (m_squared * n_dot_h * n_dot_h);
                    let d = r1 * r2.exp();

                    // geometric shadowing
                    let two_n_dot_h = 2.0 * n_dot_h;
                    let g1 = (two_n_dot_h * n_dot_v) / v_dot_h;
                    let g2 = (two_n_dot_h * n_dot_l) / v_dot_h;
                    let g = 1.0f32.min(g1).min(g2);

                    let ks = (f*d*g) / (4.0*n_dot_v*n_dot_l);

                    // Diffuse Lighting
                    color += *o_color * l.color * n_dot_l;

                    // Specular Highlight
                    color += l.color * n_dot_l * (k + ks * (1.0-k));
                }

                color
            }
        }
    }
}
