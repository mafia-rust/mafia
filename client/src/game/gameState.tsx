import GameState, { Phase, Player } from "./gameState.d"

export const ROLES: ReadonlyMap<string, any> = new Map(Object.entries(require("../resources/roles.json")));

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
        targets: [],
        voted: null,
        judgement: null,
        
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

        toString() {
            return "("+(this.index + 1)+") " + this.name;
        }
    }
}


