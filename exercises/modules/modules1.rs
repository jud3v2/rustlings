// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

// I AM DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // le mot clé pub permet de déclarer la fonction
    // en tant que fonction public car les fonctions 
    // déclarer en rust sont privée par défaut
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
