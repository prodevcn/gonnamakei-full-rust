import {ClashRoyaleMilestone} from "./game/ClashRoyale";

export type GameMilestone = GameMilestone_ClashRoyale;

export interface GameMilestone_ClashRoyale {
    game: "ClashRoyale",
    challenge: ClashRoyaleMilestone
}