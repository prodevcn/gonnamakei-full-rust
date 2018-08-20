import {PublicKey, Transaction} from "@solana/web3.js";
import {Ref} from "vue";

export interface WalletAdapter {
    key: string,
    name: string,
    url: string,
    icon: string,
    isInstalled: boolean,
    isConnecting: Ref<boolean>,
    isConnected: Ref<boolean>,
    publicKey: PublicKey | null,

    connect(): Promise<void>,

    connectSilently(): Promise<void>,

    disconnect(): Promise<void>,

    signMessage(message: string): Promise<string>,

    signTransaction(transaction: Transaction): Promise<Transaction>
}