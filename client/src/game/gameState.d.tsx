
export default interface GameState {
    myName: string | null,
    myIndex: number | null,

    chatMessages : any[],  //string + chat messages
    graves: any[],
    players: Player[],
    
    playerOnTrial: number | null,    //Number:: player_index
    phase: string | null,    //String
    secondsLeft: number,
    dayNumber: number,

    role: string | null, //String::

    will: string,
    targets: number[],    //Vec<PlayerIndex>
    voted: number | null, //Number:: player_index
    judgement: string | null, //String:: Innocent, Guilty, Abstained


    //my own data
        //My own role
        //who ive voted
        //wheater ive voted innocent or guilty
        //what chats im currently talking to
    
    roleList: any[],   //Vec<RoleListEntry>
    investigatorResults: any[],   //Vec<Vec<Role>>
    phaseTimes: PhaseTimes
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
export interface Grave {
    player: Number,

    role: GraveRole,
    death_cause: GraveDeathCause,
    will: String,

    died_phase: GravePhase,
    day_number: Number,
}

type GraveRole = "Cleaned" | "Stoned" | Role;
type GraveDeathCause = "Lynching" | GraveKiller[];
type GraveKiller = "Mafia" | Role;
type Role = string;

export enum GravePhase {
    Day, 
    Night
}