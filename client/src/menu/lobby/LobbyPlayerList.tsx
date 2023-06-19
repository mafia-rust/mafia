import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import { Player } from "../../game/gameState.d";
import { StateListener } from "../../game/gameManager.d";

interface PlayerListState {
    enteredName: string,
    players: Player[]
}

export default class LobbyPlayerList extends React.Component<{}, PlayerListState> {
    listener: StateListener;
    constructor(props: {}) {
        super(props);

        this.state = {     
            enteredName: "",
            players: GAME_MANAGER.gameState.players
        };
        this.listener = ()=>{
            this.setState({
                players: GAME_MANAGER.gameState.players
            });
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener("players", this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener("players", this.listener);
    }
    
    renderName(){return(
        <div className="name-box">
            <input type="text" value={this.state.enteredName}
                onChange={(e)=>{this.setState({enteredName: e.target.value})}}
                placeholder={translate("menu.lobby.field.namePlaceholder")}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        GAME_MANAGER.sendSetNamePacket(this.state.enteredName);
                }}
            />
            <button onClick={()=>{
                GAME_MANAGER.sendSetNamePacket(this.state.enteredName)
            }}>{translate("menu.lobby.button.setName")}</button>
        </div>
    )}

    renderPlayers(){return(<div>
        {this.state.players.map((player, i)=>{
            return(<div key={i}>{player.toString()}</div>)
        })}
    </div>)}

    render(){return(<section>
        {this.renderName()}
        {this.renderPlayers()}
    </section>)}
}