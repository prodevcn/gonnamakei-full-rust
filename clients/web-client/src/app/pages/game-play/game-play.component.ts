import {Component, OnInit} from "@angular/core";
import {ActivatedRoute, Router} from "@angular/router";

import {BetSendResponse, ChallengeCheckResponseStatus} from "../../types/api/models/Bet";
import {ChallengeBetCreateResponse} from "../../types/api/models/Challenge";
import {ChallengeService} from "../../services/http";
import {GMIWallets} from "../../services/wallet";
import {ModalController} from "../../components/modal";
import {DialogControllerService} from "../../components/dialogs";
import {ChallengeGamePipe} from "../../pipes/challenge-game/challenge-game.pipe";
import {ParticipantDataService} from "../../services/data";
import {BetService} from "../../services/http/bet/bet.service";
import {ChallengeAPIDocument, ChallengeBlockchainData} from "../../types/database/Challenge";

@Component({
    selector: "app-game-play",
    templateUrl: "./game-play.component.html",
    styleUrls: ["./game-play.component.scss"],
})
export class GamePlayComponent implements OnInit {

    gameState: "lost" | "checking" | "playing";
    challenge: ChallengeAPIDocument;
    blockchainInfo: ChallengeBlockchainData;
    challengeBet: ChallengeBetCreateResponse;
    betSendResponse: BetSendResponse;

    constructor(private dialogs: DialogControllerService, private route: ActivatedRoute,
                private challengeGamePipe: ChallengeGamePipe, private modalController: ModalController,
                private gmiWallets: GMIWallets, private challengeService: ChallengeService,
                private betService: BetService, private router: Router,
                private participantDataService: ParticipantDataService) {
    }

    ngOnInit(): void {
        let data = this.route.snapshot.data.challenge;
        this.challenge = data.challenge;
        this.blockchainInfo = data.blockchainInfo;

        const action = this.route.snapshot.params.action;
        this.route.data.subscribe(({
                                       challenge,
                                       challengeBet,
                                       betSendResponse,
                                   }) => {
            this.challenge = challenge.challenge;
            this.blockchainInfo = challenge.blockchainInfo;
            this.challengeBet = challengeBet;
            this.betSendResponse = betSendResponse;
            this.init(this.route.snapshot.params.action);
        });
        this.route.params.subscribe(({action}) => this.init(action));

        this.init(action);
    }

    init(action) {
        this.gameState = action || "playing";
    }

    getWalletParticipantGameUserId(): string | null {
        const participant = this.participantDataService.participant;
        return participant?.gamesData?.clashRoyale?.tag;
    }

    async playAgain() {
        this.router.navigate(["/challenge", this.challenge.id]);
    }

    async timeout() {
        await this.checkResult(true);

        const betId = this.challengeBet.bet;
        if (this.gameState == "playing") {
            this.router.navigate(["/challenge", this.challenge.id, betId, "lost"]);
        }
    }

    async checkResult(silent: boolean = false) {
        if (this.gameState !== "playing") {
            return;
        }

        this.gameState = "checking";
        const betId = this.challengeBet.bet;

        try {
            const {status} = await this.betService.checkSentBet();

            if (status === ChallengeCheckResponseStatus.Won) {
                this.router.navigate(["/challenge", this.challenge.id, betId, "won"]);
            } else if (status === ChallengeCheckResponseStatus.Lost) {
                this.router.navigate(["/challenge", this.challenge.id, betId, "lost"]);
            } else if (status === ChallengeCheckResponseStatus.Expired) {
                this.dialogs.showMessage("Oops 😓", "The challenge expired");
                this.router.navigate(["/challenge", this.challenge.id, betId, "lost"]);
            } else if (status === ChallengeCheckResponseStatus.NotInitiated) {
                if (!silent) {
                    this.dialogs.showMessage("Challenge in progress", "You haven't done anything yet");
                }
                this.gameState = "playing";
            } else if (status === ChallengeCheckResponseStatus.Initiated) {
                if (!silent) {
                    this.dialogs.showMessage("Challenge in progress",
                        "The challenge requires you to play more matches to complete");
                }
                this.gameState = "playing";
            }
        } catch (e) {
            if (!silent) {
                this.dialogs.showMessage("Oops 😓", "There was a problem checking the result. Please try again later");
            }
            this.gameState = "playing";
        }
    }
}
