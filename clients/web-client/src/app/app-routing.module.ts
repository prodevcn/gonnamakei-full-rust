import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";

import {ChallengeResolver} from "./resolvers/challenge.resolver";
import {ChallengeBetResolver} from "./resolvers/challenge-bet.resolver";
import {BetSendResponseResolver} from "./resolvers/bet-send-response.resolver";
import {ChallengeDetailGuard} from "./guards/challenge-detail.guard";
import {GamePlayGuard} from "./guards/game-play.guard";

const routes: Routes = [{
    path: "",
    loadChildren: () => import("./pages/home/home.module").then(m => m.HomeModule),
}, {
    path: "challenges",
    loadChildren: () => import("./pages/challenge-list/challenge-list.module").then(m => m.ChallengeListModule),
}, {
    path: "challenge/:id",
    loadChildren: () => import("./pages/challenge-detail/challenge-detail.module").then(m => m.ChallengeDetailModule),
    resolve: {challenge: ChallengeResolver},
    canActivate: [ChallengeDetailGuard],
}, {
    path: "challenge/:id/:betId/play",
    loadChildren: () => import("./pages/game-play/game-play.module").then(m => m.GamePlayModule),
    resolve: {
        challenge: ChallengeResolver,
        challengeBet: ChallengeBetResolver,
        betSendResponse: BetSendResponseResolver,
    },
    canActivate: [ChallengeDetailGuard, GamePlayGuard],
}, {
    path: "challenge/:id/:betId/won",
    loadChildren: () => import("./pages/game-won/game-won.module").then(m => m.GameWonModule),
    resolve: {
        challenge: ChallengeResolver,
        challengeBet: ChallengeBetResolver,
        betSendResponse: BetSendResponseResolver,
    },
    canActivate: [ChallengeDetailGuard],
}, {
    path: "challenge/:id/:betId/:action",
    loadChildren: () => import("./pages/game-play/game-play.module").then(m => m.GamePlayModule),
    resolve: {
        challenge: ChallengeResolver,
        challengeBet: ChallengeBetResolver,
        betSendResponse: BetSendResponseResolver,
    },
    canActivate: [ChallengeDetailGuard],
}, {
    path: "faqs",
    loadChildren: () => import("./pages/faqs/faqs.module").then(m => m.FaqsModule),
}, {
    path: "about-us",
    loadChildren: () => import("./pages/about-us/about-us.module").then(m => m.AboutUsModule),
}, {
    path: "the-proyect",
    loadChildren: () => import("./pages/the-proyect/the-proyect.module").then(m => m.TheProyectModule),
}, {
    path: "blog",
    loadChildren: () => import("./pages/stories/stories.module").then(m => m.StoriesModule),
}];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule],
})
export class AppRoutingModule {
}
