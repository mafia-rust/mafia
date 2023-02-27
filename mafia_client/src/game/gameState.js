export function create_gameState(){
    return {
        myName: null,

        chatMessages : [],  //string + chat messages

        players: [],
        

        //chatHistory
        //roleList
        
        //playerOnTrial
        //phase
        //  timeLeft

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
            vote: false
        },
        numVoted: 0,
        alive:true,
    }
}