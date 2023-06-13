import React from "react";
import translate, { getChatElement, styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import ChatMenu, { textContent } from "./ChatMenu";
import GameState, { Player, PlayerIndex } from "../../../game/gameState.d";
import GameScreen, { ContentMenus } from "../GameScreen";
import { ChatMessage } from "../../../game/chatMessage";

interface PlayerListMenuProps {
}
interface PlayerListMenuState {
    gameState: GameState,
    hideUnusable: boolean,
    hideDead: boolean,
}

export default class PlayerListMenu extends React.Component<PlayerListMenuProps, PlayerListMenuState> {
    listener: () => void;

    constructor(props: PlayerListMenuProps) {
        super(props);

        let hideDeadByDefault = GAME_MANAGER.gameState.role !== "retributionist";

        this.state = {
            gameState : GAME_MANAGER.gameState,
            hideUnusable: false,
            hideDead: hideDeadByDefault,
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

    renderRoleSpecific(){
        switch(this.state.gameState.roleState?.role){
            case "jailor":
                if(this.state.gameState.phase==="night")
                    return styleText(""+this.state.gameState.roleState.executionsRemaining);

                let jailedString = this.state.gameState.roleState.jailedTargetRef!=null?
                    this.state.gameState.players[this.state.gameState.roleState.jailedTargetRef].toString():
                    translate("none");
                return styleText(jailedString+" "+this.state.gameState.roleState.executionsRemaining);
            case "doctor":
                return styleText(""+this.state.gameState.roleState.selfHealsRemaining);
            case "bodyguard":
                return styleText(""+this.state.gameState.roleState.selfShieldsRemaining);
            case "vigilante":
                if(this.state.gameState.roleState.willSuicide)
                    return styleText(translate("grave.killer.suicide"));
                return styleText(""+this.state.gameState.roleState.bulletsRemaining);
            case "veteran":
                return styleText(""+this.state.gameState.roleState.alertsRemaining);
            case "janitor":
                return styleText(""+this.state.gameState.roleState.cleansRemaining);
        }
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
                        }}>{translate("menu.playerList.button.resetVote")}</button>
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
                        }}>{translate("menu.playerList.button.resetTargets")}</button>
                    </div>);
                }
                return null;
            default:
                return null;
        }
    }

    renderPlayer(player: Player){
        return(<div className="player" key={player.index}>
            <div className="top">
                
                <button className="whisper" onClick={()=>ChatMenu.prependWhisper(player.index)}>
                    {styleText(
                        (
                            player.numVoted!==null &&
                            player.numVoted!==0 &&
                            this.state.gameState.phase==="voting" ? 
                            player.numVoted+" :":""
                        )+
                        player.toString()+
                        (player.roleLabel==null?"":(" ("+translate("role."+player.roleLabel+".name")+")"))+
                        (player.alive?"":" ("+translate("dead")+")")
                    )}
                </button>
                <button className="filter" onClick={()=>{
                    ChatMenu.setFilterFunction(
                        (message: ChatMessage) => {
                            return textContent(getChatElement(message, 0)).includes(player.name) || 
                            message.type === "phaseChange"
                        }
                    );
                }}>{translate("menu.playerList.button.filter")}</button>
            </div>
            

            <div className="buttons">
                <div className="day-target">
                    {((player)=>{if(player.buttons.dayTarget){return(
                        <button onClick={()=>{
                            GAME_MANAGER.sendDayTargetPacket(player.index)}}
                    >{
                        translate("role."+this.state.gameState.role+".dayTarget")
                    }</button>)}})(player)}
                </div>
                <div className="target">
                    {((player)=>{if(player.buttons.target){return(
                        <button onClick={()=>{
                            GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.gameState.targets, player.index]);
                        }}>{
                            translate("role."+this.state.gameState.role+".target")
                        }</button>
                    )}})(player)}
                </div>
                <div className="vote">
                    {((player)=>{if(player.buttons.vote){return(
                        <button onClick={()=>{
                            GAME_MANAGER.sendVotePacket(player.index)}}
                        >{translate("menu.playerList.button.vote")}</button>
                    )}})(player)}
                </div>
            </div>            
        </div>)
    }
    renderPlayers(players: Player[]){
        return<div className="player-list">{
            players.map((player: Player)=>{
            if(
                (!this.state.hideUnusable || (this.state.hideUnusable && 
                    (player.buttons.dayTarget || player.buttons.target || player.buttons.vote)
                    )
                ) && (!this.state.hideDead || player.alive)
                ){
                return this.renderPlayer(player);
            }else{
                return null;
            }
        })}</div>
    }

    render(){return(<div className="player-list-menu">
        
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.PlayerListMenu)}}>{translate("menu.playerList.title")}</button>        
        <label>
            <input type="checkbox"
                checked={this.state.hideUnusable}
                onChange={(checked)=>{
                    this.setState({
                        hideUnusable: checked.target.checked
                    });
                }
            }/>
            {translate("menu.playerList.button.hideUnusable")}
        </label>
        <label>
            <input type="checkbox"
                checked={this.state.hideDead}
                onChange={(checked)=>{
                    this.setState({
                        hideDead: checked.target.checked
                    });
                }
            }/>
            {translate("menu.playerList.button.hideDead")}
        </label>
        <div className="role-specific">{this.renderRoleSpecific()}</div>
        <div className="phase-specific">{this.renderPhaseSpecific()}</div>
        {this.renderPlayers(this.state.gameState.players)}
    </div>)}
}
