import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {UndeadArena} from "../target/types/undead_arena";
import testWallet from "../wallets/test.json";

describe("undead_arena", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.UndeadArena as Program<UndeadArena>;
    const kp = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(testWallet));

    it("Should start a match", async () => {
        // Add your test here.
        const tx = await program.methods
            .startMatch(new anchor.BN(0.01 * anchor.web3.LAMPORTS_PER_SOL))
            .accounts({
                vault: "Az9bZzW2197hmk1fjMWBQAjNjJWYqWPXL85dJjvtS4RE",
                player: kp.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId
            })
            //.rpc()
            .transaction();

        try {
            const hash = await program.provider.connection.sendTransaction(tx, [kp]);
            console.log(hash)
        } catch (err: unknown) {
            console.log(program.idl.errors[0].msg);
        }
    });
});
