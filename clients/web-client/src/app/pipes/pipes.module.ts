import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {SafeUrlPipe} from "./safe-url/safe-url.pipe";
import {TimeLeftPipe} from "./time-left/time-left.pipe";
import {TimeLeftFormatPipe} from "./time-left-format/time-left-format.pipe";
import {GameComingSoonPipe} from "./game-coming-soon/game-coming-soon.pipe";
import {ChallengeGamePipe} from "./challenge-game/challenge-game.pipe";
import {GameImageUrlPipe} from "./game-image-url/game-image-url.pipe";
import {SolAmountPipe} from "./sol-amount/sol-amount.pipe";

@NgModule({
    declarations: [SafeUrlPipe, TimeLeftPipe, TimeLeftFormatPipe, GameComingSoonPipe, ChallengeGamePipe,
        GameImageUrlPipe, SolAmountPipe],
    imports: [CommonModule],
    providers: [TimeLeftPipe, ChallengeGamePipe, GameComingSoonPipe],
    exports: [SafeUrlPipe, TimeLeftPipe, TimeLeftFormatPipe, GameComingSoonPipe, ChallengeGamePipe, GameImageUrlPipe,
        SolAmountPipe],
})
export class PipesModule {
}
