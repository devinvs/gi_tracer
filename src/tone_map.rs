use crate::vector::Vec3;

const LMAX: f32 = 100.0;
const LDMAX: f32 = 500.0; // nits
const WARD_DELTA: f32 = 0.001;

pub enum Algorithm {
    Ward,
    Reinhard,
    ALM(f32)    // bias
}

pub fn tone_map(img: &mut Vec<Vec3<f32>>, algo: Algorithm) {
    // convert [0..1] pixel values to luminances by multiplying by ldmax
    img.iter_mut()
        .for_each(|v| {
            *v = *v * LMAX;
        });

    // Calculate absolute luminances
    let luminances = calc_luminance(&img);

    // Run compression algorithm
    match algo {
        Algorithm::Ward => ward(img, luminances),
        Algorithm::Reinhard => reinhard(img, luminances),
        Algorithm::ALM(b) => adaptive_logarithmic_mapping(img, luminances, b),
    }
}

fn calc_luminance(img: &Vec<Vec3<f32>>) -> Vec<f32> {
    img.iter()
        .map(|v| 0.27*v.x+0.67*v.y+0.06*v.z)
        .collect()
}

fn log_avg_luminance(lum: &Vec<f32>) -> f32 {
    let mut log_avg = lum.iter()
        .map(|l| l+WARD_DELTA)
        .map(|l| l.ln())
        .sum::<f32>();

    log_avg /= lum.len() as f32;
    log_avg = log_avg.exp();

    log_avg
}

fn ward(img: &mut Vec<Vec3<f32>>, lum: Vec<f32>) {
    let la = log_avg_luminance(&lum);

    let mut sf = 1.219 + (LDMAX/2.0).powf(0.4);
    sf /= 1.219 + la.powf(0.4);
    sf = sf.powf(2.5);
    sf /= LDMAX;

    img.iter_mut()
        .for_each(|v| *v = *v * sf);
}

fn reinhard(img: &mut Vec<Vec3<f32>>, lum: Vec<f32>) {
    let la = log_avg_luminance(&lum);
    let alpha = 0.18;

    // scale luminance by mapping key value to zone V
    img.iter_mut()
        .for_each(|v| {
            *v = *v * alpha / la;
        });

    // calculate target display luminance
    img.iter_mut()
        .for_each(|v| {
            *v = *v / (*v + 1.0);
        });
}

fn bias(b: f32, t: f32) -> f32 {
    t.powf(b.ln() / 0.5_f32.ln())
}

fn adaptive_logarithmic_mapping(img: &mut Vec<Vec3<f32>>, lum: Vec<f32>, b: f32) {
    // l_wmax is the max luminance value in the scene
    // l_w is the luminance of the pixel
    // l_w and l_wmax are booth divided by l_wa for adaptive scaling
    // l_wa is the average world luminance
    // l_d is the output luminance for a single pixel

    let l_wa = log_avg_luminance(&lum);
    ///let l_wmax = lum.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() / l_wa;
    let l_wmax = LMAX;

    let luminance = |l_w: f32| {
        let l_w = l_w / l_wa;

        let mut l_d = 1.0;
        l_d *= (l_w+1.0).log2();
        l_d /= (l_wmax+1.0).log10();

        let b = bias(b, l_w / l_wmax);
        l_d /= (2.0 + b * 0.8).log2();
        l_d
    };

    img.iter_mut()
        .for_each(|v| {
            v.x = luminance(v.x);
            v.y = luminance(v.y);
            v.z = luminance(v.z);
        });
}


