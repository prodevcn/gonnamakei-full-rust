import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {WalletListComponent} from "./wallet-list.component";
import {WalletDetailModule} from "../wallet-detail/wallet-detail.module";
import {PipesModule} from "../../pipes/pipes.module";

@NgModule({
    declarations: [WalletListComponent],
    imports: [CommonModule, WalletDetailModule, PipesModule],
    exports: [WalletListComponent],
})
export class WalletListModule {
}
