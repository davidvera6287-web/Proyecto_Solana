import * as anchor from "@coral-xyz/anchor";

describe("tareas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Tareas;

  it("CRUD tareas", async () => {
    const usuario = provider.wallet.publicKey;

    const [usuarioPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("usuario"), usuario.toBuffer()],
      program.programId
    );

    await program.methods
      .crearUsuario("David")
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .agregarTarea("Hacer tarea")
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
      })
      .rpc();

    await program.methods
      .completarTarea(0)
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
      })
      .rpc();

    await program.methods
      .eliminarTarea(0)
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
      })
      .rpc();

    console.log("CRUD completado");
  });
});
