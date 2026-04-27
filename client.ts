import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Tareas;

async function main() {
  const owner = provider.wallet.publicKey;

  const [usuarioPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("usuario"), owner.toBuffer()],
    program.programId
  );

  await program.methods
    .crearUsuario("David")
    .accounts({
      owner,
      usuario: usuarioPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log("Usuario creado");

  await program.methods
    .agregarTarea("Estudiar Solana")
    .accounts({
      owner,
      usuario: usuarioPDA,
    })
    .rpc();

  console.log("Tarea agregada");
}

main();
