import React, { ReactElement, useContext, useMemo } from "react";
import translate from "../../game/lang";
import { PhaseState, Verdict } from "../../game/gameState.d";
import "./headerMenu.css";
import StyledText from "../../components/StyledText";
import Icon from "../../components/Icon";
import { Button } from "../../components/Button";
import { GameScreenMenuContext, GameScreenMenuType, MENU_CSS_THEMES, MENU_TRANSLATION_KEYS } from "./GameScreenMenuContext";
import { GameStateContext, Player, usePlayerState } from "./GameStateContext";
import { MobileContext } from "../MobileContext";
import { WebsocketContext } from "../WebsocketContext";


export default function HeaderMenu(): ReactElement {
    const mobile = useContext(MobileContext)!;
    const phaseState = useContextGameState()!.phaseState;
    const host = useContextGameState()!.host !== null;

    const backgroundStyle = 
        phaseState.type === "briefing" ? "background-none" :
        (phaseState.type === "night" || phaseState.type === "obituary") ? "background-night" : 
        "background-day";


    const spectator = useContextGameState()!.clientState.type === "spectator";


    return <div className={"header-menu " + backgroundStyle}>
        {!(spectator && !host) && <FastForwardButton spectatorAndHost={spectator && host}/>}
        <Information />
        {!mobile && <MenuButtons/>}
        <Timer />
    </div>
}

function Timer(): ReactElement {
    let timeLeftMs = useContextGameState()!.timeLeftMs;
    if(timeLeftMs===null){timeLeftMs = 0};
    const phaseTimes = useContextGameState()!.phaseTimes;
    const phaseType = useContextGameState()!.phaseState.type;

    let phaseLength = 0
    if (phaseType !== "recess"){
        phaseLength = phaseTimes[phaseType]
    }

    const timerStyle = {
        height: "100%",
        backgroundColor: 'red',
        width: `${timeLeftMs / (phaseLength * 10)}%`,
        margin: '0 auto', // Center the timer horizontally
    };

    return <div className="timer-box">
        <div style={timerStyle}/>
    </div>
}

function Information(): ReactElement {
    const dayNumber = useContextGameState()!.dayNumber;
    let timeLeftMs = useContextGameState()!.timeLeftMs;
    if(timeLeftMs===null){timeLeftMs = 0};
    const phaseState = useContextGameState()!.phaseState;
    const players = useContextGameState()!.players;

    const playerState = usePlayerState();

    const myIndex = playerState!==undefined?playerState.myIndex:undefined;
    const roleState = playerState!==undefined?playerState.roleState:undefined;
    const myName = useMemo(() => {
        return myIndex === undefined ? undefined : players[myIndex]?.toString()
    }, [myIndex, players])


    const timeLeftText = useMemo(() => {
        if (timeLeftMs === null) {
            return "∞"
        } else {
            return Math.floor(timeLeftMs/1000);
        }
    }, [timeLeftMs])

    const dayNumberText = useMemo(() => {
        if (phaseState.type === "recess") {
            return "";
        } else {
            return ` ${dayNumber}`;
        }
    }, [dayNumber, phaseState.type])

    const spectator = useContextGameState()!.clientState.type === "spectator";
    

    return <div className="information"> 
        <div className="my-information">
            <div>
                <h3>
                    <div>
                        {translate("phase."+phaseState.type)}{dayNumberText}⏳{timeLeftText}
                    </div>
                </h3>
                {spectator || <StyledText>
                    {myName + " (" + translate("role."+(roleState!.type)+".name") + ")"}
                </StyledText>}
            </div>
        </div>
        <PhaseSpecificInformation players={players} myIndex={myIndex} phaseState={phaseState}/>
    </div>
}

export function PhaseSpecificInformation(props: Readonly<{
    phaseState: PhaseState,
    players: Player[],
    myIndex?: number
}>): ReactElement | null {
    const enabledModifiers = useContextGameState()!.enabledModifiers;
    const spectator = useContextGameState()!.clientState.type === "spectator";

    if (
        props.phaseState.type === "testimony"
        || props.phaseState.type === "finalWords"
        || props.phaseState.type === "judgement"
    ) {
        return <div className="phase-specific">
            <div className="highlighted">
                <StyledText>
                    {translate(`${props.phaseState.type}.playerOnTrial`, props.players[props.phaseState.playerOnTrial].toString())}
                </StyledText>
                {!spectator && props.phaseState.type === "judgement" && <div className="judgement-info">
                    {(() => {
                        if (props.phaseState.playerOnTrial === props.myIndex) {
                            return translate("judgement.cannotVote.onTrial");
                        } else if (!props.players[props.myIndex!].alive) {
                            return translate("judgement.cannotVote.dead");
                        } else {
                            return (
                                enabledModifiers.includes("abstaining") ? 
                                    ["guilty", "abstain", "innocent"] as const :
                                    ["guilty", "innocent"] as const 
                                ).map((verdict) => {
                                return <VerdictButton key={verdict} verdict={verdict}/>
                            })
                        }
                    })()}
                </div>}
            </div>
        </div>
        
    } else {
        return null;
    }
}

function VerdictButton(props: Readonly<{ verdict: Verdict }>) {
    const judgement = usePlayerState()!.judgement;
    const websocketContext = useContext(WebsocketContext)!;

    return <Button
        highlighted={judgement === props.verdict}
        onClick={()=>{websocketContext.sendJudgementPacket(props.verdict)}}
    >
        <StyledText noLinks={true}>
            {translate("verdict." + props.verdict)}
        </StyledText>
    </Button>
}

export function MenuButtons(): ReactElement | null {
    const menuController = useContext(GameScreenMenuContext)!;
    const missedChatMessages = useContextGameState()!;
    const chatMenuNotification = useMemo(
        ()=>missedChatMessages && !menuController.menuIsOpen(GameScreenMenuType.ChatMenu),
        [missedChatMessages, menuController.menusOpen()]
    );

    return <div className="menu-buttons">
        {menuController.menusAvailable().map(menu => {
            return <Button key={menu} className={MENU_CSS_THEMES[menu] ?? ""}
                highlighted={menuController.menusOpen().includes(menu)} 
                onClick={()=>{
                    if(menuController.menusOpen().includes(menu)){
                        menuController.closeMenu(menu)
                    }else{
                        menuController.openMenu(menu)
                    }
                }}
            >
                {menu === GameScreenMenuType.ChatMenu
                    && chatMenuNotification
                    && <div className="chat-notification highlighted">!</div>
                }
                {translate(MENU_TRANSLATION_KEYS[menu] + ".icon")}
                <span className="mobile-hidden">{translate(MENU_TRANSLATION_KEYS[menu] + ".title")}</span>
            </Button>
        })}
    </div>
}

export function FastForwardButton(props: { spectatorAndHost: boolean }): ReactElement {
    const fastForward = useContextGameState()!.fastForward;
    const websocketContext = useContext(WebsocketContext)!;

    return <Button 
        onClick={() => {
            if (props.spectatorAndHost) {
                websocketContext.sendHostSkipPhase()
            } else {
                websocketContext.sendVoteFastForwardPhase(!fastForward)
            }
        }}
        className="fast-forward-button"
        highlighted={fastForward}
    >
        <Icon>double_arrow</Icon>
    </Button>
}
