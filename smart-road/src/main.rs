use macroquad::input::KeyCode::{Down, Left, Right, Up}; // Codes des touches fléchées
use macroquad::{prelude::*, rand::gen_range};
use std::default::Default;
mod vehicule;
mod view;
mod r#move;
use vehicule::*;
use view::*;
mod stats;
use stats::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Smart Road"), // Titre de la fenêtre
        window_height: 1200, // Hauteur de la fenêtre
        window_width: 1200, // Largeur de la fenêtre
        fullscreen: false, // Mode plein écran désactivé
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
     // Variables initiales du jeu
     let mut statistics: Stats = Stats {
        total_cars: 0,
        best_time: 999999999., // Temps le plus court
        worst_time: 0., // Temps le plus long
        best_velocity: 0., // Vitesse maximale
        worst_velocity: 999999999., // Vitesse minimale
        collisions: 0, // Nombre de collisions
    };

    // État du jeu
    let mut is_escaped: bool = false;
    let mut is_exit: bool = false;
    let mut is_paused = false;
    let mut is_debug_mode = false;

    let mut is_random = false; // Génération aléatoire de voitures

    let background: Texture2D = load_texture("assets/background.png").await.unwrap();

      let vehicule_textures: Vec<Texture2D> = vec![
        load_texture("assets/vehicule2.png").await.unwrap(),
        load_texture("assets/vehicule4.png").await.unwrap(),
        load_texture("assets/vehicule5.png").await.unwrap(),
        load_texture("assets/vehicule6.png").await.unwrap(),
        load_texture("assets/vehicule7.png").await.unwrap(),
        load_texture("assets/vehicule8.png").await.unwrap(),
        load_texture("assets/vehicule9.png").await.unwrap(),
        load_texture("assets/vehicule10.png").await.unwrap(),
    ];

    let mut vehicules: Vec<Vehicule> = Vec::new();
    let core_intersection = Rect::new(503., 520., 180., 180.);
    

    loop{
        let random_vehicule_type = gen_range(0, vehicule_textures.len());

        if is_key_pressed(KeyCode::Escape) {
            if is_exit {
                std::process::exit(0); // Quitte l'application
            } else {
                is_escaped = true;
                is_exit = true;
            }
        }
        if is_key_pressed(KeyCode::P) {
            is_paused = !is_paused; // Alterne l'état de pause
        }
        if is_key_pressed(KeyCode::D) {
            is_debug_mode = !is_debug_mode; // Alterne le mode débogage
        }

        // Si échappé, affiche les statistiques de fin de jeu
        if is_escaped {
            statistics.afficher_stats();
        } else if is_paused {

            // Dessine l'arrière-plan (intersection)
            draw_texture(&background, 0., 0., WHITE);
            if is_debug_mode {
                // Dessine un rectangle pour le débogage autour de l'intersection
                draw_rectangle(
                    core_intersection.x,
                    core_intersection.y,
                    core_intersection.w,
                    core_intersection.h,
                    Color::new(0.5, 0.5, 0., 0.1),
                );
            }

            // Dessine les voitures
            vehicules.iter()
                .for_each(|car| car.afficher_vehicules(&vehicule_textures[car.vehicule_type], is_debug_mode));
            // Affiche le texte de pause
            draw_text("Press P to continue", 430., 600., 40., BLACK);
        } else {

            if is_key_pressed(Left) {
            // vehicule =  &vehicule_textures[gen_range(0, vehicule_textures.len())];
                Vehicule::ajouter_vehicule(&mut vehicules, vec!["RU", "RL", "RD"][gen_range(0, 3)], "West", random_vehicule_type);
            } else if is_key_pressed(Up) {
                Vehicule::ajouter_vehicule(&mut vehicules, vec!["DU", "DL", "DR"][gen_range(0, 3)], "North", random_vehicule_type);
            } else if is_key_pressed(Down) {
                Vehicule::ajouter_vehicule(&mut vehicules, vec!["UL", "UD", "UR"][gen_range(0, 3)], "South", random_vehicule_type);
            } else if is_key_pressed(Right) {
                Vehicule::ajouter_vehicule(&mut vehicules, vec!["LU", "LR", "LD"][gen_range(0, 3)], "East", random_vehicule_type);
            } else if is_key_pressed(KeyCode::R) {
                is_random = !is_random; // Alterne la génération aléatoire
            } else if is_random {
                let random_vehicule_type = gen_range(0, vehicule_textures.len()); // Index aléatoire pour la texture
                // Génération aléatoire de voitures
                let mut tab_car: [&str; 150] = [""; 150];
                tab_car[20]="West";
                tab_car[40]="East";
                tab_car[60]="North";
                tab_car[80]="South";

                let direction = tab_car[gen_range(0, 150)];
                random_direction(direction, random_vehicule_type, &mut vehicules)
                
            }
    

        //  Retire les voitures qui ont quitté l'intersection
        retirer_vehicules_sortis(&mut vehicules, &mut statistics);

         // Vérifie les collisions entre les voitures
         let mut temp_cars = vehicules.clone();
         vehicules.iter()
             .for_each(|car| car.check_for_collision(&mut temp_cars, &mut statistics));

        // Communication avec l'intersection
        let _all_vehicules = vehicules.clone();
        vehicules.iter_mut()
            .for_each(|car| car.intersection(&_all_vehicules, &core_intersection));

        // Met à jour les positions des radars après le mouvement des voitures
        let _all_vehicules = vehicules.clone();
        for (car_index, car) in vehicules.iter_mut().enumerate() {
            car.update_zone(car_index, &_all_vehicules);
        }

        // Ajuste la vitesse actuelle des voitures
        vehicules.iter_mut().for_each(|car| car.speed());

        // Déplace les voitures d'un pas si aucune collision
        let mut _all_vehicules = vehicules.clone();
        vehicules.iter_mut()
            .filter(|car| !car.waiting)
            .for_each(|car| car.move_vehicule(&mut _all_vehicules, &mut statistics));

        // Vérifie si les voitures peuvent tourner
        let _all_vehicules = vehicules.clone();
        vehicules.iter_mut().for_each(|car| car.turn_if_can(&_all_vehicules));

        // Dessine l'arrière-plan (intersection)
        draw_texture(&background, 0., 0., WHITE);
        if is_debug_mode {
            draw_rectangle(
                core_intersection.x,
                core_intersection.y,
                core_intersection.w,
                core_intersection.h,
                Color::new(0.5, 0.5, 0., 0.1),
            );
        }

        // Dessine les voitures
        vehicules.iter()
            .for_each(|car| car.afficher_vehicules(&vehicule_textures[car.vehicule_type], is_debug_mode));
    }

        next_frame().await;
    }

}

