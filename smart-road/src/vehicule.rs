use macroquad::{prelude::*, rand::gen_range};
use uuid::Uuid;
use std::time::Instant;

pub const CAR_SIZE: Vec2 = vec2(43., 33.);
pub const ZONE_SIZE: Vec2 = vec2(43., 33.);

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
    pub status: String,// Code de comportement pour déterminer comment la voiture se déplace
    pub waiting: bool,// Indique si la voiture attend à une intersection
    pub car_size: Dimensions,//
    pub zone_size: Dimensions,//
    pub final_point: Vec2,//
    pub current_speed: f32,
    pub randomized_initial_speed: f32,
    pub duree: Instant,
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
                start_point.x - ZONE_SIZE.x,
                start_point.y,
                ZONE_SIZE.x,
                ZONE_SIZE.y,
            ),
            proximity: ZONE_SIZE.x,
            current_direction: initial_direction.to_string(),
            randomized_initial_speed: random_speed,
            current_speed: random_speed,
            has_turned: false,
            status: randomized_behavior.to_string(),
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
            duree: Instant::now(),
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

    // Met à jour la zone de détection du véhicule
    pub fn update_zone(&mut self, car_index: usize, temp_vehicules: &Vec<Vehicule>) {
        match &*self.current_direction {
            "West" => {
                (self.zone.x, self.zone.y) = (self.rectangle.x - self.zone_size.long_edge, self.rectangle.y);
                (self.zone.w, self.zone.h) = (self.zone_size.long_edge, self.zone_size.short_edge);

                for (other_index, other_car) in temp_vehicules.iter().enumerate() {
                    if car_index != other_index && self.zone.intersect(other_car.rectangle).is_some(){
                        self.zone.x = other_car.rectangle.x + other_car.rectangle.w;
                    }
                    self.zone.w = (self.rectangle.x - self.zone.x).abs().min(43.);
                }
            }
            "North" => {
                (self.zone.x, self.zone.y) = (self.rectangle.x, self.rectangle.y - self.zone_size.long_edge);
                for (other_index, other_car) in temp_vehicules.iter().enumerate() {
                    if car_index != other_index && (self.zone.intersect(other_car.rectangle).is_some()){
                        self.zone.y = other_car.rectangle.y + other_car.rectangle.h;
                    }
                    self.zone.h = (self.rectangle.y - self.zone.y).abs().min(43.);
                    self.zone.w = 33.;
                }
            }
            "South" => {

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

 // Gère le tournant si possible
    pub fn turn_if_can(&mut self, temp_vehicules: &Vec<Vehicule>) {
        if !self.has_turned && self.status == "RU" && self.rectangle.x <= 683. {
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
        if !self.has_turned && self.status == "RD" && self.rectangle.x <= 555. {
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
        if !self.has_turned && self.status == "DR" && self.rectangle.y <= 695. {
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
        if !self.has_turned && self.status == "DL" && self.rectangle.y <= 574. {
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
            && self.status == "LD"
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
            && self.status == "LU"
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
        if !self.has_turned && self.status == "UL" && self.rectangle.y + self.car_size.long_edge >= 528.
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
        if !self.has_turned && self.status == "UR" && self.rectangle.y + self.car_size.long_edge >= 650.
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
                            && other_car.status == "DL"))
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

}
