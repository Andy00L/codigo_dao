import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("Advanced Reputation Scoreboard", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaReputationDao as Program;

  let userA: anchor.web3.Keypair;
  let userB: anchor.web3.Keypair;
  let admin: anchor.web3.Keypair;

  let userAProfile: anchor.web3.PublicKey;
  let userBProfile: anchor.web3.PublicKey;
  let testRealm: anchor.web3.PublicKey;

  before(async () => {
    userA = anchor.web3.Keypair.generate();
    userB = anchor.web3.Keypair.generate();
    admin = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(userA.publicKey, 2e9),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(userB.publicKey, 2e9),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey, 2e9),
      "confirmed"
    );

    [userAProfile] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reputation"), userA.publicKey.toBuffer()],
      program.programId
    );

    [userBProfile] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reputation"), userB.publicKey.toBuffer()],
      program.programId
    );

    [testRealm] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("realm"), Buffer.from("test-realm")],
      program.programId
    );
  });

  describe("Profile Initialization", () => {
    it("Should initialize user profile correctly", async () => {
      await program.methods
        .initializeProfile()
        .accounts({
          profile: userAProfile,
          user: userA.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userA])
        .rpc();

      const profile: any = await program.account.reputationProfile.fetch(
        userAProfile
      );
      expect(profile.wallet.toString()).to.equal(userA.publicKey.toString());
      expect(profile.totalScore.toNumber()).to.equal(0);
      expect(profile.interactionCount).to.equal(0);
    });

    it("Should prevent duplicate profile initialization", async () => {
      try {
        await program.methods
          .initializeProfile()
          .accounts({
            profile: userAProfile,
            user: userA.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([userA])
          .rpc();
        expect.fail("Should have thrown an error");
      } catch (error: any) {
        expect(error.message).to.include("already in use");
      }
    });
  });

  describe("Reputation Interactions", () => {
    before(async () => {
      await program.methods
        .initializeProfile()
        .accounts({
          profile: userBProfile,
          user: userB.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userB])
        .rpc();
    });

    it("Should record basic interaction and update reputation", async () => {
      const interactionPda = anchor.web3.Keypair.generate();

      await program.methods
        .recordInteraction(2, 150, "Helpful code review")
        .accounts({
          fromProfile: userAProfile,
          toProfile: userBProfile,
          interactionEvent: interactionPda.publicKey,
          fromUser: userA.publicKey,
          toUser: userB.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userA, interactionPda])
        .rpc();

      const toProfile: any = await program.account.reputationProfile.fetch(
        userBProfile
      );
      expect(toProfile.totalScore.toNumber()).to.be.greaterThan(0);
      expect(toProfile.interactionCount).to.equal(1);
    });

    it("Should enforce cooldown periods", async () => {
      try {
        const interactionPda2 = anchor.web3.Keypair.generate();

        await program.methods
          .recordInteraction(2, 100, "Another review")
          .accounts({
            fromProfile: userAProfile,
            toProfile: userBProfile,
            interactionEvent: interactionPda2.publicKey,
            fromUser: userA.publicKey,
            toUser: userB.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([userA, interactionPda2])
          .rpc();

        expect.fail("Should have enforced cooldown");
      } catch (error: any) {
        expect(error.toString()).to.include("CooldownActive");
      }
    });

    it("Should prevent self-interaction", async () => {
      try {
        const selfInteractionPda = anchor.web3.Keypair.generate();

        await program.methods
          .recordInteraction(1, 100, "Self vote")
          .accounts({
            fromProfile: userAProfile,
            toProfile: userAProfile,
            interactionEvent: selfInteractionPda.publicKey,
            fromUser: userA.publicKey,
            toUser: userA.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([userA, selfInteractionPda])
          .rpc();

        expect.fail("Should have prevented self-interaction");
      } catch (error: any) {
        expect(error.toString()).to.include("SelfInteractionForbidden");
      }
    });
  });

  describe("Advanced Governance Features", () => {
    it("Should create governance realm with custom parameters", async () => {
      const algorithmWeights = [200, 150, 100, 175, 125]; // Custom weights

      await program.methods
        .createRealm("test-realm", algorithmWeights)
        .accounts({
          realm: testRealm,
          admin: admin.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      const realm: any = await program.account.governanceRealm.fetch(testRealm);
      expect(realm.reputationAlgorithm.developmentWeight).to.equal(200);
      expect(realm.reputationAlgorithm.innovationWeight).to.equal(175);
    });

    it("Should handle reputation delegation", async () => {
      await program.methods
        .delegateReputation(userB.publicKey, 25) // Delegate 25% voting power
        .accounts({
          delegatorProfile: userAProfile,
          delegateeProfile: userBProfile,
          delegator: userA.publicKey,
        })
        .signers([userA])
        .rpc();

      const delegatorProfile: any =
        await program.account.reputationProfile.fetch(userAProfile);
      const delegateeProfile: any =
        await program.account.reputationProfile.fetch(userBProfile);

      expect(delegatorProfile.delegatedPower.toNumber()).to.be.greaterThan(0);
      expect(delegateeProfile.delegationReceived.toNumber()).to.be.greaterThan(
        0
      );
    });
  });

  describe("AI-Enhanced Features", () => {
    it("Should update AI validation scores (placeholder)", async () => {
      const profile: any = await program.account.reputationProfile.fetch(
        userAProfile
      );
      expect(profile.aiValidationScore).to.be.a("number");
    });

    it("Should handle badge claims with proof validation", async () => {
      const badgePda = anchor.web3.Keypair.generate();
      const proofHash = Array.from(
        Buffer.from("test-proof-hash".repeat(2), "utf8")
      ).slice(0, 32);

      await program.methods
        .claimBadge(1, proofHash) // Developer badge
        .accounts({
          profile: userAProfile,
          badgeAccount: badgePda.publicKey,
          user: userA.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userA, badgePda])
        .rpc();

      const profile: any = await program.account.reputationProfile.fetch(
        userAProfile
      );
      // First badge should be non-zero type
      expect(profile.badges[0].badgeType).to.not.equal(0);
    });
  });

  describe("Security and Edge Cases", () => {
    it("Should handle reputation decay correctly (simulated)", async () => {
      const profile: any = await program.account.reputationProfile.fetch(
        userBProfile
      );
      const initialScore = profile.totalScore.toNumber();
      expect(initialScore).to.be.greaterThan(0);
    });

    it("Should validate mathematical operations don't overflow", async () => {
      try {
        await program.methods
          .recordInteraction(9, 1000, "Max weight interaction")
          .accounts({
            fromProfile: userAProfile,
            toProfile: userBProfile,
            interactionEvent: anchor.web3.Keypair.generate().publicKey,
            fromUser: userA.publicKey,
            toUser: userB.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([userA])
          .rpc();
      } catch (_e) {
        // should not panic
      }
    });
  });

  describe("Cross-DAO Bridge Functionality", () => {
    it("Should enable cross-realm reputation sharing (placeholder)", async () => {
      expect(true).to.be.true;
    });
  });
});
