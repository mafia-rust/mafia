import React, { ReactElement, useContext, useMemo } from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import { PhaseState, Player, Verdict } from "../../game/gameState.d";
import "./headerMenu.css";
import StyledText from "../../components/StyledText";
import Icon from "../../components/Icon";
import { Button } from "../../components/Button";
import { GameScreenMenuContext, GameScreenMenuType, MENU_CSS_THEMES, MENU_TRANSLATION_KEYS } from "./GameScreenMenuContext";
import { GameStateContext } from "./GameStateContext";
import { MobileContext } from "../MobileContext";


export default function HeaderMenu(): ReactElement {
    const mobile = useContext(MobileContext)!;
    const phaseState = useContext(GameStateContext)!.phaseState;
    const host = useContext(GameStateContext)!.host !== null;

    const backgroundStyle = 
        phaseState.type === "briefing" ? "background-none" :
        (phaseState.type === "night" || phaseState.type === "obituary") ? "background-night" : 
        "background-day";


    const spectator = useContext(GameStateContext)!.clientState.type === "spectator";


    return <div className={"header-menu " + backgroundStyle}>
        {!(spectator && !host) && <FastForwardButton spectatorAndHost={spectator && host}/>}
        <Information />
        {!mobile && <MenuButtons/>}
        <Timer />
    </div>
}

function Timer(): ReactElement {
    let timeLeftMs = useContext(GameStateContext)!.timeLeftMs;
    if(timeLeftMs===null){timeLeftMs = 0};
    const phaseTimes = useContext(GameStateContext)!.phaseTimes;
    const phaseType = useContext(GameStateContext)!.phaseState.type;

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
    const dayNumber = useContext(GameStateContext)!.dayNumber;
    let timeLeftMs = useContext(GameStateContext)!.timeLeftMs;
    if(timeLeftMs===null){timeLeftMs = 0};
    const phaseState = useContext(GameStateContext)!.phaseState;
    const players = useContext(GameStateContext)!.players;

    const myIndex = usePlayerState(
        gameState => gameState.myIndex,
        ["yourPlayerIndex"]
    )
    const roleState = usePlayerState(
        clientState => clientState.roleState,
        ["yourRoleState"]
    )
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

    const spectator = useContext(GameStateContext)!.clientState.type === "spectator";
    

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
    myIndex: number | undefined
}>): ReactElement | null {
    const enabledModifiers = useContext(GameStateContext)!.enabledModifiers;
    const spectator = useContext(GameStateContext)!.clientState.type === "spectator";

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
    const judgement = usePlayerState(
        clientState => clientState.judgement,
        ["yourJudgement"]
    )!

    return <Button
        highlighted={judgement === props.verdict}
        onClick={()=>{GAME_MANAGER.sendJudgementPacket(props.verdict)}}
    >
        <StyledText noLinks={true}>
            {translate("verdict." + props.verdict)}
        </StyledText>
    </Button>
}

export function MenuButtons(): ReactElement | null {
    const menuController = useContext(GameScreenMenuContext)!;
    const missedChatMessages = useContext(GameStateContext)!;
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
    const fastForward = useContext(GameStateContext)!.fastForward;

    return <Button 
        onClick={() => {
            if (props.spectatorAndHost) {
                GAME_MANAGER.sendHostSkipPhase()
            } else {
                GAME_MANAGER.sendVoteFastForwardPhase(!fastForward)
            }
        }}
        className="fast-forward-button"
        highlighted={fastForward}
    >
        <Icon>double_arrow</Icon>
    </Button>
}
