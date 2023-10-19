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

    const owner = Keypair.fromSecretKey(bs58.decode("2jgPdKQQE9fqgj8jtj6hESw8z7ibv7b6rQVxpPgxrTjGqyeq61uVcqGbm7JQ7egiD3cwFYbtPcQotyJEX9QbUXdv"))

    let balance = null;

    it("Init pool", async () => {

        balance = await connection.getBalance(wallet.publicKey)
        console.log(balance/LAMPORTS_PER_SOL)

        const tx = new Transaction();
        const stakePoolId = findStakePoolId(stakePoolIdentifier);
        const discountId = PublicKey.findProgramAddressSync(
            [
                anchor.utils.bytes.utf8.encode("discount-prefix"),
                anchor.utils.bytes.utf8.encode(stakePoolIdentifier),
            ],
            programId
        )[0];
    
        const discountix = await program.methods
            .initDiscount({
                discountStr: "",
                authority: wallet.publicKey,
                identifier: stakePoolIdentifier
            })
            .accounts({
                discountData: discountId,
                nftMint: nftmint,
                nftTokenAccount: mint_ata,
                metadataAccount: metadataAccount,
                payer: wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .instruction();

            
        const remain = [
            {
                pubkey: discountId,
                isSigner: false,
                isWritable: false,
            },
        ]

        const ix = await program.methods
            .initPool({
            identifier: stakePoolIdentifier,
            allowedCollections: [],
            allowedCreators: [],
            requiresAuthorization: false,
            authority: provider.wallet.publicKey,
            resetOnUnstake: false,
            cooldownSeconds: null,
            minStakeSeconds: null,
            endDate: null,
            stakePaymentInfo: SOL_PAYMENT_INFO,
            unstakePaymentInfo: SOL_PAYMENT_INFO,
            })
            .accounts({
            owner: owner.publicKey,
            stakePool: stakePoolId,
            payer: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
            })
            .remainingAccounts()
            .instruction();
        tx.add(ix);
        await executeTransaction(provider.connection, tx, provider.wallet);

        balance = await connection.getBalance(wallet.publicKey)
        console.log(balance/LAMPORTS_PER_SOL)
        
    });

    it("Update", async () => {
        const tx = new Transaction();
        const stakePoolId = findStakePoolId(stakePoolIdentifier);
        const ix = await program.methods
            .updatePool({
            allowedCollections: [],
            allowedCreators: [],
            requiresAuthorization: true,
            authority: provider.wallet.publicKey,
            resetOnUnstake: false,
            cooldownSeconds: null,
            minStakeSeconds: null,
            endDate: null,
            stakePaymentInfo: SOL_PAYMENT_INFO,
            unstakePaymentInfo: SOL_PAYMENT_INFO,
            })
            .accounts({
            stakePool: stakePoolId,
            authority: provider.wallet.publicKey,
            payer: provider.wallet.publicKey,
            })
            .instruction();
        tx.add(ix);
        await executeTransaction(provider.connection, tx, provider.wallet);
        const pool = await fetchIdlAccount(
            provider.connection,
            stakePoolId,
            "stakePool",
            IDL,
        );
        const balance = await connection.getBalance(wallet.publicKey)

        assert.ok(pool.parsed.authority.toString() == provider.wallet.publicKey.toString())
        assert.ok(pool.parsed.requiresAuthorization == true)
    });
})