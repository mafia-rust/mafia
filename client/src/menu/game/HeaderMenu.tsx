import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import { PhaseType, Player, PlayerIndex, Verdict } from "../../game/gameState.d";
import GameScreen, { ContentMenu as GameScreenContentMenus } from "./GameScreen";
import ROLES from "../../resources/roles.json"
import "./headerMenu.css";
import { Role, RoleState } from "../../game/roleState.d";
import StyledText from "../../components/StyledText";
import { StateEventType } from "../../game/gameManager.d";
import Icon from "../../components/Icon";
import { Button } from "../../components/Button";


type HeaderMenuProps = {
    phase: PhaseType | null,
    chatMenuNotification: boolean,
}
type HeaderMenuState = {    
    phase: PhaseType | null,
    playerOnTrial: PlayerIndex | null,
    players: Player[],
    myIndex: PlayerIndex | null,
    judgement: Verdict,
    roleState: RoleState | null,
    dayNumber: number,
    timeLeftMs: number,
    fastForward: boolean,
}

export default class HeaderMenu extends React.Component<HeaderMenuProps, HeaderMenuState> {
    listener: (type: StateEventType | undefined) => void;
    
    constructor(props: HeaderMenuProps) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
            this.state = {
                phase: GAME_MANAGER.state.phaseState.type,
                playerOnTrial: 
                    GAME_MANAGER.state.phaseState.type === "testimony" ||
                    GAME_MANAGER.state.phaseState.type === "judgement" ||
                    GAME_MANAGER.state.phaseState.type === "finalWords" ? 
                        GAME_MANAGER.state.phaseState.playerOnTrial : null,
                players: GAME_MANAGER.state.players,
                myIndex: GAME_MANAGER.state.clientState.myIndex,
                judgement: GAME_MANAGER.state.clientState.judgement,
                roleState: GAME_MANAGER.state.clientState.roleState,
                dayNumber: GAME_MANAGER.state.dayNumber,
                timeLeftMs: GAME_MANAGER.state.timeLeftMs,
                fastForward: GAME_MANAGER.state.fastForward,
            };
        this.listener = (type) => {
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                switch (type) {
                    case "phase":
                    case "playerOnTrial":
                        if(type==="phase")
                            this.setState({
                                phase: GAME_MANAGER.state.phaseState.type,
                                dayNumber: GAME_MANAGER.state.dayNumber
                            });
                            
                        if(
                            GAME_MANAGER.state.phaseState.type === "testimony" ||
                            GAME_MANAGER.state.phaseState.type === "judgement" ||
                            GAME_MANAGER.state.phaseState.type === "finalWords"
                        )
                        this.setState({playerOnTrial: GAME_MANAGER.state.phaseState.playerOnTrial})
                    break;
                    case "gamePlayers":
                        this.setState({players: GAME_MANAGER.state.players})
                    break;
                    case "yourPlayerIndex":
                        this.setState({myIndex: GAME_MANAGER.state.clientState.myIndex})
                    break;
                    case "yourJudgement":
                        this.setState({judgement: GAME_MANAGER.state.clientState.judgement})
                    break;
                    case "yourRoleState":
                        this.setState({roleState: GAME_MANAGER.state.clientState.roleState})
                    break;
                    case "phaseTimeLeft":
                    case "tick":
                        this.setState({timeLeftMs: GAME_MANAGER.state.timeLeftMs})
                    break;
                    case "yourVoteFastForwardPhase":
                        this.setState({fastForward: GAME_MANAGER.state.fastForward})
                    break;
                }
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

        let phaseSpecific = null;

        switch(this.state.phase){
            case "testimony":
            case "finalWords":
                phaseSpecific = <div  className="highlighted">
                    <StyledText>
                        {this.state.players[this.state.playerOnTrial!]?.toString()}
                    </StyledText>
                </div>;
            break;
            case "judgement":
                if(this.state.playerOnTrial !== null){

                    let verdictButtons = null;
                    if (this.state.playerOnTrial === this.state.myIndex) {
                        verdictButtons = <div className="judgement-info">{translate("judgement.cannotVote.onTrial")}</div>;
                    } else if (!this.state.players[this.state.myIndex!].alive){
                        verdictButtons = <div className="judgement-info">{translate("judgement.cannotVote.dead")}</div>;
                    } else {
                        verdictButtons = <div className="judgement-info">
                            {this.renderVerdictButton("guilty")}
                            {this.renderVerdictButton("abstain")}
                            {this.renderVerdictButton("innocent")}
                        </div>;
                    }


                    phaseSpecific = <div className="highlighted">
                        <StyledText>
                            {this.state.players[this.state.playerOnTrial!]?.toString()}
                        </StyledText>
                        {verdictButtons}
                    </div>;
                }else{
                    return(<div> 
                        ERROR NO PLAYER ON TRIAL FOUND IN JUDGEMENT PHASE 
                    </div>);
                }
        }

        return phaseSpecific ? <div className="phase-specific">{phaseSpecific}</div> : null;
    }

    renderVerdictButton(verdict: Verdict) {
        return <Button
            highlighted={this.state.judgement === verdict}
            onClick={()=>{GAME_MANAGER.sendJudgementPacket(verdict)}}
        >
            <StyledText noLinks={true}>
                {translate("verdict." + verdict)}
            </StyledText>
        </Button>
    }
    
    renderMenuButtons(){
        return <div className="menu-buttons">
            <Button className="chat-menu-colors"
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.ChatMenu)}
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.ChatMenu)}
            >
                {this.props.chatMenuNotification?<div className="chat-notification highlighted">!</div>:null}
                {translate("menu.chat.icon")}
                <span className="mobile-hidden">{translate("menu.chat.title")}</span>
            </Button>
            <Button className="player-list-menu-colors"
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.PlayerListMenu)}
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.PlayerListMenu)}
            >
                {translate("menu.playerList.icon")}
                <span className="mobile-hidden">{translate("menu.playerList.title")}</span>
            </Button>
            <Button className="will-menu-colors" 
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WillMenu)}
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.WillMenu)}
            >
                {translate("menu.will.icon")}
                <span className="mobile-hidden">{translate("menu.will.title")}</span>
            </Button>
            {(()=>
                (
                    ROLES[this.state.roleState?.type as Role] === undefined || !ROLES[this.state.roleState?.type as Role].largeRoleSpecificMenu
                )?null:
                    <Button className="role-specific-colors" 
                        highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.RoleSpecificMenu)}
                        onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.RoleSpecificMenu)}
                    >
                        <StyledText noLinks={true}>
                            {translate("role."+this.state.roleState?.type+".name")}
                        </StyledText>
                    </Button>
            )()}
            <Button className="graveyard-menu-colors" 
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.GraveyardMenu)}
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.GraveyardMenu)}
            >
                {translate("menu.graveyard.icon")}
                <span className="mobile-hidden">{translate("menu.graveyard.title")}</span>
            </Button>
            <Button className="wiki-menu-colors"
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WikiMenu)} 
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.WikiMenu)}
            >
                {translate("menu.wiki.icon")}
                <span className="mobile-hidden">{translate("menu.wiki.title")}</span>
            </Button>
        </div>
    }
    renderPhase(){
        if(this.state.phase){
            return(<div>
                {translate("phase."+this.state.phase)} {this.state.dayNumber}⏳{Math.floor(this.state.timeLeftMs/1000)}
            </div>);
        }
        return null;
    }

    renderFastForwardButton(){
        return <Button 
            onClick={()=>GAME_MANAGER.sendVoteFastForwardPhase(true)}
            className="fast-forward-button"
            highlighted={this.state.fastForward}
        >
            <Icon>double_arrow</Icon>
        </Button>
    }

    render(){

        const DEFAULT_MAX_TIME = 60*1000;
        let timeBarPercentage = (this.state.timeLeftMs) * (100/DEFAULT_MAX_TIME);
        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.phaseTimes[this.state.phase!] !== undefined)
            //The 10 * is 100/1000. 100 for converting to percent. 1000 for converting to seconds.
            timeBarPercentage = this.state.timeLeftMs/(GAME_MANAGER.state.phaseTimes[this.state.phase!] * 10);

        const timerStyle = {
            height: "100%",
            backgroundColor: 'red',
            width: `${timeBarPercentage}%`,
            margin: '0 auto', // Center the timer horizontally
        };
        
        return <div className="header-menu">
            <h3>{this.renderPhase()}</h3>
            {(()=>{
                return <StyledText>
                    {(this.state.players[this.state.myIndex!] ?? "").toString() +
                    " (" + translate("role."+(this.state.roleState?.type)+".name") + ")"}
                </StyledText>;
            })()}
            {this.renderFastForwardButton()}
            {this.renderPhaseSpecific()}
            {this.renderMenuButtons()}
            <div className="timer-box">
                <div style={timerStyle}/>
            </div>
        </div>
    }
}