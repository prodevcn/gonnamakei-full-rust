import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {GamePlayComponent} from "./game-play.component";

import {GameTimerModule} from "../../components/game-timer/game-timer.module";
import {FooterModule} from "../../components/footer/footer.module";
import {HeaderModule} from "../../components/header/header.module";

import {PipesModule} from "../../pipes/pipes.module";

const routes: Routes = [{
    path: "",
    component: GamePlayComponent,
}];

@NgModule({
    declarations: [GamePlayComponent],
    imports: [CommonModule, GameTimerModule, FooterModule, HeaderModule, PipesModule, RouterModule.forChild(routes)],
})
export class GamePlayModule {
}
