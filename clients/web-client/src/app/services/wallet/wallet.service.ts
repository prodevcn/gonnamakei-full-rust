import {EventEmitter, Injectable} from "@angular/core";
import {ParticipantService, SignatureService} from "../http";

import {environment} from "../../../environments/environment";
import {Connection, Transaction, TransactionSignature} from "@solana/web3.js";
import {isAPIError} from "../../types/api/APITypes";
import {SignatureAction} from "../../types/api/models/Signature";
import {ParticipantDataService} from "../data";
import {ParticipantLoginRequestBody} from "../../types/api/models/Participant";

const {
    PREFERRED_WALLET_STORAGE_KEY,
} = environment;

declare const window;

@Injectable({
    providedIn: "root",
})
export class GMIWallets {
    // WALLETS ----------------------------------------------------------------
    wallets: { [key: string]: WalletAndProvider } = {
        phantom: new WalletAndProvider("phantom", getPhantomWallet()),
    };

    onConnect = new EventEmitter<WalletAndProvider>();
    onDisconnect = new EventEmitter<WalletAndProvider>();
    selectedWalletKey: string | null = null;
    connectedWallet: WalletAndProvider | null = null;

    constructor(private signatureClient: SignatureService, private participantService: ParticipantService,
                private participantDataService: ParticipantDataService) {
        for (let wallet of Object.values(this.wallets)) {
            wallet.signatureClient = signatureClient;

            wallet.onConnect.subscribe(() => {
                this.connectedWallet = wallet;
                this.onConnect.next(wallet);
            });

            wallet.onDisconnect.subscribe(() => {
                if (this.connectedWallet === wallet) {
                    this.connectedWallet = null;
                    this.selectedWalletKey = null;
                }

                this.onDisconnect.next(wallet);
            });
        }

        this.initPreferredWallet();
    }

    async loginAndLoadParticipantInfo(wallet: WalletAndProvider) {
        let initiallyLogged = this.participantDataService.isLoggedIn;

        // Try login if n
        if (!initiallyLogged) {
            try {
                const loginRequest = await wallet.solveSignatureChallenge({T: "login"});
                await this.participantService.loginParticipant(loginRequest);
            } catch (error) {
                // Rollback.
                await this.disconnectWallet();

                throw error;
            }
        }

        // Get user data.
        try {
            const participantGetResponse = await this.participantService.getParticipant({
                returnFields: true,
                returnActiveBets: false, // TODO activate in the future
            });

            this.participantDataService.participant = participantGetResponse?.participant;
        } catch (error) {
            // Rollback.
            await this.disconnectWallet();

            throw error;
        }
    }

    async initPreferredWallet() {
        try {
            const preferred = this.loadPreferredWallet();

            if (preferred && preferred.isInstalled) {
                await this.selectAndConnectWallet(preferred, true);
            }
        } catch (e) {
            await this.disconnectWallet();
        }
    }

    async selectAndConnectWallet(wallet: WalletAndProvider, silent: boolean = false) {
        if (this.connectedWallet === wallet) {
            return;
        }

        if (!wallet.isInstalled) {
            throw new Error("Cannot connect to a not installed wallet");
        }

        this.selectedWalletKey = wallet.key;

        if (!wallet.isConnected && !wallet.isConnecting) {
            try {
                await wallet.connect(silent);
            } catch (e) {
                if (silent) {
                    return;
                }

                throw e;
            }
        }

        this.storePreferredWallet(wallet);

        try {
            await this.loginAndLoadParticipantInfo(this.connectedWallet);
        } catch (e) {
            if (silent) {
                return;
            }

            throw e;
        }
    }

    async disconnectWallet() {
        this.storePreferredWallet(null);

        let wallet = this.connectedWallet;
        if (wallet.isConnected || wallet.isConnecting) {
            await wallet.disconnect();
        }

        this.selectedWalletKey = null;
        this.participantDataService.apiToken = null;
    }

    // METHODS ----------------------------------------------------------------
    walletList(): WalletAndProvider[] {
        return Object.values(this.wallets);
    }

    findWalletByKey(key: string): WalletAndProvider | null {
        return this.wallets[key] || null;
    }

    loadPreferredWallet(): WalletAndProvider | null {
        let key = localStorage.getItem(PREFERRED_WALLET_STORAGE_KEY);

        if (key) {
            return this.findWalletByKey(key);
        }

        return null;
    }

    storePreferredWallet(wallet: WalletAndProvider | null) {
        if (wallet) {
            localStorage.setItem(PREFERRED_WALLET_STORAGE_KEY, wallet.key);
        } else {
            localStorage.removeItem(PREFERRED_WALLET_STORAGE_KEY);
        }
    }
}

export class WalletAndProvider {
    key: string;
    name: string;
    url: string;
    icon: string;
    isConnecting = false;
    isConnected = false;
    signatureClient: SignatureService;
    onConnect = new EventEmitter();
    onDisconnect = new EventEmitter();

    wallet: any;

    private _provider: any;

    constructor(key: string, wallet: any) {
        this.wallet = wallet;

        this.key = key;
        this.name = wallet.name;
        this.url = wallet.url;
        this.icon = wallet.icon;
        this._provider = wallet.provider();

        // Listen to events.
        if (this._provider) {
            this.initProvider();
        } else {
            this.tryToInitialize();
        }
    }

    get isInstalled(): boolean {
        return this._provider?.ready || (this.wallet.isInstalled && this.wallet.isInstalled());
    }

    get publicKey() {
        return this._provider?.publicKey;
    }

    get address() {
        return this.publicKey?.toBase58();
    }

    tryToInitialize() {
        let timerId;
        const maxTries = 10;
        let tries = 0;
        timerId = setInterval(() => {
            this._provider = this.wallet.provider();
            if (this._provider) {
                this.initProvider();
            }

            if (this._provider || ++tries >= maxTries) {
                clearInterval(timerId);
            }
        }, 300);
    }

    initProvider() {
        this._provider.on("connect", () => {
            this.isConnected = true;
            this.onConnect.next();
        });

        this._provider.on("disconnect", () => {
            this.isConnected = false;
            this.onDisconnect.next();
        });
    }

    async connect(silent: boolean = false) {
        if (this.isConnected || this.isConnecting) {
            return;
        }

        try {
            this.isConnecting = true;

            if (silent) {
                await this._provider.connect({onlyIfTrusted: true});
            } else {
                await this._provider.connect();
            }

            this.isConnected = true;
        } finally {
            this.isConnecting = false;
        }
    }

    async disconnect() {
        return this._provider.disconnect();
    }

    async signTransaction(transaction: string): Promise<{ signature: string }> {
        let provider = this._provider as any;
        if (provider.request) {

            return await provider.request({
                method: "signTransaction",
                params: {
                    message: transaction,
                },
            });
        }

        throw new Error("Unsupported signTransaction for the wallet: " + this.name);
    }

    async signMessage(message: string): Promise<string> {
        let provider = this._provider as any;
        if (provider.signMessage) {
            let text = new TextEncoder().encode(message);
            let response = await provider.signMessage(text as any);

            return btoa(String.fromCharCode.apply(null, response.signature as any));
        }

        throw new Error("Unsupported signMessage for the wallet: " + this.name);
    }

    async solveSignatureChallenge(action: SignatureAction): Promise<ParticipantLoginRequestBody> {
        if ((this._provider as any).signMessage) {
            let provider = this._provider as any;
            let challenge = await this.signatureClient.request({
                address: provider.publicKey!.toBase58(),
                action,
            });

            if (isAPIError(challenge)) {
                throw new Error("Cannot get a signature challenge");
            }

            let signature = await this.signMessage(challenge.message);

            return {
                id: challenge.id,
                signature,
            };
        }

        throw new Error("Unsupported signMessage for the wallet: " + this.name);
    }

    async sendTransaction(transaction: Transaction, connection: Connection,
                          options?: any): Promise<TransactionSignature> {
        return this._provider.sendTransaction(transaction, connection, options);
    }
}

export const getPhantomWallet = () => ({
    name: "Phantom",
    url: "https://www.phantom.app",
    icon: "data:image/svg+xml;base64,PHN2ZyBmaWxsPSJub25lIiBoZWlnaHQ9IjM0IiB3aWR0aD0iMzQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGxpbmVhckdyYWRpZW50IGlkPSJhIiB4MT0iLjUiIHgyPSIuNSIgeTE9IjAiIHkyPSIxIj48c3RvcCBvZmZzZXQ9IjAiIHN0b3AtY29sb3I9IiM1MzRiYjEiLz48c3RvcCBvZmZzZXQ9IjEiIHN0b3AtY29sb3I9IiM1NTFiZjkiLz48L2xpbmVhckdyYWRpZW50PjxsaW5lYXJHcmFkaWVudCBpZD0iYiIgeDE9Ii41IiB4Mj0iLjUiIHkxPSIwIiB5Mj0iMSI+PHN0b3Agb2Zmc2V0PSIwIiBzdG9wLWNvbG9yPSIjZmZmIi8+PHN0b3Agb2Zmc2V0PSIxIiBzdG9wLWNvbG9yPSIjZmZmIiBzdG9wLW9wYWNpdHk9Ii44MiIvPjwvbGluZWFyR3JhZGllbnQ+PGNpcmNsZSBjeD0iMTciIGN5PSIxNyIgZmlsbD0idXJsKCNhKSIgcj0iMTciLz48cGF0aCBkPSJtMjkuMTcwMiAxNy4yMDcxaC0yLjk5NjljMC02LjEwNzQtNC45NjgzLTExLjA1ODE3LTExLjA5NzUtMTEuMDU4MTctNi4wNTMyNSAwLTEwLjk3NDYzIDQuODI5NTctMTEuMDk1MDggMTAuODMyMzctLjEyNDYxIDYuMjA1IDUuNzE3NTIgMTEuNTkzMiAxMS45NDUzOCAxMS41OTMyaC43ODM0YzUuNDkwNiAwIDEyLjg0OTctNC4yODI5IDEzLjk5OTUtOS41MDEzLjIxMjMtLjk2MTktLjU1MDItMS44NjYxLTEuNTM4OC0xLjg2NjF6bS0xOC41NDc5LjI3MjFjMCAuODE2Ny0uNjcwMzggMS40ODQ3LTEuNDkwMDEgMS40ODQ3LS44MTk2NCAwLTEuNDg5OTgtLjY2ODMtMS40ODk5OC0xLjQ4NDd2LTIuNDAxOWMwLS44MTY3LjY3MDM0LTEuNDg0NyAxLjQ4OTk4LTEuNDg0Ny44MTk2MyAwIDEuNDkwMDEuNjY4IDEuNDkwMDEgMS40ODQ3em01LjE3MzggMGMwIC44MTY3LS42NzAzIDEuNDg0Ny0xLjQ4OTkgMS40ODQ3LS44MTk3IDAtMS40OS0uNjY4My0xLjQ5LTEuNDg0N3YtMi40MDE5YzAtLjgxNjcuNjcwNi0xLjQ4NDcgMS40OS0xLjQ4NDcuODE5NiAwIDEuNDg5OS42NjggMS40ODk5IDEuNDg0N3oiIGZpbGw9InVybCgjYikiLz48L3N2Zz4K",
    provider: () => window.solana,
    isInstalled: () => window.solana?.isPhantom,
});