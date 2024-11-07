use macroquad::{prelude::*, rand::gen_range};
use uuid::Uuid;
use std::thread;
use std::time::Duration;

pub const CAR_SIZE: Vec2 = vec2(43., 33.);
pub const Zone_SIZE: Vec2 = vec2(43., 33.);

#[derive(Clone, Debug, PartialEq)]
pub struct Vehicule {
    pub uuid: Uuid,
    pub vehicule_type: usize,
    pub start_point: Vec2, // Point de départ de la voiture.
    pub rectangle: Rect, // Rectangle représentant la position et la taille de la voiture.
    pub current_direction: String,// Direction actuelle de la voiture
    pub zone: Rect,// Rectangle représentant la zone de détection de la voiture
    pub proximity: f32,// Distance de détection
    pub has_turned: bool,// Indique si la voiture a tourné
    pub behavior_code: String,// Code de comportement pour déterminer comment la voiture se déplace
    pub waiting: bool,// Indique si la voiture attend à une intersection
    pub car_size: Dimensions,//
    pub zone_size: Dimensions,//
    pub final_point: Vec2,//
    pub current_speed: f32,
    pub randomized_initial_speed: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions {
    pub long_edge: f32,
    pub short_edge: f32,
    pub delta_edge: f32,
}

impl Vehicule {
    pub fn new(randomized_behavior: &str, initial_direction: &str, vehicule_type: usize) -> Self {
        let random_speed = gen_range(0.8, 2.);
        let start_point = match randomized_behavior {
            "RU" => vec2(1050., 495.),
            "RL" => vec2(1050., 535.),
            "RD" => vec2(1050., 574.),
            "DU" => vec2(643., 1050.),
            "DL" => vec2(603., 1050.),
            "DR" => vec2(683., 1050.),
            "LU" => vec2(150., 617.),
            "LR" => vec2(150., 655.),
            "LD" => vec2(150., 695.),
            "UD" => vec2(516., 100.),
            "UR" => vec2(558., 100.),
            "UL" => vec2(477., 100.),
            _ => panic!("Unexpected lane"),
        };

        Vehicule {
            uuid: Uuid::new_v4(),
            start_point: start_point,
            rectangle: if initial_direction == "West" || initial_direction == "East" {
                Rect::new(start_point.x, start_point.y, CAR_SIZE.x, CAR_SIZE.y)
            } else {
                Rect::new(start_point.x, start_point.y, CAR_SIZE.y, CAR_SIZE.x)
            },
           zone : Rect::new(
                start_point.x - Zone_SIZE.x,
                start_point.y,
                Zone_SIZE.x,
                Zone_SIZE.y,
            ),
            proximity: Zone_SIZE.x,
            current_direction: initial_direction.to_string(),
            randomized_initial_speed: random_speed,
            current_speed: random_speed,
            has_turned: false,
            behavior_code: randomized_behavior.to_string(),
            waiting: false,

            car_size: Dimensions {
                long_edge: 43.,
                short_edge: 33.,
                delta_edge: CAR_SIZE.x - CAR_SIZE.y,
            },
            zone_size: Dimensions {
                long_edge: 43.,
                short_edge: 33.,
                delta_edge: CAR_SIZE.x - CAR_SIZE.y,
            },
            final_point: match randomized_behavior {
                "RU" => vec2(683., 100.),
                "RL" => vec2(100., 535.),
                "RD" => vec2(555., 1050.),
                "DU" => vec2(643., 100.),
                "DL" => vec2(100., 574.),
                "DR" => vec2(1057., 695.),
                "LU" => vec2(593., 100.),
                "LR" => vec2(1057., 655.),
                "LD" => vec2(567., 1050.),
                "UD" => vec2(516., 1050.),
                "UR" => vec2(1057., 607.),
                "UL" => vec2(100., 485.),
                _ => panic!("Unexpected lane"),
            },
            vehicule_type,
        }
    }
 // Ajoute un véhicule à la liste, s'il n'y a pas de collision
    pub fn ajouter_vehicule(vehicules_ref: &mut Vec<Vehicule>, randomized_behavior: &str, initial_direction: &str, vehicule_type: usize) {
        let possible_new_car = Vehicule::new(randomized_behavior, initial_direction, vehicule_type);
        if !vehicules_ref.iter_mut().any(|other_car| {
            possible_new_car.rectangle.intersect(other_car.rectangle).is_some()
        }) && vehicules_ref.len() < 9999
        {
            vehicules_ref.push(possible_new_car)
        }
    }
// Gère l'intersection du véhicule avec d'autres véhicules
    pub fn intersection(&mut self, vehicules_ref: &Vec<Vehicule>, core_intersection: &Rect) {
        let mut temp_vehicules = vehicules_ref.clone();
        temp_vehicules.retain(|car| car.uuid != self.uuid);
        if self.behavior_code == "LR" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.behavior_code == "LR" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.behavior_code == "LU" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.behavior_code == "LU" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.behavior_code == "RD" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.behavior_code == "RD" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.behavior_code == "RL" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.behavior_code == "RL" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }

        if self.behavior_code == "UR" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.behavior_code == "UR" || car.behavior_code == "RL")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.behavior_code == "UD" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.behavior_code == "UD" || car.behavior_code == "RL")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }

        if self.behavior_code == "DL" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.behavior_code == "DL" || car.behavior_code == "UR")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.behavior_code == "DU" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.behavior_code == "DU" || car.behavior_code == "LR")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
    }

// Déplace le véhicule selon sa direction
    pub fn move_vehicule(&mut self, temp_vehicules: &mut Vec<Vehicule>) {
        let mut temp_self_car = self.clone();
        temp_vehicules.retain(|car| temp_self_car.uuid != car.uuid);

        match &*self.current_direction {
            "West" => {
                temp_self_car.rectangle.x -= temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.x -= self.current_speed;
                } 
            }
            "North" => {
                temp_self_car.rectangle.y -= temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.y -= self.current_speed;
                }
            }
            "South" => {
                temp_self_car.rectangle.y += temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.y += self.current_speed;
                }
            }
            "East" => {
                temp_self_car.rectangle.x += temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.x += self.current_speed;
                }
            }
            _ => {}
        };
    }

    // Met à jour la zone de détection du véhicule
    pub fn update_zone(&mut self, car_index: usize, temp_vehicules: &Vec<Vehicule>) {
        match &*self.current_direction {
            "West" => {
                // Update zone rectangle
                (self.zone.x, self.zone.y) = (self.rectangle.x - self.zone_size.long_edge, self.rectangle.y);
                (self.zone.w, self.zone.h) = (self.zone_size.long_edge, self.zone_size.short_edge);

                // Reposition the zone when intersection occur
                for (other_index, other_car) in temp_vehicules.iter().enumerate() {
                    if car_index != other_index && self.zone.intersect(other_car.rectangle).is_some(){
                        self.zone.x = other_car.rectangle.x + other_car.rectangle.w;
                    }
                    // Update zone width
                    self.zone.w = (self.rectangle.x - self.zone.x).abs().min(43.);
                }
            }
            "North" => {
                // Update zone rectangle
                (self.zone.x, self.zone.y) = (self.rectangle.x, self.rectangle.y - self.zone_size.long_edge);
                for (other_index, other_car) in temp_vehicules.iter().enumerate() {
                    if car_index != other_index && (self.zone.intersect(other_car.rectangle).is_some()){
                        self.zone.y = other_car.rectangle.y + other_car.rectangle.h;
                    }
                    // Update zone width
                    self.zone.h = (self.rectangle.y - self.zone.y).abs().min(43.);
                    self.zone.w = 33.;
                }
            }
            "South" => {
                // Update zone rectangle

                (self.zone.x, self.zone.y) = (self.rectangle.x, self.rectangle.y + self.zone_size.long_edge);
                (self.zone.w, self.zone.h) = (self.zone_size.short_edge, self.zone_size.long_edge);
                for (other_index, other_car) in temp_vehicules.iter().enumerate() {
                    if car_index != other_index && self.zone.intersect(other_car.rectangle).is_some()
                    {
                        //self.zone.h = vec2(self.zone.x, self.zone.y).distance(vec2(other_car.rectangle.x, other_car.rectangle.y)).min(self.zone_size.long_edge)
                        self.zone.h = other_car.rectangle.y - (self.rectangle.y + self.car_size.long_edge)
                    }
                }
            }
            "East" => {
                // Update zone rectangle
                (self.zone.x, self.zone.y) = (self.rectangle.x + self.rectangle.w, self.rectangle.y);
                (self.zone.w, self.zone.h) = (self.zone_size.long_edge, self.zone_size.short_edge);

                for (other_index, other_car) in temp_vehicules.iter().enumerate() {
                    if car_index != other_index && self.zone.intersect(other_car.rectangle).is_some(){
                        //self.zone.y = other_car.rectangle.y + other_car.rectangle.h;
                        self.zone.w = other_car.rectangle.x - (self.rectangle.x + self.rectangle.w);
                    }
                    if self.uuid != other_car.uuid
                        && self.zone.intersect(other_car.zone).is_some()
                        && self.rectangle.intersect(other_car.zone).is_none()
                        && other_car.current_direction != "North"
                    {
                        self.zone.w = other_car.rectangle.x - (self.rectangle.x + self.rectangle.w);
                    }
                }
            }
            _ => {}
        }
    }
// Met à jour la vitesse du véhicule en fonction de la zone
    pub fn speed(&mut self) {
        if &*self.current_direction == "West" || &*self.current_direction == "East" {
            match self.zone.w {
                //zone_width if zone_width <= 4. => self.current_speed = 0.,
                zone_width if zone_width <= 3. => {
                    self.current_speed = self.randomized_initial_speed * 0.;
                }
                zone_width if zone_width <= 30. => {
                    self.current_speed = self.randomized_initial_speed * 0.25;
                }
                zone_width if zone_width <= 39. => {
                    self.current_speed = self.randomized_initial_speed * 0.50
                }
                _ => self.current_speed = self.randomized_initial_speed,
            }
        } else if &*self.current_direction == "North" || &*self.current_direction == "South" {
            match self.zone.h {
                //zone_height if zone_height <= 4. => self.current_speed = 0.,
                zone_height if zone_height <= 3. => {
                    self.current_speed = 0.;
                }
                zone_height if zone_height <= 20. => {
                    self.current_speed = self.randomized_initial_speed * 0.25;
                }
                zone_height if zone_height <= 39. => {
                    self.current_speed = self.randomized_initial_speed * 0.50;
                }
                _ => self.current_speed = self.randomized_initial_speed,
            }
        }
    }
 // Gère le tournant si possible
    pub fn turn_if_can(&mut self, temp_vehicules: &Vec<Vehicule>) {
        if !self.has_turned && self.behavior_code == "RU" && self.rectangle.x <= 683. {
            self.waiting = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                683.,
                self.rectangle.y - (self.rectangle.w - self.rectangle.h).abs(),
                self.rectangle.h,
                self.rectangle.w,
            );
            for other_car in temp_vehicules {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.rectangle).is_some()
                        || temp_rect.intersect(other_car.rectangle).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.rectangle = temp_rect;
                self.waiting = false;
                self.current_direction = "North".to_string();
                self.has_turned = true;
            }
        }
        if !self.has_turned && self.behavior_code == "RD" && self.rectangle.x <= 555. {
            self.waiting = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(555., self.rectangle.y, self.rectangle.h, self.rectangle.w);
            for other_car in temp_vehicules {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.rectangle).is_some()
                        || temp_rect.intersect(other_car.rectangle).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.rectangle = temp_rect;
                self.waiting = false;
                self.current_direction = "South".to_string();
                self.has_turned = true;
            }
        }
        if !self.has_turned && self.behavior_code == "DR" && self.rectangle.y <= 695. {
            self.waiting = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(self.rectangle.x, 695., self.rectangle.h, self.rectangle.w);
            for other_car in temp_vehicules {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.rectangle).is_some()
                        || temp_rect.intersect(other_car.rectangle).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.rectangle = temp_rect;
                self.waiting = false;
                self.current_direction = "East".to_string();
                self.has_turned = true;
            }
        }
        if !self.has_turned && self.behavior_code == "DL" && self.rectangle.y <= 574. {
            self.waiting = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                self.rectangle.x - (self.rectangle.h - self.rectangle.w).abs(),
                574.,
                self.rectangle.h,
                self.rectangle.w,
            );
            for other_car in temp_vehicules {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.rectangle).is_some()
                        || temp_rect.intersect(other_car.rectangle).is_some())
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.rectangle = temp_rect;
                self.waiting = false;
                self.current_direction = "West".to_string();
                self.has_turned = true;
            }
        }
        if !self.has_turned
            && self.behavior_code == "LD"
            && self.rectangle.x + self.car_size.long_edge >= 510.
        {
            self.waiting = true;
            let temp_rect = Rect::new(
                510. - (self.car_size.long_edge - self.car_size.delta_edge),
                self.rectangle.y,
                self.car_size.short_edge,
                self.car_size.long_edge,
            );
            self.rectangle = temp_rect;
            self.waiting = false;
            self.current_direction = "South".to_string();
            self.has_turned = true;
        }
        if !self.has_turned
            && self.behavior_code == "LU"
            && self.rectangle.x + self.car_size.delta_edge >= 603.
        {
            self.waiting = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                603.,
                self.rectangle.y - self.car_size.delta_edge,
                self.car_size.short_edge,
                self.car_size.long_edge,
            );
            for other_car in temp_vehicules {
                if self.uuid != other_car.uuid && temp_rect.intersect(other_car.rectangle).is_some()
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.rectangle = temp_rect;
                self.waiting = false;
                self.current_direction = "North".to_string();
                self.has_turned = true;
            }
        }
        if !self.has_turned && self.behavior_code == "UL" && self.rectangle.y + self.car_size.long_edge >= 528.
        {
            self.waiting = true;
            let temp_rect = Rect::new(
                self.rectangle.x - self.car_size.delta_edge,
                528. - (self.car_size.long_edge - self.car_size.delta_edge),
                self.car_size.long_edge,
                self.car_size.short_edge,
            );

            self.rectangle = temp_rect;
            self.waiting = false;
            self.current_direction = "West".to_string();
            self.has_turned = true;
        }
        if !self.has_turned && self.behavior_code == "UR" && self.rectangle.y + self.car_size.long_edge >= 650.
        {
            self.waiting = true;
            let mut clear_to_turn = true;
            let temp_rect = Rect::new(
                self.rectangle.x,
                650. - (self.car_size.long_edge - self.car_size.delta_edge),
                self.car_size.long_edge,
                self.car_size.short_edge,
            );
            for other_car in temp_vehicules {
                if self.uuid != other_car.uuid
                    && (temp_rect.intersect(other_car.rectangle).is_some()
                        || (temp_rect.intersect(other_car.zone).is_some()
                            && other_car.behavior_code == "DL"))
                {
                    clear_to_turn = false;
                }
            }
            if clear_to_turn {
                self.rectangle = temp_rect;
                self.waiting = false;
                self.current_direction = "East".to_string();
                self.has_turned = true;
            }
        }
    }
//afficher tous les vehicules
    pub fn afficher_vehicules(&self, vehicule_img: &Texture2D) {
        match &*self.current_direction {
            "West" => draw_texture_ex(
                vehicule_img,
                self.rectangle.x + 1.5,
                self.rectangle.y + 1.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(40., 30.)),
                    source: None,
                    rotation: 0.,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            ),
            "North" => {
                let degree: f32 = 90.;
                draw_texture_ex(
                    vehicule_img,
                    self.rectangle.x - 3.,
                    self.rectangle.y + 7.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(40., 30.)),
                        source: None,
                        rotation: degree.to_radians(),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                )
            }
            "South" => {
                let degree: f32 = 270.;
                draw_texture_ex(
                    vehicule_img,
                    self.rectangle.x - 3.,
                    self.rectangle.y + 7.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(40., 30.)),
                        source: None,
                        rotation: degree.to_radians(),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                )
            }
            "East" => {
                let degree: f32 = 180.;
                draw_texture_ex(
                    vehicule_img,
                    self.rectangle.x + 2.,
                    self.rectangle.y + 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(40., 30.)),
                        source: None,
                        rotation: degree.to_radians(),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                )
            }
            _ => {}
        }
    }
}
