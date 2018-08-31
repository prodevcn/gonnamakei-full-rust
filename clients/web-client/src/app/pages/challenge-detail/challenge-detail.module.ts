import {NgModule} from "@angular/core";
import {FormsModule} from "@angular/forms";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {ChallengeDetailComponent} from "./challenge-detail.component";

import {ChallengeModule} from "../../components/challenge/challenge.module";
import {FooterModule} from "../../components/footer/footer.module";
import {HeaderModule} from "../../components/header/header.module";

import {PipesModule} from "../../pipes/pipes.module";
import {ModalModule} from "../../components/modal/modal.module";
import {GamePreplayModule} from "../../components/game-preplay/game-preplay.module";

const routes: Routes = [{
    path: "",
    component: ChallengeDetailComponent,
}];

@NgModule({
    declarations: [ChallengeDetailComponent],
    imports: [CommonModule, ChallengeModule, HeaderModule, FooterModule, FormsModule, ModalModule, GamePreplayModule,
        PipesModule, RouterModule.forChild(routes)],
})
export class ChallengeDetailModule {
}
