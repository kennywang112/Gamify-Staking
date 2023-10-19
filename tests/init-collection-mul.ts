import { executeTransaction,  fetchIdlAccountDataById } from "@cardinal/common";
import { 
    SystemProgram, 
    Transaction, 
    PublicKey, 
    Connection, 
    Commitment 
} from "@solana/web3.js";
import {
  findStakePoolId
} from "../sdk";
import { assert } from "chai";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import * as anchor from "@coral-xyz/anchor";
import { IDL } from "../target/types/trading_train_center";
import { utils } from "@coral-xyz/anchor";

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

    let multiply_1 = anchor.web3.Keypair.generate();
    let multiply_2 = anchor.web3.Keypair.generate();

    it("Init collection mul", async() =>{
      
      const collectionmulId = PublicKey.findProgramAddressSync(
        [
          utils.bytes.utf8.encode('collection-mul'),
          utils.bytes.utf8.encode(stakePoolIdentifier),
        ],
        programId
      )[0];
      const stakePoolId = findStakePoolId(stakePoolIdentifier);

      const tx = new Transaction();
      const ix = await program.methods
      .initCollectionMul({
        collectionsMultiply:[multiply_1.publicKey, multiply_2.publicKey],
        multiplyData: [
          [1.5, 5], 
          [2, 1]
        ],
        identifier: stakePoolIdentifier,
        authority: provider.wallet.publicKey
      })
      .accounts({
        collectionMul: collectionmulId,
        stakePool: stakePoolId,
        payer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .instruction();

      tx.add(ix)
      await executeTransaction(provider.connection, tx, provider.wallet)

      const collectionmul = await fetchIdlAccountDataById(
        connection,
        [collectionmulId],
        programId,
        IDL
      )
      // console.log(collectionmul[Object.keys(collectionmul)[0]].parsed)

    })

    it("Update collection mul", async() =>{
    
      const collectionmulId = PublicKey.findProgramAddressSync(
        [
          utils.bytes.utf8.encode('collection-mul'),
          utils.bytes.utf8.encode(stakePoolIdentifier),
        ],
        programId
      )[0];
      const stakePoolId = findStakePoolId(stakePoolIdentifier);

      const tx = new Transaction();
      const ix = await program.methods
      .updateCollectionMul({
        collectionsMultiply:[multiply_1.publicKey, multiply_2.publicKey],
        multiplyData: [
          [1.5, 5], 
          [2, 1]
        ],
        identifier: stakePoolIdentifier,
        authority: provider.wallet.publicKey
      })
      .accounts({
        collectionMul: collectionmulId,
        stakePool: stakePoolId,
        payer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .instruction();

      tx.add(ix)
      await executeTransaction(provider.connection, tx, provider.wallet)

      const collectionmul = await fetchIdlAccountDataById(
        connection,
        [collectionmulId],
        programId,
        IDL
      )

    })
})