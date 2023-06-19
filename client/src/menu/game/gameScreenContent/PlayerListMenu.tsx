import React from "react";
import translate, { getChatElement, styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import ChatMenu, { textContent } from "./ChatMenu";
import GameState, { Player, PlayerIndex } from "../../../game/gameState.d";
import GameScreen, { ContentMenus } from "../GameScreen";
import { ChatMessage } from "../../../game/chatMessage";
import { StateListener } from "../../../game/gameManager.d";

interface PlayerListMenuProps {
}
interface PlayerListMenuState {
    gameState: GameState,
    playerFilter: PlayerFilter
}
type PlayerFilter = "all"|"living"|"usable";

export default class PlayerListMenu extends React.Component<PlayerListMenuProps, PlayerListMenuState> {
    gameStatelistener: StateListener;
    phaseListener: StateListener;

    constructor(props: PlayerListMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            playerFilter: "living",
        };
        this.gameStatelistener = () => { this.setState({ gameState: GAME_MANAGER.gameState }) };  
        this.phaseListener = () => {
            if (GAME_MANAGER.gameState.phaseState?.phase === "night") {
                this.setState({ playerFilter: "usable" });
            } else if (GAME_MANAGER.gameState.phaseState?.phase === "morning"){
                this.setState({ playerFilter: "living" });
            }
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener("tick", this.gameStatelistener);
        GAME_MANAGER.addStateListener("phaseState", this.phaseListener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener("tick", this.gameStatelistener);
        GAME_MANAGER.addStateListener("phaseState", this.phaseListener);
    }

    renderRoleSpecific(){
        let roleSpecificJSX = null;
        switch(this.state.gameState.roleState?.role){
            case "jailor":
                if(this.state.gameState.phaseState?.phase==="night")
                    roleSpecificJSX = styleText(""+this.state.gameState.roleState.executionsRemaining);
                else
                {
                    let jailedString = this.state.gameState.roleState.jailedTargetRef!=null?
                        this.state.gameState.players[this.state.gameState.roleState.jailedTargetRef].toString():
                        translate("none");
                    roleSpecificJSX = styleText(jailedString+" "+this.state.gameState.roleState.executionsRemaining);
                }
                break;
            case "doctor":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.selfHealsRemaining);
                break;
            case "bodyguard":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.selfShieldsRemaining);
                break;
            case "vigilante":
                if(this.state.gameState.roleState.willSuicide)
                    roleSpecificJSX = styleText(translate("grave.killer.suicide"));
                else
                    roleSpecificJSX = styleText(""+this.state.gameState.roleState.bulletsRemaining);
                    break;
            case "veteran":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.alertsRemaining);
                break;
            case "janitor":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.cleansRemaining);
                break;
        }
        if(roleSpecificJSX!==null){
            return <div className="role-specific">{roleSpecificJSX}</div>
        }
        return null
    }
    renderPhaseSpecific(){
        let phaseSpecificJSX = null;
        switch(this.state.gameState.phaseState?.phase){
            case "voting":
                let votedString = "";
                if(this.state.gameState.voted!=null){
                    votedString = this.state.gameState.players[this.state.gameState.voted].name;
                    phaseSpecificJSX = (<div>
                        <div>{votedString}</div>
                        <button className="button gm-button" onClick={()=>{
                            GAME_MANAGER.sendVotePacket(null);
                        }}>{translate("menu.playerList.button.resetVote")}</button>
                    </div>);
                }
                else
                    phaseSpecificJSX = null;
                break;
            case "night":
                let targetStringList = this.state.gameState.targets.map((playerIndex: PlayerIndex)=>{
                    return this.state.gameState.players[playerIndex].toString();
                });

                if(targetStringList.length > 0){
                    phaseSpecificJSX = (<div>
                        <div>{targetStringList.join(", ")+"."}</div>
                        <button className="button gm-button" onClick={()=>{
                            GAME_MANAGER.sendTargetPacket([]);
                        }}>{translate("menu.playerList.button.resetTargets")}</button>
                    </div>);
                }
                else
                    phaseSpecificJSX =  null;
        }
        
        if(phaseSpecificJSX!==null){
            return <div className="phase-specific">{phaseSpecificJSX}</div>
        }
        return null;
    }

    renderPlayer(player: Player){
        return(<div className="player" key={player.index}>
            <div className="top">
                <button className="whisper" onClick={()=>ChatMenu.prependWhisper(player.index)}>
                    {styleText(
                        (
                            player.numVoted!==null &&
                            player.numVoted!==0 &&
                            this.state.gameState.phaseState?.phase==="voting" ? 
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
                        translate("role."+this.state.gameState.roleState.role+".dayTarget")
                    }</button>)}})(player)}
                </div>
                <div className="target">
                    {((player)=>{if(player.buttons.target){return(
                        <button onClick={()=>{
                            GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.gameState.targets, player.index]);
                        }}>{
                            translate("role."+this.state.gameState.roleState.role+".target")
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
        return <div className="player-list">{
            players.filter((player: Player) => {
                switch(this.state.playerFilter){
                    case "all": return true;
                    case "living": return player.alive;
                    case "usable": return Object.values(player.buttons).includes(true);
                    default: return false;
                }
            }).map(player => this.renderPlayer(player))
        }</div>
    }

    renderFilterButton(filter: PlayerFilter) {
        return <button 
            className={this.state.playerFilter === filter ? "highlighted" : undefined}
            onClick={()=>{this.setState({playerFilter: filter})}}
        >
            {translate("menu.playerList.button." + filter)}
        </button>
    }

    render(){return(<div className="player-list-menu">
        
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.PlayerListMenu)}}>{translate("menu.playerList.title")}</button>
        
        {this.renderFilterButton("all")}
        {this.renderFilterButton("living")}
        {this.renderFilterButton("usable")}

        {this.renderRoleSpecific()}
        {this.renderPhaseSpecific()}
        {this.renderPlayers(this.state.gameState.players)}
    </div>)}
}
