import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("Security Validations Unit", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaReputationDao as Program;

  it("Prevents self interactions", async () => {
    const a = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(a.publicKey, 2e9);

    const [aP] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reputation"), a.publicKey.toBuffer()],
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

    const evt = anchor.web3.Keypair.generate();
    try {
      await program.methods
        .recordInteraction(0, 100, "Self")
        .accounts({
          fromProfile: aP,
          toProfile: aP,
          interactionEvent: evt.publicKey,
          fromUser: a.publicKey,
          toUser: a.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([a, evt])
        .rpc();
      expect.fail("Should have failed");
    } catch (e: any) {
      expect(e.toString()).to.include("SelfInteractionForbidden");
    }
  });
});
