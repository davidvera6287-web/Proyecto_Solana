use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod tareas {
    use super::*;

    pub fn crear_usuario(ctx: Context<CrearUsuario>, nombre: String) -> Result<()> {
        let usuario = &mut ctx.accounts.usuario;
        usuario.owner = ctx.accounts.owner.key();
        usuario.nombre = nombre;
        usuario.tareas = Vec::new();
        Ok(())
    }

    pub fn agregar_tarea(ctx: Context<ModificarUsuario>, titulo: String) -> Result<()> {
        let usuario = &mut ctx.accounts.usuario;

        require!(
            usuario.owner == ctx.accounts.owner.key(),
            ErrorTarea::NoOwner
        );

        usuario.tareas.push(Tarea {
            titulo,
            completada: false,
        });

        Ok(())
    }

    pub fn completar_tarea(ctx: Context<ModificarUsuario>, index: u8) -> Result<()> {
        let usuario = &mut ctx.accounts.usuario;

        require!(
            usuario.owner == ctx.accounts.owner.key(),
            ErrorTarea::NoOwner
        );

        require!(
            (index as usize) < usuario.tareas.len(),
            ErrorTarea::NoExiste
        );

        usuario.tareas[index as usize].completada = true;

        Ok(())
    }

    pub fn eliminar_tarea(ctx: Context<ModificarUsuario>, index: u8) -> Result<()> {
        let usuario = &mut ctx.accounts.usuario;

        require!(
            usuario.owner == ctx.accounts.owner.key(),
            ErrorTarea::NoOwner
        );

        require!(
            (index as usize) < usuario.tareas.len(),
            ErrorTarea::NoExiste
        );

        usuario.tareas.remove(index as usize);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearUsuario<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 500,
        seeds = [b"usuario", owner.key().as_ref()],
        bump
    )]
    pub usuario: Account<'info, Usuario>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarUsuario<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub usuario: Account<'info, Usuario>,
}

#[account]
pub struct Usuario {
    pub owner: Pubkey,
    pub nombre: String,
    pub tareas: Vec<Tarea>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tarea {
    pub titulo: String,
    pub completada: bool,
}

#[error_code]
pub enum ErrorTarea {
    #[msg("No eres propietario")]
    NoOwner,

    #[msg("La tarea no existe")]
    NoExiste,
}
