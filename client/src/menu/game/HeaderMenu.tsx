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
import { Button } from "../../components/FallibleButton";


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

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                phase: GAME_MANAGER.state.phase,
                playerOnTrial: GAME_MANAGER.state.playerOnTrial,
                players: GAME_MANAGER.state.players,
                myIndex: GAME_MANAGER.state.myIndex,
                judgement: GAME_MANAGER.state.judgement,
                roleState: GAME_MANAGER.state.roleState,
                dayNumber: GAME_MANAGER.state.dayNumber,
                timeLeftMs: GAME_MANAGER.state.timeLeftMs,
                fastForward: GAME_MANAGER.state.fastForward,
            };
        this.listener = (type) => {
            if(GAME_MANAGER.state.stateType === "game"){
                switch (type) {
                    case "phase":
                        this.setState({
                            phase: GAME_MANAGER.state.phase,
                            dayNumber: GAME_MANAGER.state.dayNumber
                        })
                    break;
                    case "playerOnTrial":
                        this.setState({playerOnTrial: GAME_MANAGER.state.playerOnTrial})
                    break;
                    case "gamePlayers":
                        this.setState({players: GAME_MANAGER.state.players})
                    break;
                    case "yourPlayerIndex":
                        this.setState({myIndex: GAME_MANAGER.state.myIndex})
                    break;
                    case "yourJudgement":
                        this.setState({judgement: GAME_MANAGER.state.judgement})
                    break;
                    case "yourRoleState":
                        this.setState({roleState: GAME_MANAGER.state.roleState})
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
                phaseSpecific = <div>
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


                    phaseSpecific = <div>
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
            </Button>
            <Button className="player-list-menu-colors"
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.PlayerListMenu)}
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.PlayerListMenu)}
            >{translate("menu.playerList.icon")}</Button>
            <Button className="will-menu-colors" 
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WillMenu)}
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.WillMenu)}
            >{translate("menu.will.icon")}</Button>
            {(()=>
                (
                    ROLES[this.state.roleState?.role as Role] === undefined || !ROLES[this.state.roleState?.role as Role].largeRoleSpecificMenu
                )?null:
                    <Button className="role-specific-colors" 
                        highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.RoleSpecificMenu)}
                        onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.RoleSpecificMenu)}
                    >
                        <StyledText noLinks={true}>
                            {translate("role."+this.state.roleState?.role+".name")}
                        </StyledText>
                    </Button>
            )()}
            <Button className="graveyard-menu-colors" 
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.GraveyardMenu)}
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.GraveyardMenu)}
            >{translate("menu.graveyard.icon")}</Button>
            <Button className="wiki-menu-colors"
                highlighted={GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WikiMenu)} 
                onClick={()=>GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.WikiMenu)}
            >{translate("menu.wiki.icon")}</Button>
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
        const timerStyle = {
            height: "100%",
            backgroundColor: 'red',
            width: `${(this.state.timeLeftMs) * (100/(60*1000))}%`,
            margin: '0 auto', // Center the timer horizontally
        };
        
        return <div className="header-menu">
            <h3>{this.renderPhase()}</h3>
            {(()=>{
                return <StyledText>
                    {(this.state.players[this.state.myIndex!] ?? "").toString() +
                    " (" + translate("role."+(this.state.roleState?.role ?? "unknown")+".name") + ")"}
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