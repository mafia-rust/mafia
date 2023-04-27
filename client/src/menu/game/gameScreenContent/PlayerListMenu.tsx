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
    hideDead: boolean,
}

export default class PlayerListMenu extends React.Component<PlayerListMenuProps, PlayerListMenuState> {
    listener: () => void;

    constructor(props: PlayerListMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            hideDead: false,
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
                        }}>Reset Vote</button>
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
                        }}>Reset Targets</button>
                    </div>);
                }
                return null;
            default:
                return null;
        }
    }

    renderPlayer(player: Player){
        
        let canWhisper = 
            this.state.gameState.phase !== "night" && 
            GAME_MANAGER.gameState.phase !== null && 
            this.state.gameState.myIndex !== player.index;

        // let buttonCount = player.buttons.dayTarget + player.buttons.target + player.buttons.vote + canWhisper;

        return(<div key={player.index}>
            {player.toString()}<br/>

            <div>
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

                {((player)=>{if(canWhisper){return(
                    <button className="button gm-button" onClick={()=>{
                        ChatMenu.prependWhisper(player.index)
                }}>{translate("button.whisper")}</button>)}})(player)}

                <div></div>
            </div>

            
        </div>)
    }
    renderPlayers(players: Player[]){return<div>
        {
            players.map((player)=>{
                if(!this.state.hideDead || player.alive){
                    return this.renderPlayer(player);
                }
                return null;
            }, this)
        }
    </div>}

    render(){return(<div>
        
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.PlayerListMenu)}}>{translate("menu.playerList.title")}</button>

        <button className="button gm-button" onClick={()=>{
            this.setState({
                hideDead: !this.state.hideDead
            })
        }}>LANG TODO hide dead</button>
        <br/>
        <br/>
        {this.renderPhaseSpecific()}
        <br/>
        {this.renderPlayers(this.state.gameState.players)}
    </div>)}
}
