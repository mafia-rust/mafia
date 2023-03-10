import React from "react";
import gameManager from "../../index.js";
import "./lobbyMenu.css";

export class LobbyPlayerList extends React.Component {
    constructor(props) {
        super(props);

        this.state = {            
            name: "",

            // Player list
        };
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }
    
    render(){return(<div className="lm-player-list-pane">
        {this.renderName()}
        {this.renderPlayers()}
    </div>)}
 
    renderName(){return(<div className="input-box lm-name-box">
        <input type="text" value={this.state.name}
            onChange={(e)=>{this.setState({name: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    gameManager.setName_button(this.state.name);
            }}
        />
        <button onClick={()=>{gameManager.setName_button(this.state.name)}}>Set Name</button><br/>
    </div>)}

    renderPlayers(){return(<div>
        {gameManager.gameState.players.map((player, i)=>{
            return(<div key={i}>{player.name}<br/></div>)
        })}
    </div>)}
}