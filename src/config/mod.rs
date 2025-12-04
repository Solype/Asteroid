<<<<<<< HEAD
use bevy::{prelude::{Vec2, Vec3}};
=======
use bevy::prelude::{Vec2, Vec3};
>>>>>>> b3f6beb (feat(manifest.xml): now it reads the manifest, create a ressource based on it and use it for the size, name and title of the window)
use xml::reader::{EventReader, XmlEvent};
use std::fs::File;

pub mod structs;

pub fn load_game_config(path: &str) -> structs::GameConfig {
    let file = File::open(path).expect("Cannot open XML file");
    let parser = EventReader::new(file);

    let mut cfg = structs::GameConfig::default();
    let mut scope: Vec<String> = vec![];

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                scope.push(name.local_name.clone());

                match name.local_name.as_str() {
                    "vec2" => {
                        let v = parse_vec2(&attributes);

                        if let Some(parent) = scope.get(scope.len() - 2) {
                            match parent.as_str() {
                                "window" => cfg.window = v,
                                "dimension" => cfg.ui.dimension = v,
                                _ => {}
                            }
                        }
                    }

                    "vec3" => {
                        let v = parse_vec3(&attributes);
                        match scope_path(&scope).as_str() {
                            "game/ship/camera/position/vec3" => cfg.main_cam.position = v,
                            "game/ship/camera/look_at_menu/vec3" => cfg.main_cam.look_at_menu = v,
                            "game/ship/camera/look_at_forward/vec3" => cfg.main_cam.look_at_forward = v,

                            "game/ship/backcamera/position/vec3" => cfg.ship.backcamera_position = v,
                            "game/ship/backcamera/look_at/vec3" => cfg.ship.backcamera_look_at = v,

                            "game/ship/thruster/right/vec3" => cfg.ship.thruster_right = v,
                            "game/ship/thruster/left/vec3" => cfg.ship.thruster_left = v,

                            "game/ship/gun/right/vec3" => cfg.ship.gun_right = v,
                            "game/ship/gun/left/vec3" => cfg.ship.gun_left = v,

                            "game/ship/screens/right/tr/vec3" => cfg.ship.screen_right.tr = v,
                            "game/ship/screens/right/tl/vec3" => cfg.ship.screen_right.tl = v,
                            "game/ship/screens/right/br/vec3" => cfg.ship.screen_right.br = v,
                            "game/ship/screens/right/bl/vec3" => cfg.ship.screen_right.bl = v,

                            "game/ship/screens/center/tr/vec3" => cfg.ship.screen_center.tr = v,
                            "game/ship/screens/center/tl/vec3" => cfg.ship.screen_center.tl = v,
                            "game/ship/screens/center/br/vec3" => cfg.ship.screen_center.br = v,
                            "game/ship/screens/center/bl/vec3" => cfg.ship.screen_center.bl = v,

                            "game/ship/screens/left/tr/vec3" => cfg.ship.screen_left.tr = v,
                            "game/ship/screens/left/tl/vec3" => cfg.ship.screen_left.tl = v,
                            "game/ship/screens/left/br/vec3" => cfg.ship.screen_left.br = v,
                            "game/ship/screens/left/bl/vec3" => cfg.ship.screen_left.bl = v,

                            _ => {}
                        }
                    }

                    "background" => {
                        if let Some(src) = find_attr(&attributes, "src") {
                            cfg.ui.background = src.to_string();
                        }
                    }

                    "font" => {
                        if let Some(src) = find_attr(&attributes, "src") {
                            cfg.ui.font = src.to_string();
                        }
                    }

                    "asset" => {
                        if let Some(src) = find_attr(&attributes, "src") {
                            cfg.ship.asset = src.to_string();
                        }
                    }

<<<<<<< HEAD
                    "mouseasset" => {
                        if let Some(src) = find_attr(&attributes, "src") {
                            cfg.ui.mouseasset = src.to_string();
                        }
                    }

=======
>>>>>>> b3f6beb (feat(manifest.xml): now it reads the manifest, create a ressource based on it and use it for the size, name and title of the window)
                    _ => {}
                }
            }

            Ok(XmlEvent::Characters(s)) => {
                if let Some(parent) = scope.last() {
                    match parent.as_str() {
                        "title" => cfg.window_title = s.trim().to_string(),
                        "name" => cfg.window_name = s.trim().to_string(),
                        _ => {}
                    }
                }
            }

            Ok(XmlEvent::EndElement { .. }) => {
                scope.pop();
            }

            Err(e) => panic!("XML Parse Error: {e}"),

            _ => {}
        }
    }

    // Ensure window has sane default size
    if cfg.window.x <= 0.0 { cfg.window.x = 800.0; }
    if cfg.window.y <= 0.0 { cfg.window.y = 600.0; }

    cfg
}

fn parse_vec2(attrs: &[xml::attribute::OwnedAttribute]) -> Vec2 {
    Vec2 {
        x: find_attr(attrs, "x").unwrap_or("0").parse().unwrap_or(0.0),
        y: find_attr(attrs, "y").unwrap_or("0").parse().unwrap_or(0.0),
    }
}

fn parse_vec3(attrs: &[xml::attribute::OwnedAttribute]) -> Vec3 {
    Vec3 {
        x: find_attr(attrs, "x").unwrap_or("0").parse().unwrap_or(0.0),
        y: find_attr(attrs, "y").unwrap_or("0").parse().unwrap_or(0.0),
        z: find_attr(attrs, "z").unwrap_or("0").parse().unwrap_or(0.0),
    }
}

fn find_attr<'a>(attrs: &'a [xml::attribute::OwnedAttribute], key: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name.local_name == key).map(|a| a.value.as_str())
}

fn scope_path(scope: &[String]) -> String {
    scope.join("/")
}
