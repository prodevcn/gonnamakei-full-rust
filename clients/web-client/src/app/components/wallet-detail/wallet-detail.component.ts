import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {GMIWallets} from "../../services/wallet";

@Component({
    selector: "app-wallet-detail",
    templateUrl: "./wallet-detail.component.html",
    styleUrls: ["./wallet-detail.component.scss"],
})
export class WalletDetailComponent implements OnInit {

    @Output() closeModal = new EventEmitter();
    @Input() wallet: any;
    isPreferredWallet = false;

    constructor(private gmiWallets: GMIWallets) {

    }

    ngOnInit(): void {
        this.loadPreferred();
    }

    loadPreferred() {
        this.isPreferredWallet = this.gmiWallets.loadPreferredWallet()?.key === this.wallet.key;
    }

    async disconnectWallet(wallet: any) {
        if (wallet.isConnected) {
            await this.gmiWallets.disconnectWallet();
            this.close();
        }
    }

    close() {
        this.closeModal.next();
    }

    togglePreferredWallet() {
        if (!this.isPreferredWallet) {
            this.gmiWallets.storePreferredWallet(this.wallet);
        } else {
            this.gmiWallets.storePreferredWallet(null);
        }

        this.loadPreferred();
    }
}
