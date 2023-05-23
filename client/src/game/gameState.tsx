import GameState, { Player } from "./gameState.d"

export function createGameState(): GameState {
    return {
        myName: null,
        myIndex: null,

        chatMessages : [],
        graves: [],
        players: [],
        
        playerOnTrial: null,
        phase: null,
        secondsLeft: 0,
        dayNumber: 1,

        role: null,

        will: "",
        notes: "",
        targets: [],
        voted: null,
        judgement: "abstain",
        
        roleList: [],
        investigatorResults: [],
        phaseTimes: {
            morning: 5,
            discussion: 45, 
            voting: 30, 
            testimony: 20, 
            judgement: 20, 
            evening: 7, 
            night: 37,
        },
    }
}

export function createPlayer(name: string, index: number): Player {
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
        roleLabel: null,

        toString() {
            return "("+(this.index + 1)+") " + this.name;
        }
    }
}


