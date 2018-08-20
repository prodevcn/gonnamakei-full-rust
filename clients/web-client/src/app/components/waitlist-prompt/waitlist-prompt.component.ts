import {Component, EventEmitter, OnInit, Output} from "@angular/core";
import {NgForm} from "@angular/forms";
import {Axios} from "axios";

import {ModalContainer} from "../modal/modal.interface";

@Component({
    selector: "app-waitlist-prompt",
    templateUrl: "./waitlist-prompt.component.html",
    styleUrls: ["./waitlist-prompt.component.scss"],
})
export class WaitlistPromptComponent implements OnInit, ModalContainer {

    email = "";
    display = "none";

    @Output() closeModal = new EventEmitter();

    ngOnInit(): void {
    }

    async onSubmit(form: NgForm) {
        if (form.valid) {

            let axios = new Axios({});
            await axios.post("https://api.gonnamakeit.app/email/subscribe", JSON.stringify({email: this.email}), {
                headers: {"Content-type": "application/json"},
            });

            form.resetForm();

            this.close();
        }
    }

    close() {
        this.closeModal.next();
    }

}
