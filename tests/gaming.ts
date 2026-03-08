import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Gaming } from "../target/types/gaming";

describe("gaming", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.gaming as Program<Gaming>;
  const usuario = anchor.AnchorProvider.env().wallet;

  const id = new anchor.BN(1);
  const titulo = "The Legend of Zelda";
  const genero = "Aventura";
  const precio = new anchor.BN(5000);

  const [videojuegoPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("videojuego"), usuario.publicKey.toBuffer(), id.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  it("CREATE - Crear videojuego", async () => {
    const tx = await program.methods
      .crearVideojuego(id, titulo, genero, precio)
      .accounts({ videojuego: videojuegoPDA, usuario: usuario.publicKey, systemProgram: anchor.web3.SystemProgram.programId })
      .rpc();
    console.log("Creado! TX:", tx);
    const cuenta = await program.account.videojuego.fetch(videojuegoPDA);
    console.log("ID:", cuenta.id.toString());
    console.log("Titulo:", cuenta.titulo);
    console.log("Precio:", cuenta.precio.toString());
  });

  it("READ - Leer videojuego", async () => {
    const tx = await program.methods.leerVideojuego()
      .accounts({ videojuego: videojuegoPDA, usuario: usuario.publicKey })
      .rpc();
    console.log("Leido! TX:", tx);
  });

  it("UPDATE - Actualizar videojuego", async () => {
    const tx = await program.methods
      .actualizarVideojuego("Zelda Actualizado", "RPG", new anchor.BN(9999))
      .accounts({ videojuego: videojuegoPDA, usuario: usuario.publicKey })
      .rpc();
    console.log("Actualizado! TX:", tx);
    const cuenta = await program.account.videojuego.fetch(videojuegoPDA);
    console.log("Nuevo titulo:", cuenta.titulo);
    console.log("Nuevo precio:", cuenta.precio.toString());
  });

  it("DELETE - Eliminar videojuego", async () => {
    const tx = await program.methods.eliminarVideojuego()
      .accounts({ videojuego: videojuegoPDA, usuario: usuario.publicKey })
      .rpc();
    console.log("Eliminado por ID:", id.toString(), "TX:", tx);
  });
});
