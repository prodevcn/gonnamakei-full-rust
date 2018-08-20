import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {ChallengeListComponent} from "./challenge-list.component";

import {ChallengeModule} from "../../components/challenge/challenge.module";
import {HomeFooterModule} from "../../components/home-footer/home-footer.module";
import {HeaderModule} from "../../components/header/header.module";

const routes: Routes = [{
    path: "",
    component: ChallengeListComponent,
}];

@NgModule({
    declarations: [ChallengeListComponent],
    imports: [CommonModule, ChallengeModule, HeaderModule, HomeFooterModule, RouterModule.forChild(routes)],
})
export class ChallengeListModule {
}
