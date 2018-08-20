import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {GameWonComponent} from "./game-won.component";

import {AngularSvgIconModule} from "angular-svg-icon";

const routes: Routes = [{
    path: "",
    component: GameWonComponent,
}];

@NgModule({
    declarations: [GameWonComponent],
    imports: [CommonModule, AngularSvgIconModule, RouterModule.forChild(routes)],
})
export class GameWonModule {
}
