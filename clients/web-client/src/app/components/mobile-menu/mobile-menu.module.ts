import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule} from "@angular/router";
import {MobileMenuComponent} from "./mobile-menu.component";
import {ModalModule} from "../modal/modal.module";
import {WaitlistPromptModule} from "../waitlist-prompt/waitlist-prompt.module";

import {PipesModule} from "../../pipes/pipes.module";
import {WalletListModule} from "../wallet-list/wallet-list.module";
import {WalletDetailModule} from "../wallet-detail/wallet-detail.module";

@NgModule({
    declarations: [MobileMenuComponent],
    imports: [CommonModule, RouterModule, ModalModule, PipesModule, WaitlistPromptModule, WalletListModule,
        WalletDetailModule],
    exports: [MobileMenuComponent],
})
export class MobileMenuModule {
}
