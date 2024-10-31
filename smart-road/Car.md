# Algorithme de Simulation de Voitures

## Description

Cet algorithme simule le comportement de plusieurs voitures sur une route, en gérant leur apparition, leur mouvement, les collisions, et les interactions avec les intersections.

## Algorithme

1. **Initialisation :**
   - Créer une liste vide pour stocker les voitures.
   - Définir les constantes pour la taille des voitures et des radars.

2. **Création d'une Voiture :**
   - Générer un identifiant unique pour la voiture.
   - Déterminer le point d'apparition en fonction du comportement (ex. : direction).
   - Initialiser les attributs de la voiture (vitesse, direction, etc.).

3. **Apparition des Voitures :**
   - Vérifier si une nouvelle voiture peut apparaître sans collision avec d'autres.
   - Si possible, ajouter la voiture à la liste.

4. **Déplacement des Voitures :**
   - Pour chaque voiture, vérifier sa direction.
   - Déplacer la voiture dans la direction actuelle en fonction de sa vitesse.
   - Vérifier les collisions avec d'autres voitures.

5. **Gestion des Intersections :**
   - Vérifier si la voiture atteint une intersection.
   - Si oui, déterminer si elle doit attendre ou continuer en fonction des autres voitures.

6. **Mise à Jour des Radars :**
   - Mettre à jour la position du radar de la voiture en fonction de sa direction.
   - Vérifier les distances avec d'autres voitures pour ajuster la vitesse.

7. **Vérification des Temps et Vitesses :**
   - Calculer le temps écoulé depuis l'apparition de la voiture.
   - Mettre à jour les statistiques de temps et de vitesse (meilleur/pire).

8. **Dessiner les Voitures :**
   - Afficher les voitures sur l'écran avec leur image et leurs rectangles de collision.
   - Afficher les radars pour visualiser les zones de détection.

9. **Répéter le Processus :**
   - Continuer à mettre à jour la simulation jusqu'à ce qu'un événement d'arrêt soit déclenché (ex. : fermeture du programme).
   