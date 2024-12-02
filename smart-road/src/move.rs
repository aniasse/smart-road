use macroquad::{prelude::*};
use crate::vehicule::*;
use crate::stats::*;

impl Vehicule{

    // Déplace le véhicule selon sa direction
    pub fn move_vehicule(&mut self, temp_vehicules: &mut Vec<Vehicule>, statistics: &mut Stats) {
        let mut temp_self_car = self.clone();
        temp_vehicules.retain(|car| temp_self_car.uuid != car.uuid);

        match &*self.current_direction {
            "West" => {
                temp_self_car.rectangle.x -= temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.x -= self.current_speed;
                } else {
                    statistics.close_calls += 1;
                }
            }
            "North" => {
                temp_self_car.rectangle.y -= temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.y -= self.current_speed;
                } else {
                    statistics.close_calls += 1;
                }
            }
            "South" => {
                temp_self_car.rectangle.y += temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.y += self.current_speed;
                } else {
                    statistics.close_calls += 1;
                }
            }
            "East" => {
                temp_self_car.rectangle.x += temp_self_car.current_speed;
                if temp_vehicules.iter_mut().all(|car| temp_self_car.rectangle.intersect(car.rectangle).is_none()){
                    temp_vehicules.push(temp_self_car);
                    self.rectangle.x += self.current_speed;
                } else {
                    statistics.close_calls += 1;
                }
            }
            _ => {}
        };
    }

    // Met à jour la vitesse du véhicule en fonction de la zone
    pub fn speed(&mut self) {
        if &*self.current_direction == "West" || &*self.current_direction == "East" {
            match self.zone.w {
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

    // Gère l'intersection du véhicule avec d'autres véhicules
    pub fn intersection(&mut self, vehicules_ref: &Vec<Vehicule>, core_intersection: &Rect) {
        let mut temp_vehicules = vehicules_ref.clone();
        temp_vehicules.retain(|car| car.uuid != self.uuid);
        if self.status == "LR" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.status == "LR" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.status == "LU" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.status == "LU" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.status == "RD" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.status == "RD" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.status == "RL" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                car.status == "RL" && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }

        if self.status == "UR" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.status == "UR" || car.status == "RL")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.status == "UD" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.status == "UD" || car.status == "RL")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }

        if self.status == "DL" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.status == "DL" || car.status == "UR")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
        if self.status == "DU" && self.zone.intersect(*core_intersection).is_some()&& self.rectangle.intersect(*core_intersection).is_none()
        {
            self.waiting = false;
            if temp_vehicules.iter().any(|car| {
                (car.status == "DU" || car.status == "LR")
                    && car.rectangle.intersect(*core_intersection).is_some()
            }) {
                self.waiting = true;
            }
        }
    }

}