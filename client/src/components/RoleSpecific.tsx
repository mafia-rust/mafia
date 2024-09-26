import React, { ReactElement, useMemo } from "react";
import { Role, RoleState } from "../game/roleState.d";
import LargeAuditorMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeAuditorMenu";
import LargeHypnotistMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeHypnotistMenu";
import LargeDoomsayerMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import LargeForgerMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeForgerMenu";
import LargeJournalistMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeJournalistMenu";
import StyledText from "./StyledText";
import translate from "../game/lang";
import GAME_MANAGER from "..";
import SmallOjoMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallOjoMenu";
import SmallPuppeteerMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallPuppeteerMenu";
import RoleDropdown from "./RoleDropdown";
import ROLES from "../resources/roles.json";
import "../menu/game/gameScreenContent/RoleSpecificMenus/smallRoleSpecificMenu.css";
import LargeKiraMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeKiraMenu";
import Counter from "./Counter";
import "./roleSpecific.css";
import ErosMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/ErosMenu";
import CounterfeiterMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/CounterfeiterMenu";
import RetrainerMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/RetrainerMenu";
import { useGameState, usePlayerState } from "./useHooks";
import RecruiterMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/RecruiterMenu";
import StewardMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/StewardMenu";

export default function RoleSpecificSection(){
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase"]
    )!;

    const dayNumber = useGameState(
        gameState => gameState.dayNumber,
        ["phase"]
    )!;

    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"]
    )!;
    
    switch(roleState.type){
        case "auditor":
            return <LargeAuditorMenu/>;
        case "journalist":
            return <LargeJournalistMenu/>;
        case "hypnotist":
            return <LargeHypnotistMenu/>;
        case "forger":
            return <LargeForgerMenu/>;
        case "doomsayer":
            return <LargeDoomsayerMenu/>;
        case "kira":
            return <LargeKiraMenu/>;
        case "retrainer":
            return <RetrainerMenu/>
        case "jailor": 
            return <JailorRoleSpecificMenu roleState={roleState}/>;
        case "medium": 
            return <MediumRoleSpecificMenu roleState={roleState}/>
        case "doctor": {
            return <Counter
                max={1}
                current={roleState.selfHealsRemaining}
            >
                <StyledText>{translate("role.doctor.roleDataText", roleState.selfHealsRemaining)}</StyledText>
            </Counter>
        }
        case "bodyguard":
            return <Counter
                max={1}
                current={roleState.selfShieldsRemaining}
            >
                <StyledText>{translate("role.bodyguard.roleDataText", roleState.selfShieldsRemaining)}</StyledText>
            </Counter>
        case "engineer":
            return <div className="role-information">
                <StyledText>{translate("role.engineer.roleDataText." + roleState.trap.type)}</StyledText>
            </div>;
        case "vigilante":
            switch(roleState.state.type){
                case "willSuicide":
                    return <div className="role-information">
                        <StyledText>{translate("role.vigilante.roleDataText.suicide")}</StyledText>
                    </div>
                case "notLoaded":
                    return <div className="role-information">
                        <StyledText>{translate("role.vigilante.roleDataText.notLoaded")}</StyledText>
                    </div>
                case "loaded":
                    return <Counter 
                        max={3} 
                        current={roleState.state.bullets}
                    >
                        <StyledText>{translate("role.vigilante.roleDataText", roleState.state.bullets)}</StyledText>
                    </Counter>
                default:
                    return null
            }
        case "veteran":
            return <Counter
                max={3}
                current={roleState.alertsRemaining}
            >
                <StyledText>{translate("role.veteran.roleDataText", roleState.alertsRemaining)}</StyledText>
            </Counter>
        case "armorsmith":
            return <Counter
                max={2}
                current={roleState.openShopsRemaining}
            >
                <StyledText>{translate("role.armorsmith.roleDataText", roleState.openShopsRemaining)}</StyledText>
            </Counter>
        case "marksman": 
            return <MarksmanRoleSpecificMenu roleState={roleState} />
        case "eros":
            return <ErosMenu/>;
        case "counterfeiter":
            return <CounterfeiterMenu/>;
        case "mortician":
            return <Counter
                max={3}
                current={3-roleState.obscuredPlayers.length}
            >
                <StyledText>{translate("role.mortician.roleDataText", (3-roleState.obscuredPlayers.length))}</StyledText>
            </Counter>
        case "death":
            return <Counter
                max={6}
                current={roleState.souls}
            >
                <StyledText>{translate("role.death.roleDataText", roleState.souls)}</StyledText>
            </Counter>
        case "ojo":
            return <SmallOjoMenu action={roleState.chosenAction}/>
        case "steward":
            return <StewardMenu
                roleState={roleState}
            />;
        case "puppeteer":
            return <SmallPuppeteerMenu 
                action={roleState.action} 
                marionettesRemaining={roleState.marionettesRemaining}
                phase={phaseState.type}
            />;
        case "recruiter":
            return <RecruiterMenu 
                action={roleState.action} 
                remaining={roleState.recruitsRemaining}
                dayNumber={dayNumber}
                phase={phaseState.type}
            />;
        case "wildcard":
        case "trueWildcard":
        case "mafiaSupportWildcard":
        case "mafiaKillingWildcard":
        case "fiendsWildcard": {
            return <WildcardRoleSpecificMenu roleState={roleState} />
        }
        case "martyr":
            if (roleState.state.type === "stillPlaying") {
                return <>
                    <div className="role-information">
                        <StyledText>{translate("role.martyr.roleDataText.eccentric")}</StyledText>
                    </div>
                    <Counter
                        max={2}
                        current={roleState.state.bullets}
                    >
                        <StyledText>{translate("role.martyr.roleDataText", roleState.state.bullets)}</StyledText>
                    </Counter>
                </>
            } else {
                return null;
            }
        default:
            return null;
    }
}

function MarksmanRoleSpecificMenu(props: Readonly<{
    roleState: RoleState & { type: "marksman" }
}>): ReactElement {
    let stateText;
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!;

    switch(props.roleState.state.type){
        case "notLoaded":
        case "shotTownie":
            stateText = translate("role.marksman.roleDataText."+props.roleState.state.type)
            break;
        case "marks":
            if(props.roleState.state.marks.length === 0){
                stateText = translate("role.marksman.roleDataText.marks.none")
            }else{
                stateText = translate("role.marksman.roleDataText.marks",
                    props.roleState.state.marks.map(index => players[index].toString()).join(", ")
                )
            }
    }
    
    return <div className="role-information">
        <StyledText>{stateText}</StyledText>
    </div>
}

function JailorRoleSpecificMenu(props: Readonly<{
    roleState: RoleState & { type: "jailor" }
}>): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!;
    const phaseState = useGameState(
        gameState => gameState.phaseState
    )!;

    const counter = <Counter 
        max={3} 
        current={props.roleState.executionsRemaining}
    >
        <StyledText>{translate("role.jailor.roleDataText.executionsRemaining", props.roleState.executionsRemaining)}</StyledText>
    </Counter>;
    if(phaseState.type==="night") {
        return counter;
    } else if (props.roleState.jailedTargetRef === null) {
        return <>
            {counter}
            <div className="role-information">
                <StyledText>{translate("role.jailor.roleDataText.nobody")}</StyledText>
            </div>
        </>
    } else {
        return <>
            {counter}
            <div className="role-information">
                <StyledText>{translate("role.jailor.roleDataText", 
                    players[props.roleState.jailedTargetRef].toString(), 
                )}</StyledText>
            </div>
        </>
    }
}

function MediumRoleSpecificMenu(props: Readonly<{
    roleState: RoleState & { type: "medium" }
}>): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!;

    const counter = <Counter
        max={2}
        current={props.roleState.seancesRemaining}
    >
        <StyledText>{translate("role.medium.roleDataText.hauntsRemaining", props.roleState.seancesRemaining)}</StyledText>
    </Counter>
    if (props.roleState.seancedTarget === null) {
        return <>
            {counter}
            <div className="role-information">
                <StyledText>{translate("role.medium.roleDataText.nobody")}</StyledText>
            </div>
        </>
    } else {
        return <>
            {counter}
            <div className="role-information">
                <StyledText>{translate("role.medium.roleDataText", 
                    players[props.roleState.seancedTarget].toString(),
                )}</StyledText>
            </div>
        </>;
    }
}

function WildcardRoleSpecificMenu(props: Readonly<{
    roleState: RoleState & { type: "wildcard" | "trueWildcard" | "mafiaSupportWildcard" | "mafiaKillingWildcard" | "fiendsWildcard" }
}>): ReactElement {
    const enabledRoles = useGameState(
        gameState => gameState.enabledRoles,
        ["enabledRoles"]
    )!;

    const choosable = useMemo(() => {
        switch (props.roleState.type) {
            case "wildcard":
            case "trueWildcard":
                return enabledRoles
            case "mafiaSupportWildcard":
                return Object.keys(ROLES).filter((rle)=>
                    rle === "mafiaSupportWildcard" ||
                    (
                        ROLES[rle as keyof typeof ROLES].roleSet === "mafiaSupport" &&
                        enabledRoles.includes(rle as Role)
                    )
                ).map((r)=>r as Role)
            case "mafiaKillingWildcard":
                return Object.keys(ROLES).filter((rle)=>
                    rle === "mafiaKillingWildcard" ||
                    (
                        ROLES[rle as keyof typeof ROLES].roleSet === "mafiaKilling" &&
                        enabledRoles.includes(rle as Role)
                    )
                ).map((r)=>r as Role)
            case "fiendsWildcard":
                return Object.keys(ROLES).filter((rle)=>
                    ROLES[rle as keyof typeof ROLES].faction === "fiends" &&
                    enabledRoles.includes(rle as Role)
                ).map((r)=>r as Role)
        }
    }, [enabledRoles, props.roleState.type])

    return <div className="role-information">
        <StyledText>{translate(`role.${props.roleState.type}.smallRoleMenu`)}</StyledText>
        <div>
            <RoleDropdown 
                value={props.roleState.role} 
                enabledRoles={choosable}
                onChange={(rle)=>{
                    GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                }}
            />
        </div>
    </div>;
}