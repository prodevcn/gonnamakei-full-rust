import {boot} from "quasar/wrappers";
import {Blockhash, clusterApiUrl, Connection, PublicKey} from "@solana/web3.js";
import {Wallets} from "boot/wallets";

export const LAMPORTS_IN_SOL = 1_000_000_000;
let _solanaConnection: Connection | null = null;

export const Solana = {
    // GETTERS ----------------------------------------------------------------

    get connection() {
        if (_solanaConnection !== null) {
            return _solanaConnection;
        }

        _solanaConnection = new Connection(clusterApiUrl(process.env.SOLANA_URL as any));

        return _solanaConnection;
    },

    // METHODS ----------------------------------------------------------------

    async selfBalance(): Promise<number> {
        return this.balance(Wallets.connectedWallet.value!.publicKey!);
    },

    async balance(address: PublicKey): Promise<number> {
        return this.connection.getBalance(address);
    },

    async selfAirdrop(lamports: number): Promise<void> {
        return this.airdrop(Wallets.connectedWallet.value!.publicKey!, lamports);
    },

    async airdrop(address: PublicKey, lamports: number): Promise<void> {
        let signature = await this.connection.requestAirdrop(address, lamports);
        await this.connection.confirmTransaction(signature);
    },

    async recentBlockHash(): Promise<Blockhash> {
        return (await this.connection.getRecentBlockhash()).blockhash;
    },

    async minimumBalanceForRentExemption(data: number): Promise<number> {
        return await this.connection.getMinimumBalanceForRentExemption(data);
    },

    solToLamports(sol: number): number {
        return Math.floor(sol * LAMPORTS_IN_SOL);
    },

    lamportsToSol(lamports: number): number {
        return Math.floor(lamports) / LAMPORTS_IN_SOL;
    },
};

export default boot(() => {
});