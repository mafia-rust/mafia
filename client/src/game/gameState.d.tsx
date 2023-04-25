import { Grave } from "./grave";

export default interface GameState {
    myName: string | null,
    myIndex: PlayerIndex | null,

    chatMessages : any[],
    graves: Grave[],
    players: Player[],
    
    playerOnTrial: PlayerIndex | null,
    phase: Phase | null,
    secondsLeft: number,
    dayNumber: number,

    role: Role | null,

    will: string,
    targets: PlayerIndex[],
    voted: PlayerIndex | null,
    judgement: Verdict | null,
    
    roleList: RoleListEntry[],
    investigatorResults: Role[][],
    phaseTimes: PhaseTimes
}

export type PlayerIndex = number;
export const enum Verdict {
    Innocent = "Innocent",
    Guilty = "Guilty",
    Abstain = "Abstain",
}
export const enum Phase {
    Morning = "Morning",
    Discussion = "Discussion",
    Voting = "Voting",
    Testimony = "Testimony",
    Judgement = "Judgement",
    Evening = "Evening",
    Night = "Night",
}
export interface PhaseTimes {
    [Phase.Morning]: number,
    [Phase.Discussion]: number,
    [Phase.Voting]: number,
    [Phase.Testimony]: number,
    [Phase.Judgement]: number,
    [Phase.Evening]: number,
    [Phase.Night]: number,
}

export interface Player {
    name: string,
    index: number
    buttons: {
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    },
    numVoted: number | null,
    alive: boolean,

    toString(): string
}

export type Role = string;
export type Faction = string;
export type FactionAlignment = string;

export type RoleListEntry = {
    type: "any"
} | {
    type: "faction"
    faction: Faction,
} | {
    type: "factionAlignment"
    faction: Faction,
    factionAlignment: FactionAlignment,
} | {
    type: "exact"
    faction: Faction,
    factionAlignment: FactionAlignment,
    role: Role,
};
