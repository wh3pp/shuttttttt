use database::Db;
use std::io;
use tunecore::CreatorsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Db::connect("mongodb://localhost:27017").await?;
    loop {
        println!("\n--- MENU PRINCIPAL ---");
        println!("1. Buscar y guardar nuevas canciones (por pagina)");
        println!("2. Ver canciones guardadas");
        println!("3. Limpiar base de datos (borrar todo)");
        println!("4. Salir");
        println!("---------------------");
        print!("Elige una opcion: ");

        let choice = read_user_input_as_number()?;

        match choice {
            1 => scrape_and_save_paginated(&db).await?,
            2 => view_saved_songs(&db).await?,
            3 => clean_database(&db).await?,
            4 => {
                println!("Saliendo");
                break;
            }
            _ => {
                println!("Opcion no valida. Elige un numero del menu");
            }
        }
    }

    Ok(())
}

async fn scrape_and_save_paginated(db: &Db) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- BuSQUEDA POR PAGINAS ---");
    print!("Pagina de inicio: ");
    let start_page = read_user_input_as_number()?;

    print!("Pagina final: ");
    let end_page = read_user_input_as_number()?;

    if start_page > end_page {
        println!("La pagina de inicio no puede ser mayor que la pagina final");
        return Ok(());
    }

    let client = CreatorsClient::new();
    let mut total_found = 0;

    for page in start_page..=end_page {
        println!("\nBuscando en la pagina {}...", page);
        let response = client.scrape().page(page).per_page(100).send().await?;

        if response.community_songs.is_empty() {
            println!("No se encontraron canciones en esta pagina");
            continue;
        }

        let count = response.community_songs.len();
        total_found += count;
        println!("Se encontraron {} canciones. Guardando...", count);
        db.save_songs(&response.community_songs).await?;
    }

    println!(
        "\nBusqueda completada. Se guardaron un total de {} canciones",
        total_found
    );
    Ok(())
}

async fn view_saved_songs(db: &Db) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- CANCIONES GUARDADAS ---");
    let songs = db.get_all_songs().await?;

    if songs.is_empty() {
        println!("No hay ninguna cancion en la base de datos");
    } else {
        println!("Total de canciones en la BD: {}", songs.len());
        for (i, song) in songs.iter().enumerate() {
            println!(
                "{}. Titulo: {} | Artista: {}",
                i + 1,
                song.song_title.en,
                song.artist_name.en
            );
        }
    }
    Ok(())
}

async fn clean_database(db: &Db) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- LIMPIAR BASE DE DATOS ---");
    print!("¿Estas seguro de que quieres borrar TODAS las canciones? (s/N): ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;

    if confirmation.trim().eq_ignore_ascii_case("s") {
        println!("Borrando canciones...");
        let deleted_count = db.delete_all_songs().await?;
        println!("Se borraron {} canciones", deleted_count);
    } else {
        println!("Operacion cancelada");
    }

    Ok(())
}

fn read_user_input_as_number() -> Result<u32, Box<dyn std::error::Error>> {
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => {
            println!("Entrada no valida. Por favor, introduce un numero");
            Ok(0)
        }
    }
}

