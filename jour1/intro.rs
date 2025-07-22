/ doc: https://doc.rust-lang.org/rust-by-example/
use std::io; // biliothèque de stream  input / ouput 
fn main(){

    println!("Bienvenue à la formation RUST !"); 

    //1.  les variables RUST
     let nom = "Erwan";
     let _age:u32 = 23 ; // u32 = entier non signé sur 32 bits ( valeurs positives)
     let age_chien = 20 ; // rust comprend que c'est entier par défaut i32 
     let temperature:f32 = 21.5; 
     
     println!("Hello la température à lyon {}", temperature);

       // en langage C printf("le nombre est %d",nombre); 

      println!("le chien  de lyon  a {}", age_chien);  
       println!("Hello {} , la temperature à lyon {} est", nom, temperature);

       // Il faut utiliser les snake_case ( par convention de RUST) 
       // ne jamais commencer par un chiffre devant la variable, pas d'espace, ni tirets

       // i32     signé     de -2xxxxx à + 2xxxxxx
       // u32     non signé  de 0 à 4 xxxxxxxx
       // i64    signé         très grand interval
       // u8     non signé   de 0 à 255 


   //2. Les fonctions:
   
       // fn définit une fonction 
       // &str est de type chaine de caractère ( référence )
       // on créé une fonction addition qui retourne une somme et on l'appelle depuis le main 

        let  resultat = addition(12,3);

        println!("la somme de notre chere fonction est {}",resultat); 
        // appel de la fonction 
        say_hello("Salma"); 


     // 3.les conditions, les boucles 
     // if condition
     let nombre = 16;
     if nombre %2 == 0 {
        println!("{} est un nombre Pair",nombre);
     } else {
        println!("nombre Impair"); 
     }
     // boucle avec ou interval inclusif ..=
       for i in 1..=10{
        println!("i vaut {}",i); // jusqu'à 10 
       }
        // ou interval exclusif ..
        for j in 1..10{
        println!(" j vaut {}",j); // j jusqu'à 9
       }

       // Les tableaux itérer  sur un tableau simple 

       // pour déclarer un tableau 
       let voitures = ["citroen","renault","jeep"]; // Tableau à trois element

       println!("{}", voitures[1]);


       for voiture in voitures {
        println!("voiture : {},", voiture ); 
       }

       /* for ( index, valeur)  in collection.iter().enumerate(){
                 
                 // instruction : on utilise index et valeur ici on peut 
           
          } 
       
        */

        // je reprends l'exemple de voiture 

        for(i,voiture) in voitures.iter().enumerate(){
            println!("Index : {} : {}",i,voiture); 
        }

     // Les tableaux dynamiques  Vecteur ( vector)
     
       // exemple vect
        let noms = vec![String::from("Erwan"),String::from("Salma"), String::from("Pierre Emmanuel")];
       for(i,nom) in noms.iter().enumerate(){
        println!(" Nom {} : {} ", i, nom);
       }


       // Usage enumerate() dans un cas réel : Afficher un menu avec un numéro et choix


       let options = ["Afficher solde","Retrait","Listes des comptes","Quitter"];

       println!("Menu");

       for(i, option) in options.iter().enumerate(){
        // afficher chaque option et commencer par 1

           println!("{}.{}",i+1,option);
       }

       println!("Veuillez saisir un numéro de votre choix");
       let mut choix = String::new();

     io::stdin().read_line(&mut choix).expect("Attention erreur de lecture");
      let choix:usize = match choix.trim().parse(){
         Ok(num) => num,
         Err(_)=>{
            println!("Veuillez saisir un numero valide");
            return;
         }
      };

      if choix ==0 || choix > options.len(){
        println!(" choix hors systeme ");
      } else {
        println!("Vous avez choisi la rubrique :{}",options[choix-1]);
         // ici on peut exécuter une action selon le choix 
      }

    // 4. Les loops

    println!("###############__loop__################");

    let mut compteur = 0;
    loop{
        println!("Compteur:{}",compteur);
        compteur +=1;
        if compteur ==3{
            break; // on sort de la boucllz quand le compteur atteint 3 
        }
    }
     println!("###############__while__################");

     let mut compteur2 = 0; // mut pour changer la valeur de la variable par défaut immuable
     while compteur2 < 4{
        println!("Compteur2 :{}",compteur2);
        compteur2 +=1;
     }; 
 
   println!("############### Les structures struct ################");
   /*
              POO : programmation orienté objet -> classe => represente un objet => des propriétes et actions
                 de methode de classe et méthode d'instances 
                 => class Voiture {
                        couleur,
                        puissance,
                        annee
                        constructor()
                        freiner()
                        démarrer()
                 }
                        je crée une instance  mon_objet = new Voiture();
                        mon_objet1 = new Voiture();
                        mon_objet2 = new Voiture();
                        mon_objet2.couleur
                        mon_objet.couleur
                        mon_objet.puissance
                        mon_objet.freiner()
                        mon_objet().demarrer()
       => une classe une structure de référence ( on fait référence  à un objet )

              PP : programmation procédurale
              PF : programmation fonctionnelle 

              Rust est est procédurale avec des structures ( structurer les données)
   */

  // usage de struct : on crée le model/la structure 
   struct Salarie{
    nom: String,
    prenom:String,
    age:u32
   }
   // usage de struct => on crée une instance ou plusieurs de la structure 

   let salarie = Salarie{
    nom:String::from("Harbaoui"),
    prenom:String::from("Aymen"),
    age:40
   };

     let salarie1 = Salarie{
    nom:String::from("CONCHE"),
    prenom:String::from("Nicolas"),
    age:23
   };

        let salarie3 = Salarie{
    nom:String::from("HATTACH"),
    prenom:String::from("Salma"),
    age:23
   };
   // pour accéder aux attributs de la struture 
   println!("Nom: {}, Prénom:{}, Age:{}",salarie.nom,salarie.prenom,salarie.age);
   println!("Nom: {}, Prénom:{}, Age:{}",salarie1.nom,salarie1.prenom,salarie1.age);

   // On constate ici qu'on a pas de méthode ( fonctions associées aux structures)
     // dans ce cas 

     impl Salarie{
        fn afficher(&self){
            println!("le salarie suivant nom: {} prenom {} age: {} ,", self.nom,self.prenom, self.age);
        }
     }

      println!("*****Informations salariés*************"); 
     salarie1.afficher();
     salarie3.afficher(); 
   
}


// en C++/C     main.h  #include 'addition.h'   .cpp => main()
/*
     public  int addition( int a, int b ){
        return  a + b; 
     }
*/ 

     fn addition(a:i32,b:i32) -> i32 {  // retourne un entier et n'oublier pas d'enlever ; 
         a+b 
     }

     fn say_hello(nom :&str){
        println!("Bonjour, {}",nom); 
     }