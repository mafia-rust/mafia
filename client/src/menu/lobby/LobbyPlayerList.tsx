import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import { LobbyPlayer, PlayerID } from "../../game/gameState.d";
import { StateListener } from "../../game/gameManager.d";
import StyledText from "../../components/StyledText";

type PlayerListState = {
    enteredName: string,
    players: Map<PlayerID, LobbyPlayer>,
    host: boolean
}

export default class LobbyPlayerList extends React.Component<{}, PlayerListState> {
    listener: StateListener;
    constructor(props: {}) {
        super(props);

        if(GAME_MANAGER.state.stateType === "lobby")
            this.state = {     
                enteredName: "",
                players: GAME_MANAGER.state.players,
                host: GAME_MANAGER.getMyHost() ?? false
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "lobby")
                this.setState({
                    players: GAME_MANAGER.state.players,
                    host: GAME_MANAGER.getMyHost() ?? false
                });
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
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

    renderPlayers() {

        let out = [];
        for(let [id, player] of this.state.players.entries()){
            out.push(<li key={id}>
                <StyledText>{player.name}</StyledText>
            </li>)
        }


        return <ol>
            {out}
        </ol>
    }

    render(){return(<section>
        {this.renderName()}
        {this.renderPlayers()}
    </section>)}
}