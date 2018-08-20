import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule} from "@angular/router";

import {GamePreplayComponent} from "./game-preplay.component";

import {PipesModule} from "../../pipes/pipes.module";

@NgModule({
    declarations: [GamePreplayComponent],
    imports: [CommonModule, RouterModule, PipesModule],
    exports: [GamePreplayComponent],
})
export class GamePreplayModule {
}
