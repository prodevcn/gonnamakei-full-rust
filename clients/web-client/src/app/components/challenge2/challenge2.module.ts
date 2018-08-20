import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule} from "@angular/router";

import {ModalModule} from "../modal/modal.module";
import {WaitlistPromptModule} from "../waitlist-prompt/waitlist-prompt.module";
import {Challenge2Component} from "./challenge2.component";

import {PipesModule} from "../../pipes/pipes.module";

@NgModule({
    declarations: [Challenge2Component],
    imports: [CommonModule, RouterModule, ModalModule, WaitlistPromptModule, PipesModule],
    exports: [Challenge2Component],
})
export class Challenge2Module {
}
