use macroquad::prelude::*;
use crate::vehicule::*; 

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Stats {
    pub total_cars: u32, // Nombre total de voitures
    pub best_time: f32, // Meilleur temps de passage
    pub worst_time: f32, // Pire temps de passage
    pub best_velocity: f32, // Meilleure vitesse
    pub worst_velocity: f32, // Pire vitesse
    pub collisions: u32, // Nombre de collisions
}

impl Stats {
    // Affiche les statistiques en jeu
    pub fn afficher_stats(&self) {
        draw_text("Statistics", 500.0, 260.0, 40.0, WHITE);
        draw_text(&format!("Total Cars: {}", self.total_cars), 500.0, 320.0, 20.0, WHITE);
        draw_text(&format!("Best Time: {:.2}", self.best_time), 500.0, 350.0, 20.0, WHITE);
        draw_text(&format!("Worst Time: {:.2}", self.worst_time), 500.0, 380.0, 20.0, WHITE);
        draw_text(&format!("Best Velocity: {:.2}", self.best_velocity), 500.0, 410.0, 20.0, WHITE);
        draw_text(&format!("Worst Velocity: {:.2}", self.worst_velocity), 500.0, 440.0, 20.0, WHITE);
        draw_text(&format!("Collisions: {}", self.collisions), 500.0, 470.0, 20.0, WHITE);
    }

}

impl Vehicule{
    pub fn check_for_best_or_worst_time(&self, statistics: &mut Stats) {
        let temp_time = self.duree.elapsed().as_secs_f32();
        if temp_time < statistics.best_time {
            statistics.best_time = temp_time;
        }
        if temp_time > statistics.worst_time {
            statistics.worst_time = temp_time;
        }
        let temp_velocity = self.start_point.distance(self.final_point) / temp_time;
        if temp_velocity > statistics.best_velocity {
            statistics.best_velocity = temp_velocity;
        }
        if temp_velocity < statistics.worst_velocity {
            statistics.worst_velocity = temp_velocity;
        }
    }

    pub fn check_for_collision(&self, temp_cars: &mut Vec<Vehicule>, statistics: &mut Stats) {
        temp_cars.retain(|temp_car| temp_car.uuid != self.uuid);
        if temp_cars
            .iter()
            .any(|temp_car| temp_car.rectangle.intersect(self.rectangle).is_some())
        {
            statistics.collisions += 1;
        }
    }
}
