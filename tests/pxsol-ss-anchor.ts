import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PxsolSsAnchor } from "../target/types/pxsol_ss_anchor";

describe("pxsol-ss-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.pxsolSsAnchor as Program<PxsolSsAnchor>;
  const provider = anchor.getProvider();
  const wallet = provider.wallet as anchor.Wallet;
  const walletPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("pxsol-ss-anchor"), wallet.publicKey.toBuffer()],
    program.programId
  )[0];
  console.log("walletPda:", walletPda.toBase58());

  it("Is initialized!", async () => {
    const poemInitial = Buffer.from("");
    const poemEnglish = Buffer.from("The quick brown fox jumps over the lazy dog");
    const poemChinese = Buffer.from("片云天共远, 永夜月同孤.");
    
    // Add your test here.
    let tx = await program.methods.initialize()
      .accounts({
        user: wallet.publicKey,
        // @ts-ignore
        userPda: walletPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();
    console.log("initialize transaction signature", tx);
    const walletPdaData = async (): Promise<Buffer<ArrayBuffer>> => {
      let walletPdaData = await program.account.data.fetch(walletPda);
      return Buffer.from(walletPdaData.data);
    }
    if (!(await walletPdaData()).equals(poemInitial)) throw new Error("mismatch");

    tx = await program.methods.update(poemEnglish)
      .accounts({ 
        user: wallet.publicKey,
        // @ts-ignore
        userPda: walletPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();
    if (!(await walletPdaData()).equals(poemEnglish)) throw new Error("mismatch");
    console.log("initialize transaction signature", tx);

    tx = await program.methods.update(poemChinese)
      .accounts({ 
        user: wallet.publicKey,
        // @ts-ignore
        userPda: walletPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();
    if (!(await walletPdaData()).equals(poemChinese)) throw new Error("mismatch");
    console.log("initialize transaction signature", tx);

  });
});
