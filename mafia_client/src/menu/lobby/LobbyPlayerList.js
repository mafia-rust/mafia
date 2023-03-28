import React from "react";
import translate from "@game/lang";
import GAME_MANAGER from "@";
import "./lobbyMenu.css";

export default class LobbyPlayerList extends React.Component {
    constructor(props) {
        super(props);

        this.state = {            
            name: "",

            // Player list
            gameState: GAME_MANAGER.gameState
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
            });
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    
    render(){return(<div className="lm-player-list-pane">
        {this.renderName()}
        {this.renderPlayers()}
    </div>)}
 
    renderName(){return(<div className="input-box lm-name-box">
        <input className="input-field" type="text" value={this.state.name}
            onChange={(e)=>{this.setState({name: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    GAME_MANAGER.setName_button(this.state.name);
            }}
        />
        <button className="button" onClick={()=>{
            GAME_MANAGER.setName_button(this.state.name)
        }}>{translate("menu.lobby.button.set_name")}</button>
    </div>)}

    renderPlayers(){return(<div>
        {this.state.gameState.players.map((_, i)=>{
            return(<div key={i}>{GAME_MANAGER.getPlayer(i)}</div>)
        })}
    </div>)}
}