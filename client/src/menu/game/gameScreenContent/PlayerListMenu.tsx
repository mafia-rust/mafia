import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { regEscape } from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import ChatMenu from "./ChatMenu";
import GameState, { Player, PlayerIndex } from "../../../game/gameState.d";
import GameScreen, { ContentMenus } from "../GameScreen";
import { StateListener } from "../../../game/gameManager.d";
import SmallRoleSpecifcMenu from "./RoleSpecificMenus/SmallRoleSpecificMenu";
import Anchor from "../../Anchor";
import StyledText from "../../../components/StyledText";

interface PlayerListMenuProps {
}
interface PlayerListMenuState {
    gameState: GameState,
    playerFilter: PlayerFilter
}
type PlayerFilter = "all"|"living"|"usable";

export default class PlayerListMenu extends React.Component<PlayerListMenuProps, PlayerListMenuState> {
    listener: StateListener;

    constructor(props: PlayerListMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            playerFilter: Anchor.isMobile() ? "all" : "living",
        };
        this.listener = (type)=>{
            let playerFilter = this.state.playerFilter;
            if(type==="phase"){
                if(!Anchor.isMobile() && (GAME_MANAGER.gameState.myIndex===null || GAME_MANAGER.gameState.players[GAME_MANAGER.gameState.myIndex].alive)){
                    if(GAME_MANAGER.gameState.phase === "night"){
                        playerFilter = "usable"
                    }else if(GAME_MANAGER.gameState.phase === "morning"){
                        playerFilter = "living";
                    }
                }
            }
            this.setState({
                gameState: GAME_MANAGER.gameState,
                playerFilter: playerFilter
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
        let phaseSpecificJSX = null;
        switch(this.state.gameState.phase){
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
                            <StyledText>{player.playerTags.map((tag)=>{return translate("tag."+tag)})}</StyledText>
                            {(
                                player.numVoted!==null &&
                                player.numVoted!==0 &&
                                this.state.gameState.phase==="voting" ? 
                                player.numVoted+" :":""
                            )}
                            <StyledText>{player.toString()}</StyledText>
                            <StyledText>{(player.roleLabel==null?"":(" ("+translate("role."+player.roleLabel+".name")+")"))}</StyledText>
                            <StyledText>{(player.alive?"":" ("+translate("dead")+")")}</StyledText>
                </button>
                {(() => {
                    const filter = RegExp(`(?<!\\w)${regEscape(player.name)}(?!\\w)`, "i");
                    const isFilterSet = ChatMenu.getFilter()?.source === filter.source;
                    
                    return <button 
                        className={"filter" + (isFilterSet ? " highlighted" : "")} 
                        onClick={() => {isFilterSet ? ChatMenu.setFilter(null) : ChatMenu.setFilter(filter); this.setState({})}}
                    >
                        {translate("menu.playerList.button.filter")}
                    </button>
                })()}
            </div>
            

            <div className="buttons">
                <div className="day-target">
                    {((player)=>{if(player.buttons.dayTarget){
                        // This is a little hacky. TODO: Extract this to another function.
                        const highlighted = 
                            this.state.gameState.roleState?.role === "jailor" && 
                                this.state.gameState.roleState.jailedTargetRef === player.index;
                    return(
                        <button className={highlighted ? "highlighted" : undefined} onClick={()=>{
                            GAME_MANAGER.sendDayTargetPacket(player.index)}}
                    >{
                        translate("role."+this.state.gameState.role+".dayTarget")
                    }</button>)}})(player)}
                </div>
                <div className="target">
                    {((player) => {
                        if(player.buttons.target) {
                            return <button onClick={() => GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.gameState.targets, player.index])}>
                                {translate("role."+this.state.gameState.role+".target")}
                            </button>
                        } else if (this.state.gameState.phase === "night" && this.state.gameState.targets.includes(player.index)) {
                            let newTargets = [...GAME_MANAGER.gameState.targets];
                            newTargets.splice(newTargets.indexOf(player.index), 1);
                            return <button className="highlighted" onClick={() => GAME_MANAGER.sendTargetPacket(newTargets)}>
                                {translate("role."+this.state.gameState.role+".detarget")}
                            </button>
                        }
                    })(player)}
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
        <br/> {/* TODO remove this BR */}
        {this.renderFilterButton("all")}
        {this.renderFilterButton("living")}
        {this.renderFilterButton("usable")}

        <SmallRoleSpecifcMenu/>
        {this.renderPhaseSpecific()}
        {this.renderPlayers(this.state.gameState.players)}
    </div>)}
}
