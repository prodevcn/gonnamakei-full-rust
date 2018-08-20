import {OrderedCondition} from "../../Conditions";

export type ClashRoyaleMilestone = ClashRoyaleMilestone_WinMatches | ClashRoyaleMilestone_Achievement;

export interface ClashRoyaleMilestone_WinMatches {
    type: "winMatches",
    params: ClashRoyaleMatchConditions[]
}

export interface ClashRoyaleMilestone_Achievement {
    type: "achievement",
    params: ClashRoyaleUserConditions
}

export interface ClashRoyaleMatchConditions {
    win?: boolean,
    allowFriends?: boolean,
    arena?: OrderedCondition<ClashRoyaleArena>,
    gameMode?: OrderedCondition<ClashRoyaleGameMode>,
    team?: ClashRoyaleTeamConditions,
    opponent?: ClashRoyaleTeamConditions,
}

export interface ClashRoyaleTeamConditions {
    crowns?: OrderedCondition<number>,
    startingTrophies?: OrderedCondition<number>,
    allowedCards?: ClashRoyaleCardConditions,
    forbiddenCards?: ClashRoyaleCardConditions,
    firstMember?: ClashRoyaleTeamMemberConditions,
    secondMember?: ClashRoyaleTeamMemberConditions,
}

export interface ClashRoyaleTeamMemberConditions {
    startingTrophies?: OrderedCondition<number>,
    allowedCards?: ClashRoyaleCardConditions,
    forbiddenCards?: ClashRoyaleCardConditions,
}

export interface ClashRoyaleUserConditions {
    expLevel?: OrderedCondition<number>,
    trophies?: OrderedCondition<number>,
    bestTrophies?: OrderedCondition<number>,
    wins?: OrderedCondition<number>,
    threeCrownsWins?: OrderedCondition<number>,
    losses?: OrderedCondition<number>,
    battleCount?: OrderedCondition<number>,
    challengeCardsWon?: OrderedCondition<number>,
    challengeMaxWins?: OrderedCondition<number>,
    tournamentCardsWon?: OrderedCondition<number>,
    tournamentBattleCount?: OrderedCondition<number>,
    donations?: OrderedCondition<number>,
    donationsReceived?: OrderedCondition<number>,
    totalDonations?: OrderedCondition<number>,
    warDayWins?: OrderedCondition<number>,
    clanCardsCollected?: OrderedCondition<number>,
    currentArena?: OrderedCondition<ClashRoyaleArena>,
    allowedCards?: ClashRoyaleCardConditions,
    forbiddenCards?: ClashRoyaleCardConditions,
}

export interface ClashRoyaleCardConditions {
    card?: ClashRoyaleCard,
    level?: OrderedCondition<number>,
    count?: OrderedCondition<number>,
}

export enum ClashRoyaleGameMode {
    OneVsOne = "oneVsOne", TwoVsTwo = "twoVsTwo",
}

export enum ClashRoyaleCard {
    Knight = 26000000, Archers = 26000001, Goblins = 26000002, Giant = 26000003, PEKKA = 26000004, Minions = 26000005, Balloon = 26000006, Witch = 26000007, Barbarians = 26000008, Golem = 26000009, Skeletons = 26000010, Valkyrie = 26000011, SkeletonArmy = 26000012, Bomber = 26000013, Musketeer = 26000014, BabyDragon = 26000015, Prince = 26000016, Wizard = 26000017, MiniPEKKA = 26000018, SpearGoblins = 26000019, GiantSkeleton = 26000020, HogRider = 26000021, MinionHorde = 26000022, IceWizard = 26000023, RoyalGiant = 26000024, Guards = 26000025, Princess = 26000026, DarkPrince = 26000027, ThreeMusketeers = 26000028, LavaHound = 26000029, IceSpirit = 26000030, FireSpirit = 26000031, Miner = 26000032, Sparky = 26000033, Bowler = 26000034, Lumberjack = 26000035, BattleRam = 26000036, InfernoDragon = 26000037, IceGolem = 26000038, MegaMinion = 26000039, DartGoblin = 26000040, GoblinGang = 26000041, ElectroWizard = 26000042, EliteBarbarians = 26000043, Hunter = 26000044, Executioner = 26000045, Bandit = 26000046, RoyalRecruits = 26000047, NightWitch = 26000048, Bats = 26000049, RoyalGhost = 26000050, RamRider = 26000051, Zappies = 26000052, Rascals = 26000053, CannonCart = 26000054, MegaKnight = 26000055, SkeletonBarrel = 26000056, FlyingMachine = 26000057, WallBreakers = 26000058, RoyalHogs = 26000059, GoblinGiant = 26000060, Fisherman = 26000061, MagicArcher = 26000062, ElectroDragon = 26000063, Firecracker = 26000064, ElixirGolem = 26000067, BattleHealer = 26000068, SkeletonDragons = 26000080, MotherWitch = 26000083, ElectroSpirit = 26000084, ElectroGiant = 26000085, Cannon = 27000000, GoblinHut = 27000001, Mortar = 27000002, InfernoTower = 27000003, BombTower = 27000004, BarbarianHut = 27000005, Tesla = 27000006, ElixirCollector = 27000007, XBow = 27000008, Tombstone = 27000009, Furnace = 27000010, GoblinCage = 27000012, GoblinDrill = 27000013, Fireball = 28000000, Arrows = 28000001, Rage = 28000002, Rocket = 28000003, GoblinBarrel = 28000004, Freeze = 28000005, Mirror = 28000006, Lightning = 28000007, Zap = 28000008, Poison = 28000009, Graveyard = 28000010, TheLog = 28000011, Tornado = 28000012, Clone = 28000013, Earthquake = 28000014, BarbarianBarrel = 28000015, HealSpirit = 28000016, GiantSnowball = 28000017, RoyalDelivery = 28000018,
}

export enum ClashRoyaleArena {
    Arena1 = 54000001, Arena2 = 54000002, Arena3 = 54000003, Arena4 = 54000004, Arena5 = 54000005, Arena6 = 54000006, Arena7 = 54000008, Arena8 = 54000009, Arena9 = 54000010, Arena10 = 54000007, Arena11 = 54000024, Arena12 = 54000011, Arena13 = 54000055, Arena14 = 54000056, LegendaryArena = 54000057, ChallengerII = 54000013, ChallengerIII = 54000014, MasterI = 54000015, MasterII = 54000016, MasterIII = 54000017, Champion = 54000018, GrandChampion = 54000019, RoyalChampion = 54000020, UltimateChampion = 54000031,
}