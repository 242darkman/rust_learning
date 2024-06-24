# Manipulation de chaine de caractère

## Explications détaillées du code

1. **Initialisation de first_name avec une capacité de 10** :
```rust
let mut first_name = String::with_capacity(10);
```

Ici, on crée une chaîne de caractères mutable avec une capacité initiale de 10 caractères. Cela permet d'optimiser les performances si on sait à l'avance que la chaîne aura cette taille ou proche de cette taille.


2. **Ajout de "John" à `first_name`** :
```rust
first_name.push_str("John");
```
On utilise la méthode `push_str` pour ajouter la chaîne "Brandon" à `first_name`.


3. **Conversion de `last_name` en `String`** :
```rust
let last_name = "LANDIS";
let last_name = last_name.to_string();
```
On crée une chaîne littérale `last_name`, puis on la convertit en `String` pour pouvoir la modifier si nécessaire.

4. **Concaténation de `first_name` et `last_name`** :
```rust
let full_name = format!("{} {}", first_name, last_name);
```
On utilise la macro `format!` pour créer une nouvelle chaîne qui contient `first_name` et `last_name` séparés par un espace.


5. **Autre méthode de concaténation** :
```rust
let full_name = first_name.clone() + " " + &last_name;
```
On utilise l'opérateur + pour concaténer `first_name` (après l'avoir clonée) et `last_name`. Notez que `&last_name` est une référence à `last_name` pour éviter de transférer sa propriété.

6. **Ajout de `last_name` à `first_name`** :
```rust
first_name.push_str(&last_name);
```
On ajoute la chaîne `last_name` à la fin de `first_name`.


7. **Nouvelle concaténation**
```rust
let full_name = first_name.clone() + &last_name;
```
On crée une nouvelle chaîne `full_name` en concaténant `first_name` (après clonage) et `last_name`.

8. **Déclaration et modification d'un caractère ** :
```rust
let mut char = 'a';
char = 'b';
```
On déclare une variable mutable de type caractère et on la modifie ensuite.

9. **Erreur de compilation en tentant d'ajouter un caractère à une chaîne** :
```rust
full_name.push_str(char); // Cette ligne provoquera une erreur de compilation car push_str attend une &str.
```
La tentative d'ajouter un caractère à une chaîne de caractères avec `push_str` provoquera une erreur car cette méthode attend une référence à une chaîne (`&str`), pas un caractère.

10. **Affichage du nom complet** :
```rust
println!("full_name: {}", full_name);
```
Enfin, on affiche la valeur de `full_name`.


## Conclusion :
Ce programme montre comment manipuler des chaînes de caractères en Rust, incluant la création, la modification, la concaténation, et certaines erreurs courantes comme l'utilisation incorrecte de push_str avec un caractère.
```