import {Component, EventEmitter, OnInit} from "@angular/core";
import {ActivatedRoute} from "@angular/router";

import {ModalContainer} from "../../components/modal/modal.interface";

import {environment} from "../../../environments/environment";
import {ChallengeAPIDocument} from "../../types/database/Challenge";

@Component({
    selector: "app-game-won",
    templateUrl: "./game-won.component.html",
    styleUrls: ["./game-won.component.scss"],
})
export class GameWonComponent implements OnInit, ModalContainer {

    svgStyle = {
        fill: "black",
        cursor: "pointer",
        width: "100%",
        height: "auto",
    };
    svgLinkStyle = {
        fill: "#93AB01",
    };
    socialLinks = environment.socialLinks;
    closeModal = new EventEmitter();
    challenge: ChallengeAPIDocument;

    constructor(private route: ActivatedRoute) {
    }

    ngOnInit(): void {
        this.challenge = this.route.snapshot.data.challenge.challenge;
        this.route.data.subscribe(({challenge}) => {
            this.challenge = challenge.challenge;
        });
    }

    onClose() {

    }

}
