export function create_gameState(){
    return {
        myName: null,
        myIndex: null,

        chatMessages : [],  //string + chat messages

        players: [],
        graves: [],
        
        roleList: [],
        
        playerOnTrial: null,

        phase: null,
        secondsLeft: 0,
        dayNumber: 1,

        will: "",
        role: null,

        phaseTimes: {
            morning: 0,
            discussion: 0,
            voting: 0,
            testimony: 0,
            judgement: 0,
            evening: 0,
            night: 0,
        },

        targets: [],

        //my own data
            //My own role
            //who ive targeted
            //who ive voted
            //wheater ive voted innocent or guilty
            //what chats im currently talking to
    }
}

export function create_player(){
    return{
        //players
        //  suffixes
        name: "",
        buttons: {
            dayTarget: false,
            target: false,
            vote: false,
        },
        numVoted: null,
        alive:true,
    }
}


export function create_grave(){
    return{
        playerIndex: null,
    
        role: "",
        killer: "",
        will: "",
    
        diedPhase: "",
        dayNumber: null,
    }
}


