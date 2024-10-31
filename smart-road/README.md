## Algorithme de Simulation de Trafic

### 1. Initialisation
- Charger les textures (route, voiture).
- Initialiser les statistiques (nombre total de voitures, meilleur temps, etc.).
- Créer une liste vide pour les voitures.
- Définir les dimensions de l'intersection.

### 2. Boucle Principale
Tant que le jeu est en cours :
1. **Gestion des Entrées**
   - Vérifier si l'utilisateur appuie sur "Échap" pour quitter.
   - Vérifier si l'utilisateur appuie sur "P" pour mettre en pause.
   - Vérifier si l'utilisateur appuie sur "D" pour activer le mode debug.
   - Vérifier les touches directionnelles pour générer des voitures dans les directions appropriées.

2. **Mise à Jour des Voitures**
   - Retirer les voitures qui ont atteint leur destination.
   - Pour chaque voiture :
     - Vérifier les collisions avec d'autres voitures.
     - Communiquer avec l'intersection pour déterminer si la voiture doit attendre.
     - Mettre à jour le radar de la voiture.
     - Ajuster la vitesse de la voiture en fonction des conditions.
     - Déplacer la voiture d'un pas si aucune collision n'est détectée.
     - Vérifier si la voiture peut tourner et effectuer le virage si possible.

3. **Rendu**
   - Dessiner l'arrière-plan de l'intersection.
   - Dessiner toutes les voitures et leurs radars.
   - Afficher les statistiques du jeu.

### 3. Fin de Jeu
- Lorsque l'utilisateur choisit de quitter, afficher les statistiques finales.

## Détails des Étapes

### Gestion des Entrées
- Utiliser des touches pour contrôler la génération de voitures (flèches directionnelles pour direction).

### Mise à Jour des Voitures
- Pour chaque voiture, vérifier si elle a atteint sa destination et mettre à jour les statistiques.
- Vérifier les intersections avec d'autres voitures pour gérer les collisions.
- Mettre à jour la position du radar de chaque voiture en fonction de sa direction.
- Ajuster la vitesse en fonction de la proximité d'autres voitures.
- Déterminer si la voiture peut tourner et effectuer le virage si l'espace est libre.

### Rendu
- Dessiner les éléments graphiques à l'écran, y compris les voitures, les radars, et les statistiques.