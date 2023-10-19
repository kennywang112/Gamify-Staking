"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.withRemainingAccounts = exports.withRemainingAccountsForPayment = exports.withRemainingAccountsForPaymentInfoSync = exports.withRemainingAccountsForPaymentInfo = exports.BASIS_POINTS_DIVISOR = void 0;
const spl_token_1 = require("@solana/spl-token");
const web3_js_1 = require("@solana/web3.js");
const accounts_1 = require("./accounts");
exports.BASIS_POINTS_DIVISOR = 10000;
const withRemainingAccountsForPaymentInfo = async (connection, transaction, payer, paymentInfo) => {
    const paymentInfoData = await (0, accounts_1.fetchIdlAccount)(connection, paymentInfo, "paymentInfo");
    const remainingAccounts = [
        {
            pubkey: paymentInfo,
            isSigner: false,
            isWritable: false,
        },
    ];
    // add payer
    if (Number(paymentInfoData.parsed.paymentAmount) === 0)
        return remainingAccounts;
    remainingAccounts.push(...(0, exports.withRemainingAccountsForPayment)(transaction, payer, paymentInfoData.parsed.paymentMint, paymentInfoData.parsed.paymentShares.map((p) => p.address)));
    return remainingAccounts;
};
exports.withRemainingAccountsForPaymentInfo = withRemainingAccountsForPaymentInfo;
const withRemainingAccountsForPaymentInfoSync = (transaction, payer, paymentInfoData) => {
    const remainingAccounts = [
        {
            pubkey: paymentInfoData.pubkey,
            isSigner: false,
            isWritable: false,
        },
    ];
    // add payer
    if (Number(paymentInfoData.parsed.paymentAmount) === 0)
        return remainingAccounts;
    remainingAccounts.push(...(0, exports.withRemainingAccountsForPayment)(transaction, payer, paymentInfoData.parsed.paymentMint, paymentInfoData.parsed.paymentShares.map((p) => p.address)));
    return remainingAccounts;
};
exports.withRemainingAccountsForPaymentInfoSync = withRemainingAccountsForPaymentInfoSync;
const withRemainingAccountsForPayment = (transaction, payer, paymentMint, paymentTargets) => {
    const remainingAccounts = [
        {
            pubkey: payer,
            isSigner: true,
            isWritable: true,
        },
    ];
    if (paymentMint.equals(web3_js_1.PublicKey.default)) {
        remainingAccounts.push({
            pubkey: web3_js_1.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        });
        remainingAccounts.push(...paymentTargets.map((a) => ({
            pubkey: a,
            isSigner: false,
            isWritable: true,
        })));
    }
    else {
        remainingAccounts.push({
            pubkey: spl_token_1.TOKEN_PROGRAM_ID,
            isSigner: false,
            isWritable: false,
        });
        remainingAccounts.push({
            pubkey: (0, spl_token_1.getAssociatedTokenAddressSync)(paymentMint, payer, true),
            isSigner: false,
            isWritable: true,
        });
        const ataIds = paymentTargets.map((a) => (0, spl_token_1.getAssociatedTokenAddressSync)(paymentMint, a, true));
        for (let i = 0; i < ataIds.length; i++) {
            transaction.add((0, spl_token_1.createAssociatedTokenAccountIdempotentInstruction)(payer, ataIds[i], paymentTargets[i], paymentMint));
        }
        remainingAccounts.push(...ataIds.map((id) => ({
            pubkey: id,
            isSigner: false,
            isWritable: true,
        })));
    }
    return remainingAccounts;
};
exports.withRemainingAccountsForPayment = withRemainingAccountsForPayment;
const withRemainingAccounts = (instruction, remainingAccounts) => {
    return {
        ...instruction,
        keys: [...instruction.keys, ...remainingAccounts],
    };
};
exports.withRemainingAccounts = withRemainingAccounts;
//# sourceMappingURL=payment.js.map