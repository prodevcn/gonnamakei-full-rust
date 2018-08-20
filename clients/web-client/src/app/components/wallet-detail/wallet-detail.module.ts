import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {WalletDetailComponent} from "./wallet-detail.component";
import {PipesModule} from "../../pipes/pipes.module";

@NgModule({
    declarations: [WalletDetailComponent],
    imports: [CommonModule, PipesModule],
    exports: [WalletDetailComponent],
})
export class WalletDetailModule {
}
