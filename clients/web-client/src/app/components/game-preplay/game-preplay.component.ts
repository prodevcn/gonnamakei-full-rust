import {Component, EventEmitter, Input, OnInit} from "@angular/core";
import {Router} from "@angular/router";
import {ModalContainer} from "../modal/modal.interface";

import {GMIWallets} from "../../services/wallet";
import {ChallengeBetCreateResponse} from "../../types/api/models/Challenge";
import {DialogControllerService} from "../dialogs";
import {BetService} from "../../services/http/bet/bet.service";
import {ChallengeAPIDocument, ChallengeBlockchainData} from "../../types/database/Challenge";
import {SolanaService} from "../../services/solana/solana.service";
import {Message} from "@solana/web3.js";
import * as b58 from "b58";

@Component({
    selector: "app-game-preplay",
    templateUrl: "./game-preplay.component.html",
    styleUrls: ["./game-preplay.component.scss"],
})
export class GamePreplayComponent implements OnInit, ModalContainer {

    @Input() userId: any;
    @Input() challenge: ChallengeAPIDocument;
    @Input() blockchainInfo: ChallengeBlockchainData;
    @Input() challengeBet: ChallengeBetCreateResponse;
    closeModal = new EventEmitter();

    constructor(private gmiWallets: GMIWallets, private dialogController: DialogControllerService,
                private betService: BetService, private router: Router, private solana: SolanaService,
                private dialogs: DialogControllerService) {
    }

    async ngOnInit() {
    }

    async letsGo() {
        const wallet = this.gmiWallets.connectedWallet;

        const message = Message.from(b58.decode(this.challengeBet.message));
        const recentBlockHash = await this.solana.recentBlockhash();
        message.recentBlockhash = recentBlockHash;

        let signature: string;
        try {
            signature = (await wallet.signTransaction(b58.encode(message.serialize()))).signature;
        } catch (e) {
            this.dialogs.showMessage("Rejected signature", "You must accept the signature to start playing");
            return;
        }

        const bettingDialog = this.dialogController.showBetCheck();

        // The result is cached in the service to be used in the next view.
        await this.betService.sendBet(this.challengeBet.bet, {
            recentBlockHash,
            signature: signature,
        }).catch((error) => {
            bettingDialog.close();
            throw error;
        });

        this.closeModal.next();
        bettingDialog.close();

        this.router.navigate(["/challenge", this.challenge.id, this.challengeBet.bet, "play"]);
    }
}
