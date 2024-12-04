import { useGameState, usePlayerState } from "../../../../components/useHooks";
import React, { ReactElement, useMemo } from "react";
import AuditorMenu from "./RoleSpecificMenus/AuditorMenu";
import LargeReporterMenu from "./RoleSpecificMenus/LargeReporterMenu";
import LargeHypnotistMenu from "./RoleSpecificMenus/LargeHypnotistMenu";
import LargeDoomsayerMenu from "./RoleSpecificMenus/LargeDoomsayerMenu";
import Counter from "../../../../components/Counter";
import StyledText from "../../../../components/StyledText";
import translate from "../../../../game/lang";
import CounterfeiterMenu from "./RoleSpecificMenus/CounterfeiterMenu";
import SmallPuppeteerMenu from "./RoleSpecificMenus/SmallPuppeteerMenu";
import StewardMenu from "./RoleSpecificMenus/StewardMenu";
import OjoMenu from "./RoleSpecificMenus/OjoMenu";
import RecruiterMenu from "./RoleSpecificMenus/RecruiterMenu";
import { Role, roleJsonData, RoleState } from "../../../../game/roleState.d";
import RoleDropdown from "../../../../components/RoleDropdown";
import GAME_MANAGER from "../../../..";


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
            return <AuditorMenu roleState={roleState}/>;
        case "reporter":
            return <LargeReporterMenu/>;
        case "hypnotist":
            return <LargeHypnotistMenu/>;
        case "doomsayer":
            return <LargeDoomsayerMenu/>;
        case "jailor": 
            return <JailorRoleSpecificMenu roleState={roleState}/>;
        case "kidnapper": 
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
                max={3}
                current={roleState.openShopsRemaining}
            >
                <StyledText>{translate("role.armorsmith.roleDataText", roleState.openShopsRemaining)}</StyledText>
            </Counter>
        case "marksman": 
            return <MarksmanRoleSpecificMenu roleState={roleState} />;
        case "counterfeiter":
            return <CounterfeiterMenu/>;
        case "mortician":
            return <Counter
                max={3}
                current={roleState.cremationsRemaining}
            >
                <StyledText>{translate("role.mortician.roleDataText", roleState.cremationsRemaining)}</StyledText>
            </Counter>
        case "death":
            return <Counter
                max={6}
                current={roleState.souls}
            >
                <StyledText>{translate("role.death.roleDataText", roleState.souls)}</StyledText>
            </Counter>
        case "ojo":
            return <OjoMenu roleState={roleState}/>
        case "steward":
            return <StewardMenu roleState={roleState}/>;
        case "spiral": 
            return <SpiralMenu />;
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
    roleState: (RoleState & { type: "marksman" })
}>): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!;
    
    let stateText;
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
    roleState: RoleState & { type: "jailor" | "kidnapper" } 
}>): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!;
    const phaseState = useGameState(
        gameState => gameState.phaseState
    )!;

    const counter = <Counter 
        max={props.roleState.type === "jailor" ? 3 : 1} 
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

    const ROLES = roleJsonData();

    const choosable = useMemo(() => {
        switch (props.roleState.type) {
            case "wildcard":
            case "trueWildcard":
                return enabledRoles
            case "mafiaSupportWildcard":
                return Object.keys(ROLES).filter((rle)=>
                    rle === "mafiaSupportWildcard" ||
                    (
                        ROLES[rle as keyof typeof ROLES].roleSets.includes("mafiaSupport") &&
                        enabledRoles.includes(rle as Role)
                    )
                ).map((r)=>r as Role)
            case "mafiaKillingWildcard":
                return Object.keys(ROLES).filter((rle)=>
                    rle === "mafiaKillingWildcard" ||
                    (
                        ROLES[rle as keyof typeof ROLES].roleSets.includes("mafiaKilling") &&
                        enabledRoles.includes(rle as Role)
                    )
                ).map((r)=>r as Role)
            case "fiendsWildcard":
                return Object.keys(ROLES).filter((rle)=>
                    ROLES[rle as keyof typeof ROLES].roleSets.includes("fiends") &&
                    enabledRoles.includes(rle as Role)
                ).map((r)=>r as Role)
        }
    }, [enabledRoles, props.roleState.type, ROLES])

    return <div className="role-information">
        <StyledText>{translate(`role.${props.roleState.type}.smallRoleMenu`)}</StyledText>
        <RoleDropdown 
            value={props.roleState.role} 
            enabledRoles={choosable}
            onChange={(rle)=>{
                GAME_MANAGER.sendSetWildcardRoleOutline(rle);
            }}
        />
    </div>;
}

function SpiralMenu(props: {}): ReactElement | null {
    const spiralingPlayers = useGameState(
        gameState => gameState.players.filter(p => p.playerTags.includes("spiraling")),
        ["yourPlayerTags"]
    )!

    if (spiralingPlayers.length !== 0) {
        return <div className="role-information">
            <StyledText>{translate("role.spiral.roleDataText.cannotSelect")}</StyledText>
        </div>
    } else {
        return null;
    }
}