import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FormsModule} from "@angular/forms";
import {WaitlistPromptComponent} from "./waitlist-prompt.component";

@NgModule({
    declarations: [WaitlistPromptComponent],
    imports: [CommonModule, FormsModule],
    exports: [WaitlistPromptComponent],
})
export class WaitlistPromptModule {
}
