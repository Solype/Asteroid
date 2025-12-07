use bevy::prelude::{Vec2, Vec3};
use std::fs::File;
use xml::reader::{EventReader, XmlEvent};

pub mod structs;

pub fn load_game_config(path: &str) -> structs::GameConfig {
    let file = File::open(path).expect("Cannot open XML file");
    let parser = EventReader::new(file);

    let mut cfg = structs::GameConfig::default();
    let mut scope: Vec<String> = vec![];

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                scope.push(name.local_name.clone());

                match name.local_name.as_str() {
                    "value" => {
                        let v = parse_value(&attributes);
                        match scope_path(&scope).as_str() {
                            "game/ship/speed/value" => cfg.ship.speed = v,
                            "game/ship/gun/ammo/speed/value" => cfg.ship.ammo.speed = v,
                            "game/ship/gun/ammo/despawn_distance/value" => {
                                cfg.ship.ammo.distance_despawn = v
                            }
                            "game/ship/camera/transition/value" => {
                                cfg.main_cam.speed_transition = v
                            }
                            "game/ship/camera/maxfov/value" => cfg.main_cam.maxfov = v,
                            "game/ship/camera/menu/fov/value" => cfg.main_cam.menu.fov = v,
                            "game/ship/camera/driving/fov/value" => cfg.main_cam.driving.fov = v,
                            "game/asteroids/spawn_range/value" => cfg.asteroids.spawn_range = v,
                            "game/asteroids/despawn_range/value" => cfg.asteroids.despawn_range = v,
                            "game/asteroids/max_number/value" => {
                                cfg.asteroids.max_asteroid = v as usize
                            }
                            "game/asteroids/speed/value" => cfg.asteroids.speed = v,
                            "game/asteroids/rotationnal_speed/value" => {
                                cfg.asteroids.rotationnal_speed = v
                            }
                            "game/asteroids/size_range/min/value" => cfg.asteroids.size_range.0 = v,
                            "game/asteroids/size_range/max/value" => cfg.asteroids.size_range.1 = v,
                            "game/ship/virtual_mouse_sensitivity/value" => {
                                cfg.ship.virtual_mouse_sensitivity = v
                            }
                            "game/ship/rotation_speed/value" => cfg.ship.rotation_speed = v,
                            "game/ship/thurst_modifier/value" => cfg.ship.thurst_modifier = v,
                            _ => {}
                        }
                    }

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
                            "game/ship/camera/driving/position/vec3" => {
                                cfg.main_cam.driving.position = v
                            }
                            "game/ship/camera/driving/look/vec3" => {
                                cfg.main_cam.driving.look_at = v
                            }

                            "game/ship/camera/menu/position/vec3" => cfg.main_cam.menu.position = v,
                            "game/ship/camera/menu/look/vec3" => cfg.main_cam.menu.look_at = v,

                            "game/ship/backcamera/position/vec3" => {
                                cfg.ship.backcamera_position = v
                            }
                            "game/ship/backcamera/look_at/vec3" => cfg.ship.backcamera_look_at = v,

                            "game/ship/thruster/particules_color/from/vec3" => {
                                cfg.ship.color_particules.0 = v
                            }
                            "game/ship/thruster/particules_color/to/vec3" => {
                                cfg.ship.color_particules.1 = v
                            }

                            "game/ship/thruster/right/vec3" => cfg.ship.thruster_right = v,
                            "game/ship/thruster/left/vec3" => cfg.ship.thruster_left = v,

                            "game/ship/gun/ammo/color/vec3" => cfg.ship.ammo.color = v,
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

                    "asset" => {
                        if let Some(src) = find_attr(&attributes, "src") {
                            let path = src.to_string();
                            println!("found {} at {}", path, scope_path(&scope).as_str());
                            match scope_path(&scope).as_str() {
                                "game/ship/asset" => cfg.ship.asset = path,
                                "game/ui/background/asset" => cfg.ui.background = path,
                                "game/ui/font/asset" => cfg.ui.font = path,
                                "game/ui/sounds/asset" => cfg.ui.sounds.push(path),
                                "game/ship/gun/ammo/sounds/asset" => {
                                    cfg.ship.ammo.sounds.push(path)
                                }
                                "game/ship/music/asset" => cfg.ship.music = path,
                                "game/ship/alarm/asset" => cfg.ship.alarm = path,
                                "game/ui/music/asset" => cfg.ui.music = path,
                                _ => {}
                            }
                        }
                    }

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
    if cfg.window.x <= 0.0 {
        cfg.window.x = 800.0;
    }
    if cfg.window.y <= 0.0 {
        cfg.window.y = 600.0;
    }

    cfg
}

fn parse_vec2(attrs: &[xml::attribute::OwnedAttribute]) -> Vec2 {
    Vec2 {
        x: find_attr(attrs, "x").unwrap_or("0").parse().unwrap_or(0.0),
        y: find_attr(attrs, "y").unwrap_or("0").parse().unwrap_or(0.0),
    }
}

fn parse_value(attrs: &[xml::attribute::OwnedAttribute]) -> f32 {
    return find_attr(attrs, "value")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0.0);
}

fn parse_vec3(attrs: &[xml::attribute::OwnedAttribute]) -> Vec3 {
    Vec3 {
        x: find_attr(attrs, "x").unwrap_or("0").parse().unwrap_or(0.0),
        y: find_attr(attrs, "y").unwrap_or("0").parse().unwrap_or(0.0),
        z: find_attr(attrs, "z").unwrap_or("0").parse().unwrap_or(0.0),
    }
}

fn find_attr<'a>(attrs: &'a [xml::attribute::OwnedAttribute], key: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| a.name.local_name == key)
        .map(|a| a.value.as_str())
}

fn scope_path(scope: &[String]) -> String {
    scope.join("/")
}
