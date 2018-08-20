import {Component, Input, OnInit} from "@angular/core";
import {UtilsService} from "../../services/utils";
import {GMIWallets} from "../../services/wallet";

import {WaitlistPromptComponent} from "../waitlist-prompt/waitlist-prompt.component";
import {MobileMenuComponent} from "../mobile-menu/mobile-menu.component";
import {WalletListComponent} from "../wallet-list/wallet-list.component";
import {WalletDetailComponent} from "../wallet-detail/wallet-detail.component";

@Component({
    selector: "app-header",
    templateUrl: "./header.component.html",
    styleUrls: ["./header.component.scss"],
})
export class HeaderComponent implements OnInit {

    WaitlistPromptComponent = WaitlistPromptComponent;
    MobileMenuComponent = MobileMenuComponent;
    WalletListComponent = WalletListComponent;
    WalletDetailComponent = WalletDetailComponent;

    @Input() linkHome = false;

    constructor(public utils: UtilsService, public gmiWallets: GMIWallets) {
    }

    ngOnInit(): void {
    }

}
