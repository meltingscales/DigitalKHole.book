use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use image::ImageEncoder;

// ============================================================================
// Perlin Noise Favicon Generator
// Inspired by Well of Souls "pi" grain of sand circle estimation
// ============================================================================

/// Permutation table for Perlin noise (randomized on each call)
fn generate_perm_table() -> [u8; 512] {
    let mut perm: [u8; 256] = core::array::from_fn(|i| i as u8);

    // Fisher-Yates shuffle using js_sys::Math::random()
    for i in (1..256).rev() {
        let j = (js_sys::Math::random() * (i + 1) as f64) as usize;
        perm.swap(i, j);
    }

    // Double the permutation table
    let mut result = [0u8; 512];
    for i in 0..512 {
        result[i] = perm[i % 256];
    }
    result
}

/// Fade function for smooth interpolation: 6t^5 - 15t^4 + 10t^3
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Linear interpolation
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

/// Gradient function - returns dot product of gradient and distance vector
fn grad(hash: u8, x: f64, y: f64) -> f64 {
    match hash & 3 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        _ => -x - y,
    }
}

/// 2D Perlin noise
fn perlin_noise(x: f64, y: f64, perm: &[u8; 512]) -> f64 {
    // Find unit grid cell
    let xi = (x.floor() as i32 & 255) as usize;
    let yi = (y.floor() as i32 & 255) as usize;

    // Relative position within cell
    let xf = x - x.floor();
    let yf = y - y.floor();

    // Fade curves
    let u = fade(xf);
    let v = fade(yf);

    // Hash coordinates of the 4 corners
    let aa = perm[perm[xi] as usize + yi] as u8;
    let ab = perm[perm[xi] as usize + yi + 1] as u8;
    let ba = perm[perm[xi + 1] as usize + yi] as u8;
    let bb = perm[perm[xi + 1] as usize + yi + 1] as u8;

    // Blend
    let x1 = lerp(grad(aa, xf, yf), grad(ba, xf - 1.0, yf), u);
    let x2 = lerp(grad(ab, xf, yf - 1.0), grad(bb, xf - 1.0, yf - 1.0), u);

    lerp(x1, x2, v)
}

/// Octave noise (fractal Brownian motion) for more interesting texture
fn octave_noise(x: f64, y: f64, octaves: u32, persistence: f64, perm: &[u8; 512]) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        total += perlin_noise(x * frequency, y * frequency, perm) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }

    total / max_value
}

/// Generate favicon as a Perlin noise "hole" - darker toward center
fn generate_favicon_data_uri() -> String {
    use base64::Engine;

    const SIZE: u32 = 32;
    let perm = generate_perm_table();

    // Random offset for each favicon generation (makes each load unique)
    let offset_x = js_sys::Math::random() * 1000.0;
    let offset_y = js_sys::Math::random() * 1000.0;

    // Noise scale - higher = more zoomed out noise
    let noise_scale = 4.0 + js_sys::Math::random() * 2.0;

    let mut pixels = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    let center = SIZE as f64 / 2.0;
    let max_dist = center; // Maximum distance from center to edge

    for y in 0..SIZE {
        for x in 0..SIZE {
            let px = x as f64;
            let py = y as f64;

            // Distance from center, normalized to 0-1
            let dx = px - center;
            let dy = py - center;
            let dist = (dx * dx + dy * dy).sqrt() / max_dist;

            // Get Perlin noise value (-1 to 1), remap to 0-1
            let noise_x = (px / SIZE as f64) * noise_scale + offset_x;
            let noise_y = (py / SIZE as f64) * noise_scale + offset_y;
            let noise = (octave_noise(noise_x, noise_y, 3, 0.5, &perm) + 1.0) / 2.0;

            // "Hole" effect: darker toward center
            // Invert distance so center = 0 (darkest), edge = 1 (lightest)
            // Apply a curve to make the hole more pronounced
            let radial = dist.powf(0.7); // <1 = sharper edge, >1 = softer edge

            // Combine: base darkness from radial, modulated by noise
            // More noise influence near the edges, less in the dark center
            let noise_influence = 0.3 + radial * 0.5;
            let brightness = radial * (1.0 - noise_influence + noise * noise_influence);

            // Add some "grain" - random speckle like the sand dropping effect
            let grain = if js_sys::Math::random() < 0.1 {
                (js_sys::Math::random() - 0.5) * 0.15
            } else {
                0.0
            };

            let final_brightness = (brightness + grain).clamp(0.0, 1.0);
            let pixel_value = (final_brightness * 255.0) as u8;

            // RGBA - grayscale with full opacity
            pixels.push(pixel_value);
            pixels.push(pixel_value);
            pixels.push(pixel_value);
            pixels.push(255);
        }
    }

    // Encode as PNG
    let mut png_bytes: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    if encoder.write_image(
        &pixels,
        SIZE,
        SIZE,
        image::ExtendedColorType::Rgba8,
    ).is_err() {
        return String::new();
    }

    let b64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    format!("data:image/png;base64,{}", b64)
}

/// Set the favicon dynamically via DOM manipulation
fn set_favicon() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    // Generate the favicon
    let favicon_uri = generate_favicon_data_uri();
    if favicon_uri.is_empty() {
        return;
    }

    // Create or update the link element
    let head = match document.head() {
        Some(h) => h,
        None => return,
    };

    // Remove existing favicon if present
    if let Ok(Some(existing)) = document.query_selector("link[rel='icon']") {
        let _ = existing.remove();
    }

    // Create new favicon link
    let link = match document.create_element("link") {
        Ok(el) => el,
        Err(_) => return,
    };

    let _ = link.set_attribute("rel", "icon");
    let _ = link.set_attribute("type", "image/png");
    let _ = link.set_attribute("href", &favicon_uri);

    let _ = head.append_child(&link);
}

/// Tanka poem with music pairing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tanka {
    pub top_flavor: String,
    pub qr_link: String,
    pub art_link: String,
    #[serde(default)]
    pub bandcamp_embed: Option<String>,
    #[serde(default)]
    pub bandcamp_embed_isprivate: bool,
    pub recommended_music_pairing: MusicPairing,
    pub tanka: TankaVerses,
    pub tankadesc: String,
    #[serde(default)]
    pub tastingnotes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicPairing {
    pub track: String,
    pub artist: String,
    pub album: String,
    pub volume_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TankaVerses {
    #[serde(rename = "1")]
    pub v1: String,
    #[serde(rename = "2")]
    pub v2: String,
    #[serde(rename = "3")]
    pub v3: String,
    #[serde(rename = "4")]
    pub v4: String,
    #[serde(rename = "5")]
    pub v5: String,
}

impl TankaVerses {
    pub fn as_vec(&self) -> Vec<&str> {
        vec![&self.v1, &self.v2, &self.v3, &self.v4, &self.v5]
    }
}

/// A tanka with its slug (URL-safe name)
#[derive(Debug, Clone)]
pub struct TankaEntry {
    pub slug: String,
    pub filename: String,
    pub tanka: Tanka,
}

/// Generate QR code as base64 PNG data URI
fn generate_qr_data_uri(url: &str) -> String {
    use qrcode::QrCode;
    use image::Luma;
    use base64::Engine;

    let code = match QrCode::new(url.as_bytes()) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    let image = code.render::<Luma<u8>>()
        .min_dimensions(128, 128)
        .build();

    let mut png_bytes: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    if encoder.write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        image::ExtendedColorType::L8,
    ).is_err() {
        return String::new();
    }

    let b64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    format!("data:image/png;base64,{}", b64)
}

/// Load tanka from YAML string
fn load_tanka(yaml: &str) -> Option<Tanka> {
    serde_yaml::from_str(yaml).ok()
}

/// Convert filename to URL slug
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace(".tanka.yml", "")
        .replace(".yml", "")
}

// Auto-generated by build.rs - scans content/ for tankas
include!(concat!(env!("OUT_DIR"), "/tankas_generated.rs"));

/// Index page with ls-style listing
#[component]
fn IndexPage() -> impl IntoView {
    let tankas = all_tankas();

    view! {
        <div class="page">
            <div class="terminal">
                <div class="prompt">"$ ls -la content/*.yml"</div>
                <div class="ls-output">
                    <div class="ls-header">"total "{tankas.len()}</div>
                    {tankas.into_iter().map(|entry| {
                        let slug = entry.slug.clone();
                        view! {
                            <a class="ls-row" href={format!("/tanka/{}", slug)}>
                                <span class="ls-perms">"-rw-r--r--"</span>
                                <span class="ls-user">"henry"</span>
                                <span class="ls-date">"2026-01-24"</span>
                                <span class="ls-name">{entry.filename}</span>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

/// Single tanka page component
#[component]
fn TankaPageView() -> impl IntoView {
    let params = use_params_map();
    let tankas = all_tankas();

    let slug = move || params.read().get("slug").unwrap_or_default();

    let current_idx = {
        let s = slug();
        tankas.iter().position(|t| t.slug == s)
    };

    let entry = current_idx.and_then(|i| tankas.get(i).cloned());

    match entry {
        Some(entry) => {
            let prev_slug = current_idx
                .filter(|&i| i > 0)
                .map(|i| tankas[i - 1].slug.clone());
            let next_slug = current_idx
                .filter(|&i| i < tankas.len() - 1)
                .map(|i| tankas[i + 1].slug.clone());

            let tanka = entry.tanka;
            let qr_src = generate_qr_data_uri(&tanka.qr_link);
            let verses = tanka.tanka.as_vec();

            view! {
                <div class="page">
                    <div class="tanka-header">
                        {tanka.top_flavor}
                    </div>

                    <a class="media-row" href={tanka.qr_link.clone()} target="_blank" rel="noopener">
                        <div class="qr-code">
                            <img src={qr_src} alt="Album QR code" />
                        </div>

                        <div class="album-art">
                            <img src={tanka.art_link.clone()} alt="Album art" />
                        </div>

                        <div class="pairing">
                            <span class="track">{tanka.recommended_music_pairing.track.clone()}</span>
                            " by "
                            <span class="artist">{tanka.recommended_music_pairing.artist.clone()}</span>
                            " at "
                            <span class="volume">{tanka.recommended_music_pairing.volume_level.clone()}</span>
                        </div>
                    </a>

                    <div class="bandcamp-player">
                        {if tanka.bandcamp_embed_isprivate {
                            view! {
                                <div class="private-notice">"album is private - visit link to listen"</div>
                            }.into_any()
                        } else if let Some(embed_url) = tanka.bandcamp_embed.clone() {
                            view! {
                                <iframe src={embed_url}></iframe>
                            }.into_any()
                        } else {
                            view! { <></> }.into_any()
                        }}
                    </div>

                    <div class="tanka-body">
                        {verses.into_iter().map(|v| view! {
                            <div class="tanka-verse">{v.to_string()}</div>
                        }).collect_view()}
                    </div>

                    <div class="commentary">
                        <p class="about-tanka">{tanka.tankadesc}</p>
                        {tanka.tastingnotes.map(|notes| view! {
                            <p class="about-song">{notes}</p>
                        })}
                    </div>

                    <nav class="tanka-nav">
                        <div class="nav-prev">
                            {prev_slug.map(|s| view! {
                                <a href={format!("/tanka/{}", s)}>"< prev"</a>
                            })}
                        </div>
                        <div class="nav-index">
                            <a href="/">"[ls]"</a>
                        </div>
                        <div class="nav-next">
                            {next_slug.map(|s| view! {
                                <a href={format!("/tanka/{}", s)}>"next >"</a>
                            })}
                        </div>
                    </nav>
                </div>
            }.into_any()
        }
        None => view! {
            <div class="page">
                <div class="error">"tanka not found"</div>
                <a href="/">"back to index"</a>
            </div>
        }.into_any()
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div>"404"</div> }>
                <Route path=path!("/") view=IndexPage />
                <Route path=path!("/tanka/:slug") view=TankaPageView />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    set_favicon();
    leptos::mount::mount_to_body(App);
}
