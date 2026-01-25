use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use image::ImageEncoder;

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

/// Load tanka from embedded YAML
fn load_tanka(yaml: &str) -> Option<Tanka> {
    serde_yaml::from_str(yaml).ok()
}

/// Single tanka page component
#[component]
fn TankaPage(tanka: Tanka) -> impl IntoView {
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
        </div>
    }
}

// Embed tanka YAML at compile time
const TANKA_YAML: &str = include_str!("../content/target oracle grinder apparatus v2.tanka.yml");

#[component]
fn App() -> impl IntoView {
    let tanka = load_tanka(TANKA_YAML).expect("Failed to parse tanka YAML");

    view! {
        <TankaPage tanka=tanka />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
