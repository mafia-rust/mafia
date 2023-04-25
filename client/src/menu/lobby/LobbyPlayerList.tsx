import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import { Player } from "../../game/gameState.d";
import { StateEventType } from "../../game/net/gameManager.d";

interface PlayerListState {
    name: string,
    players: Player[]
}

export default class LobbyPlayerList extends React.Component<any, PlayerListState> {
    listener: (type: StateEventType)=>void;
    constructor(props: any) {
        super(props);

        this.state = {            
            name: "",
            players: GAME_MANAGER.gameState.players
        };
        this.listener = (type)=>{
            if (type === "yourName") {
                this.setState({
                    name: GAME_MANAGER.gameState.myName!
                })
            }
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

    renderName(){return(<div className="lm-name-box">
        <input type="text" value={this.state.name}
            onChange={(e)=>{this.setState({name: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    GAME_MANAGER.sendSetNamePacket(this.state.name);
            }}
        />

        <button className="lm-set-name-button" onClick={()=>{
            GAME_MANAGER.sendSetNamePacket(this.state.name)
        }}>{translate("menu.lobby.button.setName")}</button>
    </div>)}

    renderPlayers(){return(<ol>
        {this.state.players.map((player, i)=>{
            return(<li key={i}>{player.toString()}</li>)
        })}
    </ol>)}
}