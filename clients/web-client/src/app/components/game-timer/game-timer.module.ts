import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {GameTimerComponent} from "./game-timer.component";
import {PipesModule} from "../../pipes/pipes.module";

@NgModule({
    declarations: [GameTimerComponent],
    imports: [CommonModule, PipesModule],
    exports: [GameTimerComponent],
})
export class GameTimerModule {
}
