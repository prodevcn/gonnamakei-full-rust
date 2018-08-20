import {Injectable, Pipe, PipeTransform} from "@angular/core";
import {GameList} from "../../constants";

@Injectable() @Pipe({
    name: "gameComingSoon",
})
export class GameComingSoonPipe implements PipeTransform {

    transform(gameId): boolean {
        return !GameList.find(game => game.id === gameId);
    }

}
