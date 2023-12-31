import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { find } from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import ChatMenu from "./ChatMenu";
import GameState, { Player, PlayerIndex } from "../../../game/gameState.d";
import { ContentMenus, ContentTab } from "../GameScreen";
import { StateListener } from "../../../game/gameManager.d";
import SmallRoleSpecificMenu from "./RoleSpecificMenus/SmallRoleSpecificMenu";
import StyledText from "../../../components/StyledText";

type PlayerListMenuProps = {
}
type PlayerListMenuState = {
    gameState: GameState,
    playerFilter: PlayerFilter
}
type PlayerFilter = "all"|"living"|"usable";

export default class PlayerListMenu extends React.Component<PlayerListMenuProps, PlayerListMenuState> {
    listener: StateListener;

    constructor(props: PlayerListMenuProps) {
        super(props);

        
        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState : GAME_MANAGER.state,
                playerFilter: "living",
            };

        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType !== "game"){
                return;
            }

            let playerFilter = this.state.playerFilter;
            if(type==="phase"){
                if(
                    (
                        GAME_MANAGER.state.myIndex===null || 
                        GAME_MANAGER.state.players[GAME_MANAGER.state.myIndex].alive
                    ) && 
                    playerFilter !== "all"
                ){
                    if(GAME_MANAGER.state.phase === "night"){
                        playerFilter = "usable"
                    }else if(GAME_MANAGER.state.phase === "morning"){
                        playerFilter = "living";
                    }
                }
            }
            //if there are no usable players, switch to living
            if(playerFilter==="usable" && !GAME_MANAGER.state.players.some((player)=>{return Object.values(player.buttons).includes(true)})){
                playerFilter = "living";
            }
            //if there are no living players, switch to all
            if(playerFilter==="living" && !GAME_MANAGER.state.players.some((player)=>{return player.alive})){
                playerFilter = "all";
            }
            this.setState({
                gameState: GAME_MANAGER.state,
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
                {
                    player.numVoted!==null &&
                    player.numVoted!==0 &&
                    this.state.gameState.phase==="voting" ? 
                    player.numVoted+"-> ":""
                }
                <button className="whisper" onClick={()=>ChatMenu.prependWhisper(player.index)}>
                    <h4>
                        <StyledText>{(player.alive?"":" "+translate("tag.dead")+"")}</StyledText>
                    </h4>
                    <StyledText>{player.toString()}</StyledText>
                    <StyledText>{(player.roleLabel==null?"":("("+translate("role."+player.roleLabel+".name")+")"))}</StyledText>
                </button>
                <div className="playerTags">
                    <StyledText>{player.playerTags.map((tag)=>{return translate("tag."+tag)})}</StyledText>
                </div>
                {(() => {
                    const filter = find(player.name);
                    const isFilterSet = ChatMenu.getFilter()?.source === filter.source;
                    
                    return <button 
                        className={"material-icons-round filter" + (isFilterSet ? " highlighted" : "")} 
                        onClick={() => {isFilterSet ? ChatMenu.setFilter(null) : ChatMenu.setFilter(filter); this.setState({})}}
                        aria-label={translate("menu.playerList.button.filter")}
                    >
                        filter_alt
                    </button>
                })()}
            </div>
            

            <div className="buttons">
                <div className="day-target">
                    {((player)=>{if(player.buttons.dayTarget){
                        const highlighted = 
                            this.state.gameState.roleState?.role === "jailor" && 
                                this.state.gameState.roleState.jailedTargetRef === player.index;
                    return(
                        <button className={highlighted ? "highlighted" : undefined} onClick={()=>{
                            GAME_MANAGER.sendDayTargetPacket(player.index)}}
                    >{
                        translate("role."+this.state.gameState.roleState?.role+".dayTarget")
                    }</button>)}})(player)}
                </div>
                <div className="target">
                    {((player) => {
                        if(player.buttons.target) {
                            return <button onClick={() => {
                                if(GAME_MANAGER.state.stateType === "game")
                                    GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.state.targets, player.index])
                            }}>
                                {translate("role."+this.state.gameState.roleState?.role+".target")}
                            </button>
                        } else if (GAME_MANAGER.state.stateType === "game" && this.state.gameState.phase === "night" && this.state.gameState.targets.includes(player.index)) {
                            let newTargets = [...GAME_MANAGER.state.targets];
                            newTargets.splice(newTargets.indexOf(player.index), 1);
                            return <button className="highlighted" onClick={() => GAME_MANAGER.sendTargetPacket(newTargets)}>
                                {translate("role."+this.state.gameState.roleState?.role+".detarget")}
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

    render(){return(<div className="player-list-menu player-list-menu-colors">
        <ContentTab close={ContentMenus.PlayerListMenu}>{translate("menu.playerList.title")}</ContentTab>

        {this.renderFilterButton("all")}
        {this.renderFilterButton("living")}
        {this.renderFilterButton("usable")}

        <SmallRoleSpecificMenu/>
        {this.renderPhaseSpecific()}
        {this.renderPlayers(this.state.gameState.players)}
    </div>)}
}
