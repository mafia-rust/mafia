import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import { Player } from "../../game/gameState.d";

interface PlayerListState {
    name: string,
    players: Player[]
}

export default class LobbyPlayerList extends React.Component<any, PlayerListState> {
    listener: ()=>void;
    constructor(props: any) {
        super(props);

        this.state = {            
            name: "",
            players: GAME_MANAGER.gameState.players
        };
        this.listener = ()=>{
            this.setState({
                players: GAME_MANAGER.gameState.players
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
        {this.state.players.map((player, i)=>{
            return(<div key={i}>{player.toString()}</div>)
        })}
    </div>)}
}