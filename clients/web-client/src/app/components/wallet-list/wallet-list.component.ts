import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {GMIWallets, WalletAndProvider} from "../../services/wallet";
import {WalletDetailComponent} from "../wallet-detail/wallet-detail.component";
import {ModalController} from "../modal";
import {DialogControllerService} from "../dialogs";

@Component({
    selector: "app-wallet-list",
    templateUrl: "./wallet-list.component.html",
    styleUrls: ["./wallet-list.component.scss"],
})
export class WalletListComponent implements OnInit {

    @Input() showDetailOnConnect = false;
    @Output() closeModal = new EventEmitter();
    installedWallets = [] as Array<WalletAndProvider>;
    availableWallets = [] as Array<WalletAndProvider>;
    isConnecting = false;

    constructor(public gmiWallets: GMIWallets, private modalController: ModalController,
                private dialogs: DialogControllerService) {
    }

    ngOnInit(): void {
        this.loadWallets();

        this.gmiWallets.onConnect.subscribe(() => this.loadWallets());
        this.gmiWallets.onDisconnect.subscribe(() => this.loadWallets());
    }

    async loadWallets() {
        const wallets = this.gmiWallets.walletList();
        this.installedWallets = wallets.filter(wallet => wallet.isInstalled);
        this.availableWallets = wallets.filter(wallet => !wallet.isInstalled);
    }

    async connectWallet(wallet: WalletAndProvider) {
        if (this.isConnecting || this.gmiWallets.connectedWallet === wallet) {
            return;
        }

        this.isConnecting = true;

        try {
            await this.gmiWallets.selectAndConnectWallet(wallet);

            this.modalController.open({
                component: WalletDetailComponent,
                componentProps: {wallet},
            });
            this.close();
        } catch (e) {
            if (e.code === 4001) {
                this.dialogs.showMessage("Rejected signature", "Cannot log in if you don't sign message");
            }
        } finally {
            this.isConnecting = false;
        }
    }

    close() {
        this.closeModal.next();
    }
}
