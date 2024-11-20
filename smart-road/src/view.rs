// view.rs
use macroquad::{prelude::*, rand::gen_range};
use crate::vehicule::*;
use crate::stats::*;

impl Vehicule {
    // Affiche tous les véhicules
    pub fn afficher_vehicules(&self, vehicule_img: &Texture2D, debug: bool) {

        if debug {
            // Draw Radar Rect
            draw_rectangle(
                self.zone.x,
                self.zone.y,
                self.zone.w,
                self.zone.h,
                Color::new(1.0, 0.0, 0.0, 0.1),
            );

            // Draw Car Rect
            draw_rectangle(
                self.rectangle.x,
                self.rectangle.y,
                self.rectangle.w,
                self.rectangle.h,
                Color::new(1.0, 0.0, 0.0, 0.1),
            );
        }

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
                );
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
                );
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
        };
    }
}

   // Fonction pour retirer les véhicules qui ont quitté l'intersection
pub fn retirer_vehicules_sortis(vehicules: &mut Vec<Vehicule>, statistics: &mut Stats) {

    vehicules.retain(|car| {
        if &*car.current_direction == "West" && car.rectangle.x < 100. {
            car.check_for_best_or_worst_time(statistics);
            statistics.total_cars += 1;
            false
        } else if &*car.current_direction == "North" && car.rectangle.y < 100. {
            car.check_for_best_or_worst_time(statistics);
            statistics.total_cars += 1;
            false
        } else if &*car.current_direction == "South" && car.rectangle.y > 1050. {
            car.check_for_best_or_worst_time(statistics);
            statistics.total_cars += 1;
            false
        } else if &*car.current_direction == "East" && car.rectangle.x + car.car_size.long_edge > 1100. {
            car.check_for_best_or_worst_time(statistics);
            statistics.total_cars += 1;
            false
        } else {
            true
        }
    });
}


pub fn random_direction(direction: &str, random_vehicule_type: usize, vehicules: &mut Vec<Vehicule>){
    match direction {
        "West" => {
            Vehicule::ajouter_vehicule(
                vehicules,
                vec!["RU", "RL", "RD"][gen_range(0, 3)],
                direction,
                random_vehicule_type,
            );
        }
        "North" => {
            Vehicule::ajouter_vehicule(
                vehicules,
                vec!["DU", "DL", "DR"][gen_range(0, 3)],
                direction,
                random_vehicule_type,
            );
        }
        "South" => {
            Vehicule::ajouter_vehicule(
                vehicules,
                vec!["UL", "UD", "UR"][gen_range(0, 3)],
                direction,
                random_vehicule_type,
            );
        }
        "East" => {
            Vehicule::ajouter_vehicule(
                vehicules,
                vec!["LU", "LR", "LD"][gen_range(0, 3)],
                direction,
                random_vehicule_type,
            );
        }
        _ => {}
    }
}