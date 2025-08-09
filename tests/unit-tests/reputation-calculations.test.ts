import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("Reputation Calculations Unit", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaReputationDao as Program;

  it("Initialize profiles and run a high-impact interaction", async () => {
    const a = anchor.web3.Keypair.generate();
    const b = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(a.publicKey, 2e9);
    await provider.connection.requestAirdrop(b.publicKey, 2e9);

    const [aP] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reputation"), a.publicKey.toBuffer()],
      program.programId
    );
    const [bP] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reputation"), b.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeProfile()
      .accounts({
        profile: aP,
        user: a.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([a])
      .rpc();

    await program.methods
      .initializeProfile()
      .accounts({
        profile: bP,
        user: b.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([b])
      .rpc();

    const evt = anchor.web3.Keypair.generate();
    await program.methods
      .recordInteraction(7, 200, "Major innovation")
      .accounts({
        fromProfile: aP,
        toProfile: bP,
        interactionEvent: evt.publicKey,
        fromUser: a.publicKey,
        toUser: b.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([a, evt])
      .rpc();

    const bAcc: any = await program.account.reputationProfile.fetch(bP);
    expect(bAcc.totalScore.toNumber()).to.be.greaterThan(0);
    expect(bAcc.categoryScores[3].toNumber()).to.be.greaterThan(0); // innovation cat
  });
});
