use gilrs::{Axis, Button, EventType, Gilrs};
use macroquad::prelude::*;
use std::collections::{hash_map::Entry, HashMap};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const MAX_PROFILES: usize = 4;
const AVATAR_COUNT: usize = 20;
const PROFILE_IDS: [&str; MAX_PROFILES] = ["default", "profile-1", "profile-2", "profile-3"];
const KEYBOARD: [&str; 43] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
    "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5",
    "6", "7", "8", "9", "SPACE", "-", "_", "BACK", "CLEAR", "DONE", "CANCEL",
];

fn window_conf() -> Conf {
    Conf {
        window_title: "PlayFusion Cart Packs".into(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[derive(Default)]
struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    accept: bool,
    back: bool,
    rename: bool,
    avatar: bool,
    add: bool,
    delete: bool,
    stick_neutral: bool,
}

impl Input {
    fn poll(&mut self, gilrs: &mut Option<Gilrs>) {
        self.up = is_key_pressed(KeyCode::Up);
        self.down = is_key_pressed(KeyCode::Down);
        self.left = is_key_pressed(KeyCode::Left);
        self.right = is_key_pressed(KeyCode::Right);
        self.accept = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space);
        self.back = is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Backspace);
        self.rename = is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::F2);
        self.avatar = is_key_pressed(KeyCode::Y) || is_key_pressed(KeyCode::Tab);
        self.add = is_key_pressed(KeyCode::Insert);
        self.delete = is_key_pressed(KeyCode::Delete);

        let Some(gilrs) = gilrs.as_mut() else { return };
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::ButtonPressed(Button::DPadUp, _) => self.up = true,
                EventType::ButtonPressed(Button::DPadDown, _) => self.down = true,
                EventType::ButtonPressed(Button::DPadLeft, _) => self.left = true,
                EventType::ButtonPressed(Button::DPadRight, _) => self.right = true,
                EventType::ButtonPressed(Button::South, _) => self.accept = true,
                EventType::ButtonPressed(Button::East, _) => self.back = true,
                EventType::ButtonPressed(Button::West, _) => self.rename = true,
                EventType::ButtonPressed(Button::North, _) => self.avatar = true,
                EventType::ButtonPressed(Button::Start, _) => self.add = true,
                EventType::ButtonPressed(Button::Select, _) => self.delete = true,
                _ => {}
            }
        }

        let mut active = false;
        for (_, pad) in gilrs.gamepads() {
            let x = pad.value(Axis::LeftStickX);
            let y = pad.value(Axis::LeftStickY);
            if x.abs() < 0.55 && y.abs() < 0.55 {
                continue;
            }
            active = true;
            if self.stick_neutral {
                if y.abs() > x.abs() {
                    if y > 0.55 { self.up = true; }
                    if y < -0.55 { self.down = true; }
                } else {
                    if x < -0.55 { self.left = true; }
                    if x > 0.55 { self.right = true; }
                }
            }
            break;
        }
        self.stick_neutral = !active;
    }
}

#[derive(Clone)]
struct Profile {
    id: String,
    name: String,
    avatar: usize,
}

fn data_root() -> PathBuf {
    if let Some(path) = env::var_os("PLAYFUSION_CART_DATA") {
        return PathBuf::from(path);
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/var/kazeta"))
        .join(".local/share/playfusion-cart-packs")
}

fn profile_path(id: &str) -> PathBuf {
    data_root().join("profiles").join(format!("{id}.conf"))
}

fn clean_name(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_'))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(20)
        .collect()
}

fn load_profile(id: &str, slot: usize) -> Option<Profile> {
    let content = fs::read_to_string(profile_path(id)).ok()?;
    let mut name = None;
    let mut avatar = slot + 1;
    for line in content.lines() {
        if let Some(value) = line.strip_prefix("name=") {
            let value = clean_name(value);
            if !value.is_empty() { name = Some(value); }
        }
        if let Some(value) = line.strip_prefix("avatar=") {
            avatar = value.parse::<usize>().unwrap_or(slot + 1).clamp(1, AVATAR_COUNT);
        }
    }
    Some(Profile {
        id: id.into(),
        name: name.unwrap_or_else(|| if slot == 0 { "Default Profile".into() } else { format!("Profile {slot}") }),
        avatar,
    })
}

fn save_profile(profile: &Profile) -> io::Result<()> {
    let path = profile_path(&profile.id);
    fs::create_dir_all(path.parent().unwrap())?;
    let tmp = path.with_extension(format!("conf.tmp.{}", std::process::id()));
    fs::write(&tmp, format!("name={}\navatar={}\n", clean_name(&profile.name), profile.avatar))?;
    fs::rename(tmp, path)
}

fn load_profiles() -> Vec<Profile> {
    let mut profiles = Vec::new();
    for (slot, id) in PROFILE_IDS.iter().enumerate() {
        if let Some(profile) = load_profile(id, slot) {
            profiles.push(profile);
        }
    }
    if profiles.is_empty() {
        let profile = Profile { id: "default".into(), name: "Default Profile".into(), avatar: 1 };
        let _ = save_profile(&profile);
        profiles.push(profile);
    }
    profiles
}

fn active_profile() -> String {
    fs::read_to_string(data_root().join("active-profile"))
        .unwrap_or_else(|_| "default".into())
        .trim()
        .to_string()
}

fn set_active_profile(id: &str) -> io::Result<()> {
    if !PROFILE_IDS.contains(&id) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid profile ID"));
    }
    fs::create_dir_all(data_root())?;
    let path = data_root().join("active-profile");
    let tmp = data_root().join(format!("active-profile.tmp.{}", std::process::id()));
    fs::write(&tmp, format!("{id}\n"))?;
    fs::rename(tmp, path)
}

async fn load_avatar(number: usize) -> Option<Texture2D> {
    let root = env::var_os("PLAYFUSION_CART_ASSETS")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/usr/share/playfusion-cart-packs"));
    load_texture(root.join("avatars").join(format!("avatar-{number:02}.png")).to_str()?)
        .await
        .ok()
}

fn panel(x: f32, y: f32, w: f32, h: f32, selected: bool) {
    let fill = if selected { Color::new(0.18, 0.04, 0.27, 0.96) } else { Color::new(0.025, 0.025, 0.08, 0.94) };
    draw_rectangle(x, y, w, h, fill);
    let border = if selected { Color::new(0.05, 0.9, 1.0, 1.0) } else { Color::new(0.75, 0.1, 0.9, 0.7) };
    draw_rectangle_lines(x, y, w, h, if selected { 5.0 } else { 2.0 }, border);
    if selected {
        draw_rectangle_lines(x + 5.0, y + 5.0, w - 10.0, h - 10.0, 2.0, Color::new(1.0, 0.1, 0.65, 0.9));
    }
}

fn centered(text: &str, y: f32, size: f32, color: Color) {
    let width = measure_text(text, None, size as u16, 1.0).width;
    draw_text(text, (screen_width() - width) / 2.0, y, size, color);
}

async fn profile_ui() {
    let mut gilrs = Gilrs::new().ok();
    let mut input = Input { stick_neutral: true, ..Default::default() };
    let mut profiles = load_profiles();
    let active = active_profile();
    let mut selected = profiles.iter().position(|p| p.id == active).unwrap_or(0);
    let mut avatars: HashMap<usize, Texture2D> = HashMap::new();
    let mut keyboard: Option<(String, usize)> = None;
    let mut message = String::new();

    loop {
        input.poll(&mut gilrs);
        clear_background(Color::new(0.005, 0.008, 0.04, 1.0));
        draw_rectangle(0.0, 0.0, screen_width(), 8.0, Color::new(0.0, 0.9, 1.0, 1.0));
        draw_rectangle(0.0, screen_height() - 8.0, screen_width(), 8.0, Color::new(1.0, 0.0, 0.65, 1.0));

        if keyboard.is_some() {
            let mut close_keyboard = input.back;
            let mut committed_name: Option<String> = None;
            let (display_value, display_index) = {
                let (value, key_index) = keyboard.as_mut().expect("keyboard state");
                if input.left { *key_index = key_index.saturating_sub(1); }
                if input.right { *key_index = (*key_index + 1).min(KEYBOARD.len() - 1); }
                if input.up { *key_index = key_index.saturating_sub(9); }
                if input.down { *key_index = (*key_index + 9).min(KEYBOARD.len() - 1); }
            if input.accept {
                match KEYBOARD[*key_index] {
                    "SPACE" => if value.len() < 20 { value.push(' '); },
                    "BACK" => { value.pop(); },
                    "CLEAR" => value.clear(),
                    "DONE" => {
                        let name = clean_name(value);
                            if !name.is_empty() { committed_name = Some(name); }
                            close_keyboard = true;
                    }
                        "CANCEL" => close_keyboard = true,
                    key => if value.len() < 20 { value.push_str(key); },
                }
            }
                (value.clone(), *key_index)
            };

            if let Some(name) = committed_name {
                profiles[selected].name = name;
                let _ = save_profile(&profiles[selected]);
                message = "PROFILE NAME SAVED".into();
            }
            if close_keyboard { keyboard = None; }

            centered("RENAME PROFILE", 74.0, 38.0, WHITE);
            panel(110.0, 100.0, screen_width() - 220.0, 68.0, true);
            centered(&format!("{}|", display_value), 145.0, 34.0, WHITE);
            let cols = 9usize;
            let cell_w = (screen_width() - 180.0) / cols as f32;
            for (index, key) in KEYBOARD.iter().enumerate() {
                let row = index / cols;
                let col = index % cols;
                let x = 90.0 + col as f32 * cell_w;
                let y = 205.0 + row as f32 * 72.0;
                panel(x, y, cell_w - 9.0, 58.0, index == display_index);
                let label = measure_text(key, None, 20, 1.0);
                draw_text(key, x + (cell_w - 9.0 - label.width) / 2.0, y + 37.0, 20.0, WHITE);
            }
            centered("A TYPE   B CANCEL", screen_height() - 28.0, 21.0, LIGHTGRAY);
            next_frame().await;
            continue;
        }

        let item_count = profiles.len() + usize::from(profiles.len() < MAX_PROFILES);
        if input.left && selected > 0 { selected -= 1; }
        if input.right && selected + 1 < item_count { selected += 1; }
        if input.add && profiles.len() < MAX_PROFILES { selected = profiles.len(); }

        if selected == profiles.len() && profiles.len() < MAX_PROFILES && input.accept {
            let slot = (0..MAX_PROFILES).find(|slot| !profiles.iter().any(|p| p.id == PROFILE_IDS[*slot]));
            if let Some(slot) = slot {
                let profile = Profile { id: PROFILE_IDS[slot].into(), name: format!("Profile {slot}"), avatar: slot + 1 };
                if save_profile(&profile).is_ok() {
                    profiles.push(profile);
                    profiles.sort_by_key(|p| PROFILE_IDS.iter().position(|id| *id == p.id).unwrap_or(99));
                    selected = profiles.iter().position(|p| p.id == PROFILE_IDS[slot]).unwrap_or(0);
                    message = "PROFILE CREATED".into();
                }
            }
        } else if selected < profiles.len() {
            if input.accept || input.back {
                if set_active_profile(&profiles[selected].id).is_ok() { break; }
                message = "COULD NOT ACTIVATE PROFILE".into();
            }
            if input.rename { keyboard = Some((profiles[selected].name.clone(), 0)); }
            if input.avatar {
                profiles[selected].avatar = profiles[selected].avatar % AVATAR_COUNT + 1;
                let _ = save_profile(&profiles[selected]);
            }
            if input.delete && profiles[selected].id != "default" {
                let removed = profiles.remove(selected);
                let archive = data_root().join("removed-profiles");
                let _ = fs::create_dir_all(&archive);
                let _ = fs::rename(profile_path(&removed.id), archive.join(format!("{}.conf", removed.id)));
                selected = selected.min(profiles.len().saturating_sub(1));
                message = "PROFILE REMOVED; SAVES WERE KEPT".into();
            }
        }

        centered("CHOOSE PROFILE", 78.0, 42.0, WHITE);
        centered("SAVES AND GAME SETTINGS STAY SEPARATE", 112.0, 22.0, Color::new(0.35, 0.85, 1.0, 1.0));
        let card_w = 250.0;
        let gap = 25.0;
        let total_w = item_count as f32 * card_w + item_count.saturating_sub(1) as f32 * gap;
        let start_x = (screen_width() - total_w) / 2.0;
        for index in 0..item_count {
            let x = start_x + index as f32 * (card_w + gap);
            let y = 175.0;
            panel(x, y, card_w, 330.0, index == selected);
            if let Some(profile) = profiles.get(index) {
                if let Entry::Vacant(entry) = avatars.entry(profile.avatar) {
                    if let Some(texture) = load_avatar(profile.avatar).await { entry.insert(texture); }
                }
                if let Some(texture) = avatars.get(&profile.avatar) {
                    draw_texture_ex(texture, x + 37.0, y + 35.0, WHITE, DrawTextureParams { dest_size: Some(vec2(176.0, 176.0)), ..Default::default() });
                } else {
                    draw_circle(x + 125.0, y + 123.0, 82.0, Color::new(0.15, 0.35, 0.65, 1.0));
                }
                let label = measure_text(&profile.name, None, 26, 1.0);
                draw_text(&profile.name, x + (card_w - label.width) / 2.0, y + 250.0, 26.0, WHITE);
                let status = if profile.id == active_profile() { "CURRENT" } else { "" };
                let width = measure_text(status, None, 19, 1.0).width;
                draw_text(status, x + (card_w - width) / 2.0, y + 290.0, 19.0, Color::new(1.0, 0.25, 0.7, 1.0));
            } else {
                centered_card("+", x, y + 155.0, card_w, 82.0, Color::new(0.2, 0.9, 1.0, 1.0));
                centered_card("ADD PROFILE", x, y + 245.0, card_w, 24.0, WHITE);
            }
        }
        if !message.is_empty() { centered(&message, 555.0, 22.0, Color::new(1.0, 0.35, 0.75, 1.0)); }
        centered("A USE/ADD   X RENAME   Y AVATAR   START ADD   SELECT DELETE", screen_height() - 30.0, 20.0, LIGHTGRAY);
        next_frame().await;
    }
}

fn centered_card(text: &str, x: f32, baseline: f32, width: f32, size: f32, color: Color) {
    let measured = measure_text(text, None, size as u16, 1.0);
    draw_text(text, x + (width - measured.width) / 2.0, baseline, size, color);
}

#[derive(Clone)]
struct Cart {
    name: String,
    system: String,
    runtime: String,
    source: String,
    kzi: PathBuf,
    artwork: PathBuf,
}

fn kzi_value(content: &str, key: &str) -> String {
    content.lines().find_map(|line| line.strip_prefix(&format!("{key}="))).unwrap_or("").trim().to_string()
}

fn load_carts(root: &Path) -> Vec<Cart> {
    let mut carts = Vec::new();
    let Ok(entries) = fs::read_dir(root) else { return carts };
    for entry in entries.flatten() {
        let dir = entry.path();
        if !dir.is_dir() { continue; }
        let kzi = dir.join("cart.kzi");
        let Ok(content) = fs::read_to_string(&kzi) else { continue };
        let name = kzi_value(&content, "Name");
        let runtime = kzi_value(&content, "Runtime");
        let system = kzi_value(&content, "Platform");
        let source = kzi_value(&content, "SourceFile");
        let cover = kzi_value(&content, "Cover");
        let icon = kzi_value(&content, "Icon");
        let artwork = if !cover.is_empty() && dir.join(&cover).is_file() { dir.join(cover) } else { dir.join(icon) };
        if !name.is_empty() && !runtime.is_empty() {
            carts.push(Cart { name, system, runtime, source, kzi, artwork });
        }
    }
    carts.sort_by(|a, b| a.system.cmp(&b.system).then_with(|| a.name.cmp(&b.name)));
    carts
}

fn atomic_output(path: &Path, value: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    let tmp = path.with_extension(format!("tmp.{}", std::process::id()));
    fs::write(&tmp, format!("{}\n", value.display()))?;
    fs::rename(tmp, path)
}

async fn browser_ui(root: PathBuf, output: PathBuf) {
    let carts = load_carts(&root);
    if carts.is_empty() { return; }
    let mut gilrs = Gilrs::new().ok();
    let mut input = Input { stick_neutral: true, ..Default::default() };
    let mut selected = 0usize;
    let mut textures: HashMap<PathBuf, Texture2D> = HashMap::new();
    let columns = 4usize;
    let rows = 2usize;
    let per_page = columns * rows;

    loop {
        input.poll(&mut gilrs);
        if input.left && selected > 0 { selected -= 1; }
        if input.right && selected + 1 < carts.len() { selected += 1; }
        if input.up { selected = selected.saturating_sub(columns); }
        if input.down && selected + columns < carts.len() { selected += columns; }
        if input.back { let _ = fs::remove_file(&output); break; }
        if input.accept { let _ = atomic_output(&output, &carts[selected].kzi); break; }

        clear_background(Color::new(0.004, 0.007, 0.035, 1.0));
        draw_rectangle(0.0, 0.0, screen_width(), 8.0, Color::new(0.0, 0.9, 1.0, 1.0));
        draw_rectangle(0.0, screen_height() - 8.0, screen_width(), 8.0, Color::new(1.0, 0.0, 0.65, 1.0));
        centered("CHOOSE A GAME", 62.0, 40.0, WHITE);
        centered(&format!("{} RECOGNIZED ROMS ON REMOVABLE MEDIA", carts.len()), 91.0, 20.0, Color::new(0.3, 0.85, 1.0, 1.0));

        let page = selected / per_page;
        let start = page * per_page;
        let end = (start + per_page).min(carts.len());
        let card_w = 280.0;
        let card_h = 240.0;
        let gap_x = 22.0;
        let gap_y = 24.0;
        let start_x = (screen_width() - (columns as f32 * card_w + (columns - 1) as f32 * gap_x)) / 2.0;
        for (index, cart) in carts.iter().enumerate().take(end).skip(start) {
            let local = index - start;
            let col = local % columns;
            let row = local / columns;
            let x = start_x + col as f32 * (card_w + gap_x);
            let y = 120.0 + row as f32 * (card_h + gap_y);
            panel(x, y, card_w, card_h, index == selected);
            let art = cart.artwork.clone();
            if !textures.contains_key(&art) {
                if let Ok(texture) = load_texture(art.to_string_lossy().as_ref()).await { textures.insert(art.clone(), texture); }
            }
            if let Some(texture) = textures.get(&art) {
                let ratio = texture.width() / texture.height().max(1.0);
                let (w, h) = if ratio > 1.0 { (136.0, 136.0 / ratio) } else { (136.0 * ratio, 136.0) };
                draw_texture_ex(texture, x + (card_w - w) / 2.0, y + 14.0 + (136.0 - h) / 2.0, WHITE, DrawTextureParams { dest_size: Some(vec2(w, h)), ..Default::default() });
            }
            let mut title = cart.name.clone();
            if title.chars().count() > 28 { title = format!("{}…", title.chars().take(27).collect::<String>()); }
            centered_card(&title, x, y + 178.0, card_w, 22.0, WHITE);
            centered_card(&format!("{}  •  {}", cart.system.to_uppercase(), cart.runtime), x, y + 207.0, card_w, 16.0, Color::new(0.35, 0.85, 1.0, 1.0));
            let source = if cart.source.is_empty() { "REMOVABLE MEDIA" } else { &cart.source };
            let short_source = if source.chars().count() > 34 { format!("{}…", source.chars().take(33).collect::<String>()) } else { source.to_string() };
            centered_card(&short_source, x, y + 229.0, card_w, 14.0, GRAY);
        }
        centered(&format!("PAGE {} / {}   •   A PLAY   B CANCEL", page + 1, carts.len().div_ceil(per_page)), screen_height() - 25.0, 19.0, LIGHTGRAY);
        next_frame().await;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("profiles") => profile_ui().await,
        Some("browser") => {
            let root = args.iter().position(|arg| arg == "--root").and_then(|i| args.get(i + 1)).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/run/media/playfusion-loose-rom"));
            let output = args.iter().position(|arg| arg == "--output").and_then(|i| args.get(i + 1)).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/tmp/playfusion-selected-cart"));
            browser_ui(root, output).await;
        }
        _ => eprintln!("Usage: playfusion-cart-ui {{profiles|browser --root DIR --output FILE}}"),
    }
}
