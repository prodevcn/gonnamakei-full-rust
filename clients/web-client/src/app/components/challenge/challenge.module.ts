import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule} from "@angular/router";

import {ModalModule} from "../modal/modal.module";
import {WaitlistPromptModule} from "../waitlist-prompt/waitlist-prompt.module";
import {ChallengeComponent} from "./challenge.component";
import {PipesModule} from "../../pipes/pipes.module";

@NgModule({
    declarations: [ChallengeComponent],
    imports: [CommonModule, RouterModule, ModalModule, WaitlistPromptModule, PipesModule],
    exports: [ChallengeComponent],
})
export class ChallengeModule {
}
