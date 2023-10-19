import { executeTransaction, fetchIdlAccount, fetchIdlAccountDataById } from "@cardinal/common";
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
import { BN, utils } from "@coral-xyz/anchor";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

describe("Pool", () => {

    let discountIdentifier = `kikikiki`;
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

    it("Init discount", async () => {

        const discountId = PublicKey.findProgramAddressSync(
            [
              utils.bytes.utf8.encode("discount-prefix"),
              utils.bytes.utf8.encode(discountIdentifier),
            ],
            programId
        )[0];

        const tx = new Transaction();
        const ix = await program.methods
            .initDiscount({
                discountStr: 'discount',
                authority: provider.wallet.publicKey,
                identifier: discountIdentifier
            })
            .accounts({
                discountData: discountId,
                payer: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .instruction();
        tx.add(ix);
        // console.log(ix)
        await executeTransaction(provider.connection, tx, provider.wallet);

    });

    it("Init pool", async () => {

      const discountId = PublicKey.findProgramAddressSync(
        [
          utils.bytes.utf8.encode("discount-prefix"),
          utils.bytes.utf8.encode(discountIdentifier),
        ],
        programId
      )[0];
      const owner = Keypair.fromSecretKey(bs58.decode("2jgPdKQQE9fqgj8jtj6hESw8z7ibv7b6rQVxpPgxrTjGqyeq61uVcqGbm7JQ7egiD3cwFYbtPcQotyJEX9QbUXdv"))
      let balance = null;

      balance = await connection.getBalance(wallet.publicKey)
      console.log(balance/LAMPORTS_PER_SOL)

      const remain = [
        {
          pubkey: discountId,
          isSigner: false,
          isWritable: false,
        },
      ]

      const tx = new Transaction();
      const stakePoolId = findStakePoolId(discountIdentifier);
      const ix = await program.methods
          .initPool({
          identifier: discountIdentifier,
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
          payer: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          })
          .remainingAccounts(remain)
          .instruction();
      tx.add(ix);
      await executeTransaction(provider.connection, tx, provider.wallet);

      balance = await connection.getBalance(wallet.publicKey)
      console.log(balance/LAMPORTS_PER_SOL)
      
    });

    it("Update discount", async () => {

        const discountId = PublicKey.findProgramAddressSync(
            [
            utils.bytes.utf8.encode("discount-prefix"),
            utils.bytes.utf8.encode(discountIdentifier),
            ],
            programId
        )[0];

        const tx = new Transaction();
        const ix = await program.methods
            .updateDiscount({
                discountStr: 'discount 2',
                authority: provider.wallet.publicKey,
                identifier: discountIdentifier
            })
            .accounts({
                discountData: discountId,
                payer: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .instruction();
        tx.add(ix);
        await executeTransaction(provider.connection, tx, provider.wallet);

        const discount_data = await fetchIdlAccountDataById(
            connection,
            [discountId],
            programId,
            IDL
        )
        console.log(discount_data)
    });

})