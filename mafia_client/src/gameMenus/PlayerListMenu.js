import React from "react";
import gameManager from "../game/gameManager";

export class PlayerListMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            listener : {func : ()=>{
                this.setState({
                })
            }},
        };
                
    }

    componentDidMount() {
        gameManager.addStateListner(this.state.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListner(this.state.listener);
    }

    renderPlayer(playerIndex){return(<div key={playerIndex}>
        {playerIndex}:<br/>
        <button>Target</button><button>Whisper</button><button>Vote</button><button>DayTarget</button> 
    </div>)}
    renderPlayers(){return<div>
        {/* {this.renderPlayer("Cotton Mather")}
        <br/>
        {this.renderPlayer("Johnathan Williams")}
        <br/>
        {this.renderPlayer("Sammy")}
        <br/> */}
    </div>}

    render(){return(<div>

        {this.renderPlayers()}
        
    </div>)}
}