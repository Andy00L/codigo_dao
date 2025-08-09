import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("Governance Flows Unit", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaReputationDao as Program;

  it("Create realm and update algorithm", async () => {
    const admin = anchor.web3.Keypair.generate();
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey, 2e9),
      "confirmed"
    );

    const name = "realm-unit";
    const [realm] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("realm"), Buffer.from(name)],
      program.programId
    );

    await program.methods
      .createRealm(name, [100, 100, 100, 100, 100])
      .accounts({
        realm,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await program.methods
      .updateAlgorithm([200, 150, 120, 130, 110], 3, true, 12)
      .accounts({
        realm,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    const realmAcc: any = await program.account.governanceRealm.fetch(realm);
    expect(realmAcc.reputationAlgorithm.communityWeight).to.equal(120);
  });
});
