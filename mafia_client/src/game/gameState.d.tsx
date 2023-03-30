
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
    phaseTimes: {
        morning: number,
        discussion: number,
        voting: number,
        testimony: number,
        judgement: number,
        evening: number,
        night: number,
    },
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