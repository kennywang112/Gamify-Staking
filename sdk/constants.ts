import type { IdlAccountData as cIdlAccountData } from "@cardinal/common";
import { emptyWallet } from "@cardinal/common";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import type { AllAccountsMap } from "@coral-xyz/anchor/dist/cjs/program/namespace/types";
import type { Wallet } from "@coral-xyz/anchor/dist/cjs/provider";
import type { ConfirmOptions, Connection } from "@solana/web3.js";
import { Keypair, PublicKey } from "@solana/web3.js";

import type { CardinalRewardsCenter } from "./idl/cardinal_rewards_center";
import { IDL } from "./idl/cardinal_rewards_center";

export const REWARDS_CENTER_IDL = IDL;

export const REWARDS_CENTER_ADDRESS = new PublicKey(
  "5n4FXHbJHum7cW9w1bzYY8gdvgyC92Zk7yD2Qi9mW13g"
);

export const WRAPPED_SOL_PAYMENT_INFO = new PublicKey(
  "5vsKac5CuyHnK1A19Vs4ChvBV4PreHKKvgmACA6KyGdi"
);

export const SOL_PAYMENT_INFO = new PublicKey(
  "5vsKac5CuyHnK1A19Vs4ChvBV4PreHKKvgmACA6KyGdi"
);

export const DEFAULT_PAYMENT_INFO = new PublicKey(
  "5vsKac5CuyHnK1A19Vs4ChvBV4PreHKKvgmACA6KyGdi"
);

export type IdlAccountData<
  T extends keyof AllAccountsMap<CardinalRewardsCenter>
> = cIdlAccountData<T, CardinalRewardsCenter>;
export type RewardDistributor = IdlAccountData<"rewardDistributor">;
export type RewardEntry = IdlAccountData<"rewardEntry">;
export type StakePool = IdlAccountData<"stakePool">;
export type StakeEntry = IdlAccountData<"stakeEntry">;
export type ReceiptManager = IdlAccountData<"receiptManager">;
export type RewardReceipt = IdlAccountData<"rewardReceipt">;
export type StakeBooster = IdlAccountData<"stakeBooster">;
export type StakeAuthorizationRecord =
  IdlAccountData<"stakeAuthorizationRecord">;
export type PaymentInfo = IdlAccountData<"paymentInfo">;

export type PaymentShare = {
  address: PublicKey;
  basisPoints: number;
};

export const rewardsCenterProgram = (
  connection: Connection,
  wallet?: Wallet,
  opts?: ConfirmOptions
) => {
  return new Program<CardinalRewardsCenter>(
    REWARDS_CENTER_IDL,
    REWARDS_CENTER_ADDRESS,
    new AnchorProvider(
      connection,
      wallet ?? emptyWallet(Keypair.generate().publicKey),
      opts ?? {}
    )
  );
};
