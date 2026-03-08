use anchor_lang::prelude::*;
declare_id!("CwSpGAU2L1219yNocMLrZWB2dgr4qySyb7j8dL79dduA");
#[program]
pub mod gaming {
    use super::*;
    pub fn crear_videojuego(ctx: Context<CrearVideojuego>, id: u64, titulo: String, genero: String, precio: u64) -> Result<()> {
        let v = &mut ctx.accounts.videojuego;
        v.id = id; v.owner = ctx.accounts.usuario.key();
        v.titulo = titulo; v.genero = genero; v.precio = precio; v.bump = ctx.bumps.videojuego;
        Ok(())
    }
    pub fn leer_videojuego(ctx: Context<LeerVideojuego>) -> Result<()> {
        let v = &ctx.accounts.videojuego;
        msg!("ID: {} Titulo: {} Genero: {} Precio: {}", v.id, v.titulo, v.genero, v.precio);
        Ok(())
    }
    pub fn actualizar_videojuego(ctx: Context<ActualizarVideojuego>, titulo: String, genero: String, precio: u64) -> Result<()> {
        let v = &mut ctx.accounts.videojuego;
        v.titulo = titulo; v.genero = genero; v.precio = precio;
        Ok(())
    }
    pub fn eliminar_videojuego(_ctx: Context<EliminarVideojuego>) -> Result<()> { Ok(()) }
}
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CrearVideojuego<'info> {
    #[account(init, payer = usuario, space = 8+8+32+64+64+8+1, seeds = [b"videojuego", usuario.key().as_ref(), &id.to_le_bytes()], bump)]
    pub videojuego: Account<'info, Videojuego>,
    #[account(mut)] pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct LeerVideojuego<'info> {
    #[account(seeds = [b"videojuego", usuario.key().as_ref(), &videojuego.id.to_le_bytes()], bump = videojuego.bump)]
    pub videojuego: Account<'info, Videojuego>,
    pub usuario: Signer<'info>,
}
#[derive(Accounts)]
pub struct ActualizarVideojuego<'info> {
    #[account(mut, seeds = [b"videojuego", usuario.key().as_ref(), &videojuego.id.to_le_bytes()], bump = videojuego.bump)]
    pub videojuego: Account<'info, Videojuego>,
    pub usuario: Signer<'info>,
}
#[derive(Accounts)]
pub struct EliminarVideojuego<'info> {
    #[account(mut, close = usuario, seeds = [b"videojuego", usuario.key().as_ref(), &videojuego.id.to_le_bytes()], bump = videojuego.bump)]
    pub videojuego: Account<'info, Videojuego>,
    #[account(mut)] pub usuario: Signer<'info>,
}
#[account]
pub struct Videojuego {
    pub id: u64, pub owner: Pubkey, pub titulo: String, pub genero: String, pub precio: u64, pub bump: u8,
}
