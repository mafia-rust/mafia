import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import { Phase, Player, PlayerIndex } from "../../../game/gameState.d";
import { ContentMenu, ContentTab } from "../GameScreen";
import { StateListener } from "../../../game/gameManager.d";
import SmallRoleSpecificMenu from "./RoleSpecificMenus/SmallRoleSpecificMenu";
import StyledText from "../../../components/StyledText";
import { RoleState } from "../../../game/roleState.d";

type PlayerListMenuProps = {
}
type PlayerListMenuState = {
    players: Player[],
    phase: Phase | null,
    voted: PlayerIndex | null,
    targets: PlayerIndex[],
    roleState: RoleState | null,
    playerFilter: PlayerFilter,
    chatFilter: PlayerIndex | null
}
type PlayerFilter = "all"|"living"|"usable";

export default class PlayerListMenu extends React.Component<PlayerListMenuProps, PlayerListMenuState> {
    listener: StateListener;
    updatePlayerFilter: () => void;

    constructor(props: PlayerListMenuProps) {
        super(props);

        
        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                players: GAME_MANAGER.state.players,
                phase: GAME_MANAGER.state.phase,
                voted: GAME_MANAGER.state.voted,
                targets: GAME_MANAGER.state.targets,
                roleState: GAME_MANAGER.state.roleState,
                playerFilter: "living",
                chatFilter: null
            };

        this.updatePlayerFilter = () => {
            if(GAME_MANAGER.state.stateType !== "game"){
                return;
            }

            let playerFilter = this.state.playerFilter;
            if(
                (GAME_MANAGER.state.myIndex===null || GAME_MANAGER.state.players[GAME_MANAGER.state.myIndex].alive) && 
                playerFilter !== "all"
            ){
                if(GAME_MANAGER.state.phase === "night"){
                    playerFilter = "usable"
                }else if(GAME_MANAGER.state.phase === "obituary"){
                    playerFilter = "living";
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
                playerFilter: playerFilter
            })
        };

        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType !== "game"){
                return;
            }
            switch (type) {
                case "tick":
                    this.setState({chatFilter: GAME_MANAGER.state.chatFilter});
                break;
                case "phase":
                    this.setState({ phase: GAME_MANAGER.state.phase })
                break;
                case "gamePlayers":
                case "yourButtons":
                case "playerAlive":
                case "yourPlayerTags":
                case "yourRoleLabels":
                case "playerVotes":
                    this.setState({ players: GAME_MANAGER.state.players })
                break;
                case "yourVoting":
                    this.setState({ voted: GAME_MANAGER.state.voted })
                break;
                case "yourTarget":
                    this.setState({ targets: GAME_MANAGER.state.targets })
                break;
                case "yourRoleState":
                    this.setState({ roleState: GAME_MANAGER.state.roleState })
                break;
            }
            switch (type) {
                case "phase":
                case "gamePlayers":
                case "yourVoting":
                case "yourTarget":
                case "yourRoleState":
                    this.updatePlayerFilter();
                break;
            }
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
        switch(this.state.phase){
            case "voting":
                if(this.state.voted!=null){
                    phaseSpecificJSX = (<div>
                        <div><StyledText>{this.state.players[this.state.voted].toString()}</StyledText></div>
                        <button className="button gm-button" onClick={()=>{
                            GAME_MANAGER.sendVotePacket(null);
                        }}>{translate("menu.playerList.button.resetVote")}</button>
                    </div>);
                }
                else
                    phaseSpecificJSX = null;
                break;
            case "night":
                let targetStringList = this.state.targets.map((playerIndex: PlayerIndex)=>{
                    return this.state.players[playerIndex].toString();
                });

                if(targetStringList.length > 0){
                    phaseSpecificJSX = (<div>
                        <div><StyledText>{targetStringList.join(", ")+"."}</StyledText></div>
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
                {(
                    player.numVoted !==null &&
                    player.numVoted !==0 &&
                    this.state.phase === "voting"
                ) ?
                    <span className="keyword-player-number">
                        {player.numVoted}
                    </span>
                : null}
                <button className="whisper" onClick={()=>{GAME_MANAGER.prependWhisper(player.index)}}>
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

                    const filter = player.index;
                    const isFilterSet = this.state.chatFilter === filter;
                    
                    return <button 
                        className={"material-icons-round filter" + (isFilterSet ? " highlighted" : "")} 
                        onClick={() => {
                            if(GAME_MANAGER.state.stateType === "game"){
                                GAME_MANAGER.state.chatFilter = isFilterSet ? null : filter;
                                GAME_MANAGER.invokeStateListeners("tick");
                            }
                            this.setState({})
                        }}
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
                            (this.state.roleState?.role === "jailor" && this.state.roleState.jailedTargetRef === player.index)
                            || 
                            (this.state.roleState?.role === "medium" && this.state.roleState.seancedTarget === player.index)
                            || 
                            (this.state.roleState?.role === "journalist" && this.state.roleState.interviewedTarget === player.index)
                    return(
                        <button className={highlighted ? "highlighted" : undefined} onClick={()=>{
                            GAME_MANAGER.sendDayTargetPacket(player.index)}}
                    >{
                        translate("role."+this.state.roleState?.role+".dayTarget")
                    }</button>)}})(player)}
                </div>
                <div className="target">
                    {((player) => {
                        if(player.buttons.target) {
                            return <button onClick={() => {
                                if(GAME_MANAGER.state.stateType === "game")
                                    GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.state.targets, player.index])
                            }}>
                                {translate("role."+this.state.roleState?.role+".target")}
                            </button>
                        } else if (GAME_MANAGER.state.stateType === "game" && this.state.phase === "night" && this.state.targets.includes(player.index)) {
                            let newTargets = [...GAME_MANAGER.state.targets];
                            newTargets.splice(newTargets.indexOf(player.index), 1);
                            return <button className="highlighted" onClick={() => GAME_MANAGER.sendTargetPacket(newTargets)}>
                                {translate("cancel")}
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
        <ContentTab close={ContentMenu.PlayerListMenu} helpMenu={"standard/playerList"}>{translate("menu.playerList.title")}</ContentTab>

        <div>
            {this.renderFilterButton("all")}
            {this.renderFilterButton("living")}
            {this.renderFilterButton("usable")}
        </div>
        <SmallRoleSpecificMenu/>
        {this.renderPhaseSpecific()}
        {this.renderPlayers(this.state.players)}
    </div>)}
}
