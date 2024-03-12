import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestCell } from "../target/types/test_cell";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

describe("test-cell", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TestCell as Program<TestCell>;

  const userKeypair = Keypair.generate();

  const userPDA = findProgramAddressSync([anchor.utils.bytes.utf8.encode("Dummy")], program.programId);

  it("Airdrop 2 SOL to the user Keypair", async () => {
    // Add your test here.
    const airdropTx = await provider.connection.requestAirdrop(userKeypair.publicKey, 2 * LAMPORTS_PER_SOL);
    let latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropTx,
    });

    console.log("\n\nAirdrop Successful - TxID: ", airdropTx);
  });

  it("Initialize User Grid Account", async() => {
    const accountTx = await program.methods.initialize().accounts({
      account: userPDA[0],
      user: userKeypair.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([userKeypair]).rpc();

    console.log("\n\nUser grid account initialized! TxID: ", accountTx);
  })

  it("Add player 1 to the grid game", async() => {
    const addPlaterTx = await program.methods.addPlayer().accounts({
      account: userPDA[0],
      user: provider.publicKey,
    }).rpc();

    console.log("\n\nPlayer ", provider.publicKey.toBase58(), " Sucessfully added to the grid game! TxID: ", addPlaterTx);
  })

  it("Add player 2 to the grid game", async() => {
    const addPlaterTx = await program.methods.addPlayer().accounts({
      account: userPDA[0],
      user: userKeypair.publicKey,
    }).signers([userKeypair]).rpc();

    console.log("\n\nPlayer ", userKeypair.publicKey.toBase58(), " Sucessfully added to the grid game! TxID: ", addPlaterTx);
  })

  it("Find player in the grid game", async() => {
    const findPlayerTx = await program.methods.findPlayer().accounts({
      account: userPDA[0],
      user: provider.publicKey,
    }).rpc();

    console.log("\n\nFind User Operation Completed! TxID: ", findPlayerTx);
  })

  it("Move player in the Y axies", async() => {
    const moveTx = await program.methods.movePlayerY().accounts({
      account: userPDA[0],
      user: userKeypair.publicKey,
    }).signers([userKeypair]).rpc();

    console.log("\n\nUser Move Operation (Y Axies) Completed! TxID: ", moveTx);
  })

  it("Move player in the X axies", async() => {
    const moveTx = await program.methods.movePlayerX().accounts({
      account: userPDA[0],
      user: userKeypair.publicKey,
    }).signers([userKeypair]).rpc();

    console.log("\n\nUser Move Operation (X Axies) Completed! TxID: ", moveTx);
  })

  it("Find and attack players near Player 1", async() => {
    const findPlayerTx = await program.methods.checkSurrondings().accounts({
      account: userPDA[0],
      user: provider.publicKey,
    }).rpc();

    console.log("\n\nFind And Attack Nearby Players Function Completed! TxID: ", findPlayerTx);
  })

  it("Find and attack players near Player 2", async() => {
    const findPlayerTx = await program.methods.checkSurrondings().accounts({
      account: userPDA[0],
      user: userKeypair.publicKey,
    }).signers([userKeypair]).rpc();

    console.log("\n\nFind And Attack Nearby Players Function Completed! TxID: ", findPlayerTx);
  })

  it("Find and attack players near Player 2", async() => {
    const findPlayerTx = await program.methods.checkSurrondings().accounts({
      account: userPDA[0],
      user: userKeypair.publicKey,
    }).signers([userKeypair]).rpc();

    console.log("\n\nFind And Attack Nearby Players Function Completed! TxID: ", findPlayerTx);
  })

  it("Find and attack players near Player 2", async() => {
    const findPlayerTx = await program.methods.checkSurrondings().accounts({
      account: userPDA[0],
      user: userKeypair.publicKey,
    }).signers([userKeypair]).rpc();

    console.log("\n\nFind And Attack Nearby Players Function Completed! TxID: ", findPlayerTx);
  })
});
