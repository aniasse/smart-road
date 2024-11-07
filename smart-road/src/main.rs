use macroquad::input::KeyCode::{Down, Left, Right, Up}; // Codes des touches fléchées
use macroquad::{prelude::*, rand::gen_range}; // Préférences et génération aléatoire
use std::default::Default; // Pour les valeurs par défaut
mod vehicule; // Module pour les voitures
use vehicule::*; // Utilisation des éléments du module vehicule
// use std::thread;
// use std::time::Duration;
// Fonction de configuration de la fenêtre
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

    let mut is_random = false; // Génération aléatoire de voitures

    let background: Texture2D = load_texture("assets/background.png").await.unwrap();
    //let mut vehicule: Texture2D = load_texture("assets/vehicule1.png").await.unwrap();
      // Charger plusieurs textures de véhicules
      let vehicule_textures: Vec<Texture2D> = vec![
        // load_texture("assets/vehicule1.png").await.unwrap(),
        load_texture("assets/vehicule2.png").await.unwrap(),
        // load_texture("assets/vehicule3.png").await.unwrap(),
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
            let mut tab_car: [&str; 150] = [""; 150];;
            tab_car[20]="West";
            tab_car[40]="East";
            tab_car[60]="North";
            tab_car[80]="South";

            let random_direction = tab_car[gen_range(0, 150)];
            match random_direction {
                "West" => {
                    Vehicule::ajouter_vehicule(
                        &mut vehicules,
                        vec!["RU", "RL", "RD"][gen_range(0, 3)],
                        random_direction,
                        random_vehicule_type,
                    );
                }
                "North" => {
                    Vehicule::ajouter_vehicule(
                        &mut vehicules,
                        vec!["DU", "DL", "DR"][gen_range(0, 3)],
                        random_direction,
                        random_vehicule_type,
                    );
                }
                "South" => {
                    Vehicule::ajouter_vehicule(
                        &mut vehicules,
                        vec!["UL", "UD", "UR"][gen_range(0, 3)],
                        random_direction,
                        random_vehicule_type,
                    );
                }
                "East" => {
                    // handle.join().unwrap();
                    Vehicule::ajouter_vehicule(
                        &mut vehicules,
                        vec!["LU", "LR", "LD"][gen_range(0, 3)],
                        random_direction,
                        random_vehicule_type,
                    );
                }
                _ => {}
            }
            
        }

      //   MISE À JOUR DE L'ÉTAT
      //  Avance la simulation d'une étape

      //  Retire les voitures qui ont quitté l'intersection
        vehicules.retain(|car| {
            if &*car.current_direction == "West" && car.rectangle.x < 100. {
                false
            } else if &*car.current_direction == "North" && car.rectangle.y < 100. {
                false
            } else if &*car.current_direction == "South" && car.rectangle.y > 1050. {
                false
            } else if &*car.current_direction == "East"
                && car.rectangle.x + car.car_size.long_edge > 1100.
            {
                false
            } else {
                true
            }
        });

        // Vérifie les collisions entre les voitures
        let mut temp_vehicules = vehicules.clone();

        // Communication avec l'intersection
        let temp_vehicules = vehicules.clone();
        vehicules.iter_mut()
            .for_each(|car| car.intersection(&temp_vehicules, &core_intersection));

        // Met à jour les positions des radars après le mouvement des voitures
        let temp_vehicules = vehicules.clone();
        for (car_index, car) in vehicules.iter_mut().enumerate() {
            car.update_zone(car_index, &temp_vehicules);
        }

        // Ajuste la vitesse actuelle des voitures
        vehicules.iter_mut().for_each(|car| car.speed());

        // Déplace les voitures d'un pas si aucune collision
        let mut temp_vehicules = vehicules.clone();
        vehicules.iter_mut()
            .filter(|car| !car.waiting)
            .for_each(|car| car.move_vehicule(&mut temp_vehicules));

        // Vérifie si les voitures peuvent tourner
        let temp_vehicules = vehicules.clone();
        vehicules.iter_mut().for_each(|car| car.turn_if_can(&temp_vehicules));

        // RENDU / DESSIN
        // Dessine le jeu à l'écran

        // Dessine l'arrière-plan (intersection)
        draw_texture(&background, 0., 0., WHITE);

        // Dessine les voitures
        vehicules.iter()
            .for_each(|car| car.afficher_vehicules(&vehicule_textures[car.vehicule_type]));

    next_frame().await;
    }

}

