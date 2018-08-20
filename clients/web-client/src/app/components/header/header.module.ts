import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule} from "@angular/router";
import {HeaderComponent} from "./header.component";
import {ModalModule} from "../modal/modal.module";
import {WaitlistPromptModule} from "../waitlist-prompt/waitlist-prompt.module";
import {MobileMenuModule} from "../mobile-menu/mobile-menu.module";
import {WalletListModule} from "../wallet-list/wallet-list.module";
import {WalletDetailModule} from "../wallet-detail/wallet-detail.module";
import {PipesModule} from "../../pipes/pipes.module";

@NgModule({
    declarations: [HeaderComponent],
    imports: [CommonModule, RouterModule, ModalModule, WaitlistPromptModule, MobileMenuModule, WalletListModule,
        WalletDetailModule, PipesModule],
    exports: [HeaderComponent],
})
export class HeaderModule {
}
