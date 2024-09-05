import React, { ReactElement, useContext, useMemo } from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import { PhaseState, Player, Verdict } from "../../game/gameState.d";
import { MenuControllerContext, ContentMenu } from "./GameScreen";
import "./headerMenu.css";
import StyledText from "../../components/StyledText";
import Icon from "../../components/Icon";
import { Button } from "../../components/Button";
import { useGameState, usePlayerState } from "../../components/useHooks";
import { MobileContext } from "../Anchor";
import { roleSpecificMenuType } from "../Settings";


export default function HeaderMenu(props: Readonly<{
    chatMenuNotification: boolean
}>): ReactElement {
    const mobile = useContext(MobileContext)!;

    return <div className="header-menu">
        {!(GAME_MANAGER.getMySpectator() && !GAME_MANAGER.getMyHost()) && <FastForwardButton />}
        <Information />
        {!(GAME_MANAGER.getMySpectator() && !mobile) && <MenuButtons chatMenuNotification={props.chatMenuNotification}/>}
        <Timer />
    </div>
}

function Timer(): ReactElement {
    const timeLeftMs = useGameState(
        gameState => gameState.timeLeftMs,
        ["phaseTimeLeft", "tick"]
    )!
    const phaseLength = useGameState(
        gameState => gameState.phaseTimes[gameState.phaseState.type],
        ["phase"]
    )!

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
    const dayNumber = useGameState(
        gameState => gameState.dayNumber,
        ["phase"]
    )!
    const timeLeftMs = useGameState(
        gameState => gameState.timeLeftMs,
        ["phaseTimeLeft", "tick"]
    )!
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!

    const myIndex = usePlayerState(
        gameState => gameState.myIndex,
        ["yourPlayerIndex"]
    )
    const roleState = usePlayerState(
        clientState => clientState.roleState,
        ["yourRoleState"]
    )
    const myName = useMemo(() => {
        return myIndex === undefined ? undefined : players[myIndex].toString()
    }, [myIndex, players])

    return <div className="information">
        <div>
            <h3>
                <div>
                    {translate("phase."+phaseState.type)} {dayNumber}⏳{Math.floor(timeLeftMs/1000)}
                </div>
            </h3>
            {GAME_MANAGER.getMySpectator() 
                || <StyledText>
                    {myName + " (" + translate("role."+(roleState!.type)+".name") + ")"}
                </StyledText>
            }
        </div>
        <PhaseSpecificInformation players={players} myIndex={myIndex} phaseState={phaseState}/>
    </div>
}

export function PhaseSpecificInformation(props: Readonly<{
    phaseState: PhaseState,
    players: Player[],
    myIndex: number | undefined
}>): ReactElement | null {
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
                {!GAME_MANAGER.getMySpectator() && props.phaseState.type === "judgement" && <div className="judgement-info">
                    {(() => {
                        if (props.phaseState.playerOnTrial === props.myIndex) {
                            return translate("judgement.cannotVote.onTrial");
                        } else if (!props.players[props.myIndex!].alive) {
                            return translate("judgement.cannotVote.dead");
                        } else {
                            return (["guilty", "abstain", "innocent"] as const).map((verdict) => {
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

function MenuButtons(props: Readonly<{ chatMenuNotification: boolean }>): ReactElement {
    const roleState = usePlayerState(
        clientState => clientState.roleState,
        ["yourRoleState"]
    )

    const menuController = useContext(MenuControllerContext)!;

    return <div className="menu-buttons">
        <Button className="chat-menu-colors"
            highlighted={menuController.menusOpen().includes(ContentMenu.ChatMenu)}
            onClick={()=>menuController.closeOrOpenMenu(ContentMenu.ChatMenu)}
        >
            {props.chatMenuNotification && <div className="chat-notification highlighted">!</div>}
            {translate("menu.chat.icon")}
            <span className="mobile-hidden">{translate("menu.chat.title")}</span>
        </Button>
        <Button className="player-list-menu-colors"
            highlighted={menuController.menusOpen().includes(ContentMenu.PlayerListMenu)}
            onClick={()=>menuController.closeOrOpenMenu(ContentMenu.PlayerListMenu)}
        >
            {translate("menu.playerList.icon")}
            <span className="mobile-hidden">{translate("menu.playerList.title")}</span>
        </Button>
        {GAME_MANAGER.getMySpectator() || <Button className="will-menu-colors" 
            highlighted={menuController.menusOpen().includes(ContentMenu.WillMenu)}
            onClick={()=>menuController.closeOrOpenMenu(ContentMenu.WillMenu)}
        >
            {translate("menu.will.icon")}
            <span className="mobile-hidden">{translate("menu.will.title")}</span>
        </Button>}
        {!GAME_MANAGER.getMySpectator() && roleSpecificMenuType(roleState!.type) === "standalone"
            && <Button className="role-specific-colors" 
                highlighted={menuController.menusOpen().includes(ContentMenu.RoleSpecificMenu)}
                onClick={()=>menuController.closeOrOpenMenu(ContentMenu.RoleSpecificMenu)}
            >
                <StyledText noLinks={true}>
                    {translate("role."+roleState?.type+".name")}
                </StyledText>
            </Button>}
        <Button className="graveyard-menu-colors" 
            highlighted={menuController.menusOpen().includes(ContentMenu.GraveyardMenu)}
            onClick={()=>menuController.closeOrOpenMenu(ContentMenu.GraveyardMenu)}
        >
            {translate("menu.graveyard.icon")}
            <span className="mobile-hidden">{translate("menu.graveyard.title")}</span>
        </Button>
        {GAME_MANAGER.getMySpectator() || <Button className="wiki-menu-colors"
            highlighted={menuController.menusOpen().includes(ContentMenu.WikiMenu)} 
            onClick={()=>menuController.closeOrOpenMenu(ContentMenu.WikiMenu)}
        >
            {translate("menu.wiki.icon")}
            <span className="mobile-hidden">{translate("menu.wiki.title")}</span>
        </Button>}
    </div>
}

export function FastForwardButton(): ReactElement {
    const fastForward = useGameState(
        gameState => gameState.fastForward,
        ["yourVoteFastForwardPhase"]
    )!

    return <Button 
        onClick={()=>GAME_MANAGER.sendVoteFastForwardPhase(!fastForward)}
        className="fast-forward-button"
        highlighted={fastForward}
    >
        <Icon>double_arrow</Icon>
    </Button>
}
