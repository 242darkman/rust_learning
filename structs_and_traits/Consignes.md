# Entrainement 1

- Ecrivez une Struct **Student** avec les champs adéquats (**name, id, email, etc.**). Ecrivez une fonction prenant en paramètre une référence vers un Student et qui devra imprimer les détails de cette instance de façon élégante. Utiliser un type Uuid pour le champ id. Pour cela, vous aurez besoin d’un crate à ajouter au projet : uuid. Faites attention aux features !
- Ecrivez une Struct **Pet** avec des champs comme **name, age, is_vaccinated**. Ecrivez une fonction allow_pet prenant une instance de Pet en paramètre. Si celle-ci est vaccinée, renvoyez true, sinon renvoyez false.


# Entrainement 2

Reprenez le code présenté juste avant avec la struct Car et le trait Vehicle, et implémentez le trait Vehicle pour 2 autres Struct **Boat** et **Airplane**. Enrichissez ces structs de champs tels que is_in_motion, current_speed, etc. Ces champs devront changez en fonction des méthodes start et stop, vous devrez donc modifier l’implémentation du trait Vehicle pour rendre les références à self mutable.


# Entrainement

- Écris une méthode new pour créer une nouvelle instance d'une struct Personne avec un nom et un âge donné.
- Implémente une méthode afficher_details qui affiche le nom et l'âge d'une personne.
- Crée une méthode anniversaire qui incrémente l'âge d'une personne de 1.
- Ajoute une méthode changer_nom pour modifier le nom d'une personne.
- Implémente une méthode is_older_than qui compare l'âge de deux personnes et renvoie un booléen indiquant si la première est plus âgée que la deuxième.