export function create_gameState(){
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


        //my own data
            //My own role
            //who ive voted
            //wheater ive voted innocent or guilty
            //what chats im currently talking to
        
        roleList: [],   //Vec<RoleListEntry>
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
        killer: [],
        will: "",
    
        diedPhase: "",
        dayNumber: null,
    }
}


