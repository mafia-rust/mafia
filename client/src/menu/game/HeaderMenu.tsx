import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import { Phase, Player, PlayerIndex, Verdict } from "../../game/gameState.d";
import GameScreen, { ContentMenu as GameScreenContentMenus } from "./GameScreen";
import ROLES from "../../resources/roles.json"
import "./headerMenu.css";
import { Role, RoleState } from "../../game/roleState.d";
import StyledText from "../../components/StyledText";
import { StateEventType } from "../../game/gameManager.d";


type HeaderMenuProps = {
    phase: Phase | null,
    chatMenuNotification: boolean,
}
type HeaderMenuState = {    
    phase: Phase | null,
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
        switch(this.state.phase){
            case "judgement":
            if(this.state.playerOnTrial !== null){
                return(<div className="judgement-specific">
                    <div>
                        <StyledText>
                            {this.state.players[this.state.playerOnTrial!]?.toString()}
                        </StyledText>
                    {(()=>{
                        if (this.state.playerOnTrial === this.state.myIndex) {
                            return <div className="judgement-info">{translate("judgement.cannotVote.onTrial")}</div>;
                        } else if (!this.state.players[this.state.myIndex!].alive){
                            return <div className="judgement-info">{translate("judgement.cannotVote.dead")}</div>;
                        } else {
                            return(<div className="judgement-info">
                                {this.renderVerdictButton("guilty")}
                                {this.renderVerdictButton("abstain")}
                                {this.renderVerdictButton("innocent")}
                            </div>);
                        }
                    })()}
                    </div>
                </div>);
            }else{
                return(<div> 
                    ERROR NO PLAYER ON TRIAL FOUND IN JUDGEMENT PHASE TODO 
                </div>);
            }
            
            default:
            return null;
        }
    }

    renderVerdictButton(verdict: Verdict) {
        return <button
            className={this.state.judgement === verdict ? "highlighted" : undefined}
            onClick={()=>{GAME_MANAGER.sendJudgementPacket(verdict)}}
        >
            <StyledText noLinks={true}>
                {translate("verdict." + verdict)}
            </StyledText>
        </button>
    }
    
    renderMenuButtons(){
        return <div className="menu-buttons">
            <button
            className={"chat-menu-colors"+(GameScreen.instance.menusOpen().includes(GameScreenContentMenus.ChatMenu)?" highlighted":"")}
            onClick={()=>{
                GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.ChatMenu);
            }}>
                {this.props.chatMenuNotification?<div className="chat-notification highlighted">!</div>:null}
                {translate("menu.chat.icon")}
            </button>
            
            <button 
            className={"player-list-menu-colors"+ (GameScreen.instance.menusOpen().includes(GameScreenContentMenus.PlayerListMenu)?" highlighted":"")} 
            onClick={()=>{
                GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.PlayerListMenu)
            
            }}>{translate("menu.playerList.icon")}</button>
            <button 
            className={"will-menu-colors"+(GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WillMenu)?" highlighted":"")} 
            onClick={()=>{
                GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.WillMenu)
            }}>{translate("menu.will.icon")}</button>
            {(()=>
                (
                    ROLES[this.state.roleState?.role as Role] === undefined || !ROLES[this.state.roleState?.role as Role].largeRoleSpecificMenu
                )?null:
                    <button 
                    className={"role-specific-colors" + (GameScreen.instance.menusOpen().includes(GameScreenContentMenus.RoleSpecificMenu)?" highlighted":"")} 
                    onClick={()=>{
                        GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.RoleSpecificMenu)
                    
                    }}>
                        <StyledText noLinks={true}>
                            {translate("role."+this.state.roleState?.role+".name")}
                        </StyledText>
                    </button>
            )()}
            <button 
            className={"graveyard-menu-colors"+(GameScreen.instance.menusOpen().includes(GameScreenContentMenus.GraveyardMenu)?" highlighted":"")} 
            onClick={()=>{
                GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.GraveyardMenu)
            }}>{translate("menu.graveyard.icon")}</button>
            <button 
            className={"wiki-menu-colors"+(GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WikiMenu)?" highlighted":"")} 
            onClick={()=>{
                GameScreen.instance.closeOrOpenMenu(GameScreenContentMenus.WikiMenu)
            
            }}>{translate("menu.wiki.icon")}</button>

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
        return <button 
            onClick={()=>{
                GAME_MANAGER.sendVoteFastForwardPhase(true);
            }}
            className={"material-icons-round fast-forward-button" + (this.state.fastForward ? " highlighted" : "")}
        >
            double_arrow
        </button>
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