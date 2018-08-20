import {Injectable} from "@angular/core";
import * as web3 from "@solana/web3.js";
import {Blockhash, PublicKey} from "@solana/web3.js";
import {environment} from "../../../environments/environment";
import {GMIWallets} from "../wallet";

const LAMPORTS_IN_SOL = 1_000_000_000;

@Injectable({
    providedIn: "root",
})
export class SolanaService {
    connection: web3.Connection;

    constructor(private wallets: GMIWallets) {
        this.connection = new web3.Connection(environment.SOLANA_CLUSTER as any);
    }

    async selfBalance(): Promise<number> {
        return this.balance(new PublicKey(this.wallets.connectedWallet?.address));
    }

    async balance(address: PublicKey): Promise<number> {
        return this.connection.getBalance(address);
    }

    async selfAirdrop(lamports: number): Promise<void> {
        return this.airdrop(new PublicKey(this.wallets.connectedWallet?.address), lamports);
    }

    async airdrop(address: PublicKey, lamports: number): Promise<void> {
        let signature = await this.connection.requestAirdrop(address, lamports);
        await this.connection.confirmTransaction(signature);
    }

    async recentBlockhash(): Promise<Blockhash> {
        return (await this.connection.getRecentBlockhash()).blockhash;
    }

    async minimumBalanceForRentExemption(data: number): Promise<number> {
        return await this.connection.getMinimumBalanceForRentExemption(data);
    }

    solToLamports(sol: number): number {
        return Math.floor(sol * LAMPORTS_IN_SOL);
    }

    lamportsToSol(lamports: number): number {
        return Math.floor(lamports) / LAMPORTS_IN_SOL;
    }
}
