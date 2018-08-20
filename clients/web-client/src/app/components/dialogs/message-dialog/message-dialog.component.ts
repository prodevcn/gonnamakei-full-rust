import {Component, EventEmitter, Input, OnInit} from "@angular/core";
import {ModalContainer} from "../../modal/modal.interface";

@Component({
    selector: "app-message-dialog",
    templateUrl: "./message-dialog.component.html",
    styleUrls: ["./message-dialog.component.scss"],
})
export class MessageDialogComponent implements OnInit, ModalContainer {

    closeModal = new EventEmitter();
    @Input("showClose") showClose = true;
    @Input() title: string;
    @Input() message: string;

    constructor() {
    }

    ngOnInit(): void {
    }

}
