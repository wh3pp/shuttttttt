use crate::{error::Result, Context};
use poise::serenity_prelude as serenity;
use std::time::Duration;

#[poise::command(slash_command, prefix_command)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "The song title or artist to search for"] query: String,
) -> Result<()> {
    ctx.defer().await?;

    let response = ctx
        .data()
        .tunecore_client
        .creators()
        .songs()
        .per_page(50)
        .keyword(&query)
        .send()
        .await?;

    if response.community_songs.is_empty() {
        ctx.reply("No songs found matching that query.").await?;
        return Ok(());
    }

    // --- INICIO DE LA LÓGICA DE PAGINACIÓN MANUAL ---

    let page_size = 5;
    let pages_data = response.community_songs.chunks(page_size).collect::<Vec<_>>();
    let total_pages = pages_data.len();
    let mut current_page = 0;

    // Función para crear el embed de una página específica
    let create_embed = |page_index: usize| -> serenity::CreateEmbed {
        let chunk = pages_data[page_index];
        let description = chunk
            .iter()
            .enumerate()
            .map(|(chunk_index, song)| {
                let song_number = page_index * page_size + chunk_index + 1;
                let title = song.song_title.en.as_deref().unwrap_or(&song.song_title.ja);
                let artist = song.artist_name.en.as_deref().unwrap_or(&song.artist_name.ja);
                format!("`{song_number}.` **{title}** - *{artist}*")
            })
            .collect::<Vec<_>>()
            .join("\n");

        serenity::CreateEmbed::default()
            .title("🎶 Song Search Results")
            .description(description)
            .color(0x1DB954)
            .footer(serenity::CreateEmbedFooter::new(format!(
                "Page {}/{} ({} total songs)",
                page_index + 1,
                total_pages,
                response.total
            )))
    };

    // Define los botones
    let components = |page_index: usize| -> Vec<serenity::CreateActionRow> {
        let previous_button = serenity::CreateButton::new("previous")
            .label("Previous")
            .style(serenity::ButtonStyle::Primary)
            .disabled(page_index == 0);

        let next_button = serenity::CreateButton::new("next")
            .label("Next")
            .style(serenity::ButtonStyle::Primary)
            .disabled(page_index + 1 >= total_pages);
            
        vec![serenity::CreateActionRow::Buttons(vec![previous_button, next_button])]
    };

    // Envía el mensaje inicial con la primera página y los botones
    let initial_embed = create_embed(current_page);
    let initial_components = components(current_page);
    let reply_handle = ctx.send(
        poise::CreateReply::default()
            .embed(initial_embed)
            .components(initial_components)
    ).await?;
    
    // Bucle para escuchar las interacciones de los botones
    while let Some(interaction) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(120)) // Timeout de 2 minutos
        .filter(move |m| m.data.custom_id == "previous" || m.data.custom_id == "next")
        .await
    {
        // Actualiza el índice de la página según el botón presionado
        if interaction.data.custom_id == "previous" {
            current_page = current_page.saturating_sub(1);
        } else {
            current_page = (current_page + 1).min(total_pages - 1);
        }

        // Edita el mensaje original con el nuevo embed y los nuevos botones
        interaction.create_response(ctx, 
            serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new()
                    .embed(create_embed(current_page))
                    .components(components(current_page))
            )
        ).await?;
    }
    
    // (Opcional) Limpia los botones cuando el colector expira
    reply_handle.edit(ctx, poise::CreateReply::default()
        .embed(create_embed(current_page))
        .components(vec![]) // Envía una lista vacía de componentes
    ).await?;

    Ok(())
}
