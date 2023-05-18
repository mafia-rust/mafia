import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import ChatMenu from "./ChatMenu";
import GameState, { Player, PlayerIndex } from "../../../game/gameState.d";
import GameScreen, { ContentMenus } from "../GameScreen";

interface PlayerListMenuProps {
}
interface PlayerListMenuState {
    gameState: GameState,
    showAllPlayers: boolean,
}

export default class PlayerListMenu extends React.Component<PlayerListMenuProps, PlayerListMenuState> {
    listener: () => void;

    constructor(props: PlayerListMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            showAllPlayers: false,
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
            })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    renderPhaseSpecific(){
        switch(this.state.gameState.phase){
            case "voting":
                let votedString = "";
                if(this.state.gameState.voted!=null){
                    votedString = this.state.gameState.players[this.state.gameState.voted].name;
                    return(<div>
                        <div>{votedString}</div>
                        <button className="button gm-button" onClick={()=>{
                            GAME_MANAGER.sendVotePacket(null);
                        }}>Reset Vote LANG</button>
                    </div>);
                }
                return null;
            case "night":
                let targetStringList = this.state.gameState.targets.map((playerIndex: PlayerIndex)=>{
                    return this.state.gameState.players[playerIndex].toString();
                });

                if(targetStringList.length > 0){
                    return(<div>
                        <div>{targetStringList.join(", ")+"."}</div>
                        <button className="button gm-button" onClick={()=>{
                            GAME_MANAGER.sendTargetPacket([]);
                        }}>Reset Targets LANG</button>
                    </div>);
                }
                return null;
            default:
                return null;
        }
    }

    renderPlayer(player: Player, whisperButton: boolean){

        return(<div key={player.index}>
            {player.toString()} { player.roleLabel==null?"":("("+player.roleLabel+")") }<br/>

            <div style={{display: "flex"}}>
                {((player)=>{if(player.buttons.target){return(
                    <button className="button gm-button" onClick={()=>{
                        GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.gameState.targets, player.index]);
                    }}>{translate("button.target")}</button>
                )}})(player)}

                {((player)=>{if(player.buttons.vote){return(
                    <button className="button gm-button" onClick={()=>{
                        GAME_MANAGER.sendVotePacket(player.index)}}
                    >{translate("button.vote")}</button>
                )}})(player)}

                {((player)=>{if(player.buttons.dayTarget){return(
                    <button className="button gm-button" onClick={()=>{
                        GAME_MANAGER.sendDayTargetPacket(player.index)}}
                >{translate("button.dayTarget")}</button>)}})(player)}

                {((player)=>{if(whisperButton){return(
                    <button className="button gm-button" onClick={()=>{
                        ChatMenu.prependWhisper(player.index)
                }}>{translate("button.whisper")}</button>)}})(player)}

                <div></div>
            </div>

            
        </div>)
    }
    renderPlayers(players: Player[]){
        
        let playersHTML = [];
        for(let i = 0; i < players.length; i++){
            
            let canWhisper = 
                GAME_MANAGER.gameState.phase !== "night" && 
                GAME_MANAGER.gameState.myIndex !== players[i].index;

            if(
                this.state.showAllPlayers || (!this.state.showAllPlayers && (
                    players[i].buttons.dayTarget || 
                    players[i].buttons.target || 
                    players[i].buttons.vote || 
                    canWhisper
                )))
                    playersHTML.push(this.renderPlayer(players[i], canWhisper));
        }
        

        return<div>
        {
            playersHTML
        }
        </div>
    }

    render(){return(<div>
        
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.PlayerListMenu)}}>{translate("menu.playerList.title")}</button>
        <br/>
        
        <label>
            <input type="checkbox"
                checked={this.state.showAllPlayers}
                onChange={(checked)=>{
                    this.setState({
                        showAllPlayers: checked.target.checked
                    }); 
                }
            }/>
            {translate("menu.playerList.button.showAll")}
        </label>
        
        <br/>
        <br/>
        {this.renderPhaseSpecific()}
        <br/>
        {this.renderPlayers(this.state.gameState.players)}
    </div>)}
}
