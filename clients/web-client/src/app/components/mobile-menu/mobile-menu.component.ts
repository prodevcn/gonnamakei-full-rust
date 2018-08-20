import {Component, EventEmitter, Input, OnInit, ViewChild} from "@angular/core";
import {UtilsService} from "../../services/utils";
import {ModalComponent} from "../modal/modal.component";
import {ModalContainer} from "../modal/modal.interface";
import {WalletListComponent} from "../wallet-list/wallet-list.component";
import {WalletDetailComponent} from "../wallet-detail/wallet-detail.component";
import {GMIWallets} from "../../services/wallet";

@Component({
    selector: "app-mobile-menu",
    templateUrl: "./mobile-menu.component.html",
    styleUrls: ["./mobile-menu.component.scss"],
})
export class MobileMenuComponent implements OnInit, ModalContainer {

    closeModal = new EventEmitter<any>();

    @Input() linkHome = false;

    WalletListComponent = WalletListComponent;
    WalletDetailComponent = WalletDetailComponent;

    @ViewChild("walletListModal", {static: true}) walletListModal!: ModalComponent;
    @ViewChild("walletDetail", {static: true}) walletDetailModal!: ModalComponent;
    display = "none";

    constructor(public utils: UtilsService, public gmiWallets: GMIWallets) {
    }

    ngOnInit(): void {
    }

    scrollToMobileChallenges() {
        this.close();
        if (this.linkHome) {
            this.utils.scrollToHomeChallenges();
        }
    }

    openWalletListModal() {
        this.close();
        this.walletListModal.open();
    }

    openWalletDetailModal() {
        this.close();
        this.walletDetailModal.open();
    }

    close() {
        this.closeModal.next()
    }

}
