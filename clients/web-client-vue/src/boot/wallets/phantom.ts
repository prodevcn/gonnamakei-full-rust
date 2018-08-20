import {ref} from "vue";
import {PublicKey, Transaction} from "@solana/web3.js";

declare global {
    interface Window {
        solana: any;
    }
}

export class PhantomWallet {
    key: "phantom" = "phantom";
    name = "Phantom";
    url = "https://www.phantom.app";
    icon = "data:image/svg+xml;base64,PHN2ZyBmaWxsPSJub25lIiBoZWlnaHQ9IjM0IiB3aWR0aD0iMzQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGxpbmVhckdyYWRpZW50IGlkPSJhIiB4MT0iLjUiIHgyPSIuNSIgeTE9IjAiIHkyPSIxIj48c3RvcCBvZmZzZXQ9IjAiIHN0b3AtY29sb3I9IiM1MzRiYjEiLz48c3RvcCBvZmZzZXQ9IjEiIHN0b3AtY29sb3I9IiM1NTFiZjkiLz48L2xpbmVhckdyYWRpZW50PjxsaW5lYXJHcmFkaWVudCBpZD0iYiIgeDE9Ii41IiB4Mj0iLjUiIHkxPSIwIiB5Mj0iMSI+PHN0b3Agb2Zmc2V0PSIwIiBzdG9wLWNvbG9yPSIjZmZmIi8+PHN0b3Agb2Zmc2V0PSIxIiBzdG9wLWNvbG9yPSIjZmZmIiBzdG9wLW9wYWNpdHk9Ii44MiIvPjwvbGluZWFyR3JhZGllbnQ+PGNpcmNsZSBjeD0iMTciIGN5PSIxNyIgZmlsbD0idXJsKCNhKSIgcj0iMTciLz48cGF0aCBkPSJtMjkuMTcwMiAxNy4yMDcxaC0yLjk5NjljMC02LjEwNzQtNC45NjgzLTExLjA1ODE3LTExLjA5NzUtMTEuMDU4MTctNi4wNTMyNSAwLTEwLjk3NDYzIDQuODI5NTctMTEuMDk1MDggMTAuODMyMzctLjEyNDYxIDYuMjA1IDUuNzE3NTIgMTEuNTkzMiAxMS45NDUzOCAxMS41OTMyaC43ODM0YzUuNDkwNiAwIDEyLjg0OTctNC4yODI5IDEzLjk5OTUtOS41MDEzLjIxMjMtLjk2MTktLjU1MDItMS44NjYxLTEuNTM4OC0xLjg2NjF6bS0xOC41NDc5LjI3MjFjMCAuODE2Ny0uNjcwMzggMS40ODQ3LTEuNDkwMDEgMS40ODQ3LS44MTk2NCAwLTEuNDg5OTgtLjY2ODMtMS40ODk5OC0xLjQ4NDd2LTIuNDAxOWMwLS44MTY3LjY3MDM0LTEuNDg0NyAxLjQ4OTk4LTEuNDg0Ny44MTk2MyAwIDEuNDkwMDEuNjY4IDEuNDkwMDEgMS40ODQ3em01LjE3MzggMGMwIC44MTY3LS42NzAzIDEuNDg0Ny0xLjQ4OTkgMS40ODQ3LS44MTk3IDAtMS40OS0uNjY4My0xLjQ5LTEuNDg0N3YtMi40MDE5YzAtLjgxNjcuNjcwNi0xLjQ4NDcgMS40OS0xLjQ4NDcuODE5NiAwIDEuNDg5OS42NjggMS40ODk5IDEuNDg0N3oiIGZpbGw9InVybCgjYikiLz48L3N2Zz4K";
    isConnecting = ref(false);
    isConnected = ref(false);

    private _provider: any;

    constructor() {
        this._provider = window.solana;

        // Listen to events.
        if (this._provider) {
            this._provider.on("connect", () => {
                this.isConnected.value = true;
            });

            this._provider.on("disconnect", () => {
                this.isConnected.value = false;
            });
        }
    }

    get isInstalled(): boolean {
        return window.solana?.isPhantom;
    }

    get publicKey(): PublicKey {
        return this._provider.publicKey;
    }

    async connect() {
        if (this.isConnected.value || this.isConnecting.value) {
            return;
        }

        try {
            this.isConnecting.value = true;
            await this._provider.connect();
        } finally {
            this.isConnecting.value = false;
        }
    }

    async connectSilently() {
        if (this.isConnected.value || this.isConnecting.value) {
            return;
        }

        try {
            this.isConnecting.value = true;
            await this._provider.connect({onlyIfTrusted: true});
        } finally {
            this.isConnecting.value = false;
        }
    }

    async disconnect() {
        return this._provider.disconnect();
    }

    async signMessage(message: string): Promise<string> {
        let text = new TextEncoder().encode(message);
        let response = await this._provider.signMessage(text as any);

        return btoa(String.fromCharCode.apply(null, response as any));
    }

    async signTransaction(transaction: Transaction): Promise<Transaction> {
        return this._provider.signTransaction(transaction);
    }
}