import { Grave } from "./grave";

export default interface GameState {
    myName: string | null,
    myIndex: PlayerIndex | null,

    chatMessages : any[],  //string + chat messages
    graves: Grave[],
    players: Player[],
    
    playerOnTrial: PlayerIndex | null,    //Number:: player_index
    phase: Phase | null,    //String
    secondsLeft: number,
    dayNumber: number,

    role: Role | null, //String::

    will: string,
    targets: PlayerIndex[],    //Vec<PlayerIndex>
    voted: PlayerIndex | null, //Number:: player_index
    judgement: Verdict | null, //String:: Innocent, Guilty, Abstained
    
    roleList: RoleListEntry[],   //Vec<RoleListEntry>
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
