import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {AngularSvgIconModule} from "angular-svg-icon";
import {HomeComponent} from "./home.component";
import {HeaderModule} from "../../components/header/header.module";
import {HomeFooterModule} from "../../components/home-footer/home-footer.module";
import {ModalModule} from "../../components/modal/modal.module";
import {WaitlistPromptModule} from "../../components/waitlist-prompt/waitlist-prompt.module";
import {ChallengeModule} from "../../components/challenge/challenge.module";
import {Challenge2Module} from "../../components/challenge2/challenge2.module";
import {DevnetMessageComponent} from "../../components/devnet-message/devnet-message.component";

const routes: Routes = [{
    path: "",
    component: HomeComponent,
}];

@NgModule({
    declarations: [HomeComponent, DevnetMessageComponent],
    imports: [CommonModule, AngularSvgIconModule, HeaderModule, HomeFooterModule, ModalModule, WaitlistPromptModule,
        ChallengeModule, Challenge2Module, RouterModule.forChild(routes)],
    exports: [HomeComponent],
})
export class HomeModule {
}
