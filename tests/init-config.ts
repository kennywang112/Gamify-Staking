import { executeTransaction, fetchIdlAccount } from "@cardinal/common";
import { 
  SystemProgram, 
  Transaction, 
  PublicKey, 
  Connection, 
  Commitment, 
  LAMPORTS_PER_SOL, 
  Keypair 
} from "@solana/web3.js";
import {
  findStakePoolId,
  SOL_PAYMENT_INFO,
} from "../sdk";
import { assert } from "chai";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import * as anchor from "@coral-xyz/anchor";
import { IDL } from "../target/types/cardinal_rewards_center";

import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

describe("Pool", () => {

    let stakePoolIdentifier = `test`;
    const options = anchor.AnchorProvider.defaultOptions();
    const commitment: Commitment = "processed";
    let connection = new Connection("http://localhost:8899", {
        commitment,
        wsEndpoint: "ws://localhost:8900/",
      });
    const wallet = NodeWallet.local();
    let provider = new anchor.AnchorProvider(connection, wallet, options);
    const programId = new PublicKey("5n4FXHbJHum7cW9w1bzYY8gdvgyC92Zk7yD2Qi9mW13g");
    let program = new anchor.Program(IDL, programId, provider);

    it("Init config", async () => {

        const tx = new Transaction();

        const configEntryId = PublicKey.findProgramAddressSync(
            [
                anchor.utils.bytes.utf8.encode("config-entry"), 
                Buffer.from("", "utf-8"), 
                Buffer.from(stakePoolIdentifier, "utf-8")
            ],
            programId
        )[0];

        const ix = await program.methods
        .initConfigEntry({
            prefix: Buffer.from(""),
            key: Buffer.from(stakePoolIdentifier),
            value: "value",
            extends: [],
        })
        .accountsStrict({
            configEntry: configEntryId,
            authority: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .instruction();
        tx.instructions.push(ix);
    
        await executeTransaction(provider.connection, tx, provider.wallet);

    });
})