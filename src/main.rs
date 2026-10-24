use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;
use rayon::prelude::*;

// --- Configuration ---
const TOTAL_NUMBERS: u128 = 1_000_000; // 200 Millions
const MIN_VAL: u32 = 000001;
const MAX_VAL: u32 = 999999;
const TOP_K: usize = 10; // Nous voulons les 10 plus fréquents
const NUM_WINNERS: usize = 1; // Le nombre de gagnants tirés au sort parmi le Top K

/// Effectue la génération des nombres et le comptage en parallèle
fn generate_and_count() -> HashMap<u32, usize> {
    (0..TOTAL_NUMBERS)
        .into_par_iter()
        .fold(
            // Identité : chaque thread commence avec une HashMap vide
            || HashMap::<u32, usize>::new(),
            // Fonction de mapping : le travail de chaque thread
            |mut local_counts, _i| {
                let mut rng = rand::thread_rng();
                let random_number: u32 = rng.gen_range(MIN_VAL..=MAX_VAL);
                
                *local_counts.entry(random_number).or_insert(0) += 1;
                
                local_counts
            },
        )
        .reduce(
            // Identity : le point de départ de la réduction
            || HashMap::<u32, usize>::new(),
            // Fonction de réduction : fusionner deux HashMaps
            |mut map1, map2| {
                for (key, count) in map2 {
                    *map1.entry(key).or_insert(0) += count;
                }
                map1
            },
        )
}

/// Trouve et retourne les K nombres les plus fréquents
fn calculate_top_k(counts: HashMap<u32, usize>, k: usize) -> Vec<(u32, usize)> {
    // 1. Convertir HashMap en vecteur de (nombre, compte)
    let mut sorted_counts: Vec<(u32, usize)> = counts.into_iter().collect();
    
    // 2. Trier le vecteur par le compte (le deuxième élément du tuple), en ordre décroissant.
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    // 3. Prendre les K premiers éléments
    sorted_counts.into_iter().take(k).collect()
}

/// Effectue le tirage au sort sur la liste des résultats Top K
fn perform_draw(top_k_results: Vec<(u32, usize)>) -> Vec<u32> {
    if top_k_results.is_empty() {
        return Vec::new();
    }

    let mut rng = rand::thread_rng();
    let mut results = top_k_results; // Clonage pour pouvoir le mélanger
    
    // Mélanger complètement la liste des Top K
    use rand::seq::SliceRandom;
    results.shuffle(&mut rng);

    // Prendre les NUM_WINNERS premiers éléments mélangés
    let winners: Vec<u32> = results.into_iter()
        .take(NUM_WINNERS)
        .map(|(number, _count)| number)
        .collect();
    
    winners
}

fn main() {
    println!("--- Démarrage de la génération et du comptage parallèle ---");
    println!("🧮Total de nombres à générer : {}", TOTAL_NUMBERS);
    println!("⭐ Les {} nombres les plus fréquents seront tirés au sort pour les {} prix.", TOP_K, NUM_WINNERS);

    // DÉBUT DU CHRONOMÈTRE
    let start_time = Instant::now();

    // 1. Génération et comptage
    let final_counts = generate_and_count();

    // FIN DU CHRONOMÈTRE de la génération et du comptage
    let time_counting = start_time.elapsed();

    // 2. Calcul du Top K
    let top_k_results = calculate_top_k(final_counts, TOP_K);

    // =================================================
    // AFFICHAGE DES RÉSULTATS ET TIRAGE AU SORT
    // =================================================

    // Affichage du Top K
    println!("\n========================================");
    println!("  📊 Top {} des fréquences (avant tirage)", TOP_K);
    
    if top_k_results.is_empty() {
        println!("Aucun nombre généré.");
    } else {
        for (rank, (number, count)) in top_k_results.iter().enumerate() {
            println!("  {}. Nombre: {:<5} | Répétitions: {}", rank + 1, number, count);
        }
    }
    println!("========================================");

    // 3. Effectuer le tirage au sort
    let winners = perform_draw(top_k_results);

    // Affichage des gagnants
    println!("\n🏆 🌟 RÉSULTATS DU TIRAGE AU SORT (GAGNANTS) 🌟 🏆");
    if winners.is_empty() {
        println!("Impossible de déterminer des gagnants.");
    } else {
        for (rank, winner_number) in winners.iter().enumerate() {
            println!("  🥇 GAGNANT #{}: Nombre sélectionné: {}", rank + 1, winner_number);
        }
    }
    println!("========================================");


    // Affichage du temps
    println!("\n--- Récapitulatif des performances ---");
    println!("⏱️ Temps total de génération et de comptage : {:?}", time_counting);
}
