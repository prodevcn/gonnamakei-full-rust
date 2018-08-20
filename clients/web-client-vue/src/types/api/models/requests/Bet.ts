import {Blockhash} from "@solana/web3.js";

export interface BetSendRequestBody {
    signature: string,
    recentBlockHash: Blockhash,
}