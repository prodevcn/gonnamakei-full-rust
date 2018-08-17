import {Component, OnInit, ViewChild} from "@angular/core";
import {NgForm} from "@angular/forms";
import {ActivatedRoute} from "@angular/router";

import {ModalComponent} from "../../components/modal/modal.component";
import {GamePreplayComponent} from "../../components/game-preplay/game-preplay.component";
import {ChallengeService, ParticipantService} from "../../services/http";
import {GMIWallets} from "../../services/wallet";

import {ModalController} from "../../components/modal";
import {ChallengeListRequestBody} from "../../types/api/models/Challenge";

import {ChallengeGamePipe} from "../../pipes/challenge-game/challenge-game.pipe";
import {ParticipantDataService} from "../../services/data";
import {DialogControllerService} from "../../components/dialogs";
import {SolanaService} from "../../services/solana/solana.service";
import {isAPIError} from "../../types/api/APITypes";
import {ChallengeAPIDocument, ChallengeBlockchainData} from "../../types/database/Challenge";

@Component({
    selector: "app-challenge-detail",
    templateUrl: "./challenge-detail.component.html",
    styleUrls: ["./challenge-detail.component.scss"],
})
export class ChallengeDetailComponent implements OnInit {

    GamePreplayComponent = GamePreplayComponent;
    @ViewChild("prePlayModal", {static: true}) prePlayModal: ModalComponent;
    challenge: ChallengeAPIDocument;
    blockchainInfo: ChallengeBlockchainData;
    otherChallenges = [] as Array<ChallengeAPIDocument>;
    userId: string;
    statusMessage: string = "";

    // constructor(private modalController: ModalController, private challengeGamePipe: ChallengeGamePipe,
    //             private gmiWallets: GMIWallets, private challengeService: ChallengeService,
    //             private route: ActivatedRoute, private participantService: ParticipantService,
    //             private participantDataService: ParticipantDataService, private dialogs: DialogControllerService,
    //             private solana: SolanaService) {
    // }

    ngOnInit(): void {
        let data = this.route.snapshot.data.challenge;
        this.challenge = data.challenge;
        this.blockchainInfo = data.blockchainInfo;

        this.route.data.subscribe(({challenge}) => {
            this.challenge = challenge.challenge;
            this.blockchainInfo = challenge.blockchainInfo;
        });

        this.participantDataService.onParticipantLoaded.subscribe(() => this.loadUserId());

        this.loadChallenges();
        this.loadUserId();
    }

    getWalletParticipantGameUserId(): string | null {
        const participant = this.participantDataService.participant;
        return participant?.gamesData?.clashRoyale?.tag;
    }

    loadUserId() {
        this.userId = this.getWalletParticipantGameUserId();
    }

    async loadChallenges() {
        const params = {
            responses: {
                page: 0,
                rowsPerPage: 100,
            },
        } as ChallengeListRequestBody;
        const {results: challenges} = await this.challengeService.listChallenges(params);
        this.otherChallenges = challenges.filter(challenge => challenge.id !== this.challenge.id);
    }

    async onUserIdSubmit(form: NgForm) {
        if (this.statusMessage != "") {
            return;
        }

        if (this.gmiWallets.connectedWallet == null) {
            this.dialogs.showMessage("Oops 😓", "You need to be logged before trying to play");
            return;
        }

        if (form.valid) {
            const wallet = this.gmiWallets.connectedWallet;

            if (this.userId !== this.getWalletParticipantGameUserId()) {
                try {
                    this.statusMessage = "Updating Info";
                    this.participantDataService.participant = await this.participantService.updateParticipant({
                        gamesData: {clashRoyale: {tag: this.userId}},
                    });
                } catch (e) {
                    if (isAPIError(e) && e.errorCode === "undefined_clash_royale_player") {
                        this.dialogs.showMessage("Oops 😓", "The user you have introduced does not exist");
                        return;
                    }

                    this.dialogs.showMessage("Oops 😓",
                        "Cannot update the user info. Please retry or contact an admin through Discord");
                    return;
                } finally {
                    this.statusMessage = "";
                }
            }

            // Check balance and airdrop.
            this.statusMessage = "Getting Balance";
            let balance = await this.solana.selfBalance();
            if (balance < this.solana.solToLamports(1)) {
                try {
                    this.statusMessage = "Airdropping";
                    await this.solana.selfAirdrop(this.solana.solToLamports(2));
                } catch (e) {
                    this.dialogs.showMessage("Oops 😓",
                        "Cannot airdrop sol to your account. Please retry or contact an admin through Discord");
                    return;
                } finally {
                    this.statusMessage = "";
                }
            }

            // Create bet.
            this.statusMessage = "Betting";
            try {
                const betResponse = await this.challengeService.bet(this.challenge.id, {participant: wallet.address});
                this.modalController.open({
                    component: GamePreplayComponent,
                    componentProps: {
                        challenge: this.challenge,
                        blockchainInfo: this.blockchainInfo,
                        challengeBet: betResponse,
                        userId: this.userId,
                    },
                    modalProps: {maxWidth: "750px"},
                });
            } finally {
                this.statusMessage = "";
            }
        }
    }
}
