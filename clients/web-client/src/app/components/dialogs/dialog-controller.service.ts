import {Injectable} from "@angular/core";
import {ModalController} from "../modal";

import {BetCheckingComponent} from "../bet-checking/bet-checking.component";
import {MessageDialogComponent} from "./message-dialog/message-dialog.component";
import {first} from "rxjs/operators";

@Injectable({
    providedIn: "root",
})
export class DialogControllerService {

    constructor(private modalController: ModalController) {
    }

    showMessage(title: string, message: string): Promise<any> {

        return new Promise((res, rej) => {

            const {closeModal} = this.modalController.open({
                component: MessageDialogComponent,
                componentProps: {
                    title,
                    message,
                },
                modalProps: {maxWidth: "500px"},
            });
            closeModal.pipe(first()).subscribe((...args) => res(...args));

        });
    }

    showBetCheck() {
        return this.modalController.open({component: BetCheckingComponent});
    }

}
