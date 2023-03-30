import GameState, { Player } from "./gameState.d"

export function create_gameState(): GameState {
    return {
        myName: null,
        myIndex: null,

        chatMessages : [],  //string + chat messages
        graves: [],
        players: [],
        
        playerOnTrial: null,    //Number:: player_index
        phase: null,    //String
        secondsLeft: 0,
        dayNumber: 1,

        role: null, //String::

        will: "",
        targets: [],    //Vec<PlayerIndex>
        voted: null, //Number:: player_index
        judgement: null, //String:: Innocent, Guilty, Abstained
        
        roleList: [],   //Vec<RoleListEntry>
        investigatorResults: [],   //Vec<Vec<Role>>
        phaseTimes: {
            morning: 0,
            discussion: 0,
            voting: 0,
            testimony: 0,
            judgement: 0,
            evening: 0,
            night: 0,
        },
    }
}

export function create_player(name: string, index: number): Player {
    return{
        name: name,
        index: index,
        buttons: {
            dayTarget: false,
            target: false,
            vote: false,
        },
        numVoted: null,
        alive:true,

        toString() {
            return "("+(this.index + 1)+") " + this.name;
        }
    }
}

export function create_grave(){
    return{
        playerIndex: null,
    
        role: "",
        death_cause: [],
        will: "",
    
        diedPhase: "",
        dayNumber: null,
    }
}


