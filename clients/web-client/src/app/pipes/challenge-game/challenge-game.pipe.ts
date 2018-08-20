import {Injectable, Pipe, PipeTransform} from "@angular/core";
import {ChallengeAPIDocument} from "../../types/database/Challenge";

@Injectable() @Pipe({
    name: "challengeGame",
})
export class ChallengeGamePipe implements PipeTransform {

    transform(challenge: ChallengeAPIDocument): string {
        return (challenge.milestones[0].V as any).milestone.game;
    }

}
