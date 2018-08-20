import {Pipe, PipeTransform} from "@angular/core";
import {GameList} from "../../constants";

@Pipe({
    name: "gameImageUrl",
})
export class GameImageUrlPipe implements PipeTransform {

    transform(gameId: any): string {
        const game = GameList.find(g => g.id === gameId);
        return game?.imageUrl;
    }

}
