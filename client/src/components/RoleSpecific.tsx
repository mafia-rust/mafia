import React, { useEffect } from "react";
import { Role, RoleState } from "../game/roleState.d";
import LargeAuditorMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeAuditorMenu";
import LargeConsortMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeConsortMenu";
import LargeDoomsayerMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import LargeForgerMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeForgerMenu";
import LargeJournalistMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeJournalistMenu";
import { PhaseState } from "../game/gameState.d";
import StyledText from "./StyledText";
import translate from "../game/lang";
import GAME_MANAGER from "..";
import { StateListener } from "../game/gameManager.d";
import SmallOjoMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallOjoMenu";
import SmallPuppeteerMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallPuppeteerMenu";
import RoleDropdown from "./RoleDropdown";
import { getRolesComplement } from "../game/roleListState.d";
import ROLES from "../resources/roles.json";
import "../menu/game/gameScreenContent/RoleSpecificMenus/smallRoleSpecificMenu.css";
import LargeKiraMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeKiraMenu";

export default function RoleSpecificSection(){
    
    const [phaseState, setPhaseState] = React.useState<PhaseState | null>(null);
    const [roleState, setRoleState] = React.useState<RoleState | null>(null);

    useEffect(()=>{
        const listener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game") {
                switch (type) {
                    case "phase":
                        setPhaseState(GAME_MANAGER.state.phaseState);
                        break;
                    case "yourRoleState":
                        if(GAME_MANAGER.state.clientState.type === "player")
                            setRoleState(GAME_MANAGER.state.clientState.roleState);
                        break;
                }
            }
        };
        listener("phase");
        listener("yourRoleState");
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    },[]);

    if (!roleState) return null;
    if (!phaseState) return null;
    if (GAME_MANAGER.state.stateType !== "game") return null;
    if (GAME_MANAGER.state.clientState.type !== "player") return null;

    

    switch(roleState.type){
        case "auditor":
            return <LargeAuditorMenu/>;
        case "journalist":
            return <LargeJournalistMenu/>;
        case "hypnotist":
            return <LargeConsortMenu/>;
        case "forger":
            return <LargeForgerMenu/>;
        case "doomsayer":
            return <LargeDoomsayerMenu/>;
        case "kira":
            return <LargeKiraMenu/>;
            



        case "jailor":
            if(phaseState.type==="night") {
                return <StyledText>{translate("role.jailor.roleDataText.night", roleState.executionsRemaining)}</StyledText>;
            } else if (roleState.jailedTargetRef === null) {
                return <StyledText>{translate("role.jailor.roleDataText.nobody", roleState.executionsRemaining)}</StyledText>;
            } else {
                return <StyledText>{translate("role.jailor.roleDataText", 
                    GAME_MANAGER.state.players[roleState.jailedTargetRef].toString(), 
                    roleState.executionsRemaining
                )}</StyledText>;;
            }
        case "medium":
            if (roleState.seancedTarget === null) {
                return <StyledText>{translate("role.medium.roleDataText.nobody", roleState.seancesRemaining)}</StyledText>;
            } else {
                return <StyledText>{translate("role.medium.roleDataText", 
                    GAME_MANAGER.state.players[roleState.seancedTarget].toString(),
                    roleState.seancesRemaining
                )}</StyledText>;
            }
        case "doctor":
            return <StyledText>{translate("role.doctor.roleDataText", roleState.selfHealsRemaining)}</StyledText>;
        case "bodyguard":
            return <StyledText>{translate("role.bodyguard.roleDataText", roleState.selfShieldsRemaining)}</StyledText>;
        case "engineer":
            return <>
                <div>
                    <StyledText>{translate("role.engineer.roleDataText." + roleState.trap.type)}</StyledText>
                </div>
                {
                    roleState.trap.type === "set" &&
                    phaseState.type === "night" &&
                    <button className={roleState.trap.shouldUnset?"highlighted":""} onClick={()=>{
                        if(
                            GAME_MANAGER.state.stateType === "game" &&
                            GAME_MANAGER.state.clientState.type === "player" && 
                            roleState?.type === "engineer" && 
                            roleState.trap.type === "set"
                        )
                            GAME_MANAGER.sendSetEngineerShouldUnset(!roleState.trap.shouldUnset);
                    }}>{translate("role.engineer.roleDataText.unset")}</button>
                }
            </>;
        case "vigilante":
            switch(roleState.state.type){
                case "willSuicide":
                    return <StyledText>{translate("role.vigilante.roleDataText.suicide")}</StyledText>;
                case "notLoaded":
                    return <StyledText>{translate("role.vigilante.roleDataText.notLoaded")}</StyledText>;
                case "loaded":
                    return <StyledText>{translate("role.vigilante.roleDataText", roleState.state.bullets)}</StyledText>;
                default:
                    return null
            }
        case "veteran":
            return <StyledText>{translate("role.veteran.roleDataText", roleState.alertsRemaining)}</StyledText>;
        case "marksman":
            switch(roleState.state.type){
                case "notLoaded":
                case "shotTownie":
                    return <StyledText>{translate("role.marksman.roleDataText."+roleState.state.type)}</StyledText>
                case "marks":
                    switch(roleState.state.marks.type){
                        case "none":
                            return <StyledText>{translate("role.marksman.roleDataText.marks.none")}</StyledText>
                        case "one":
                            return <StyledText>{translate("role.marksman.roleDataText.marks.one", 
                                GAME_MANAGER.state.players[roleState.state.marks.a].toString()
                            )}</StyledText>
                        case "two":
                            return <StyledText>{translate("role.marksman.roleDataText.marks.two", 
                                GAME_MANAGER.state.players[roleState.state.marks.a].toString(), 
                                GAME_MANAGER.state.players[roleState.state.marks.b].toString()
                            )}</StyledText>
                    }
            }
            return null;
        case "mortician":
            return <StyledText>{translate("role.mortician.roleDataText", (3-roleState.obscuredPlayers.length))}</StyledText>;
        case "death":
            return <StyledText>{translate("role.death.roleDataText", roleState.souls)}</StyledText>;
        case "ojo":
            if(phaseState.type === "night" && GAME_MANAGER.state.clientState.myIndex!==null && GAME_MANAGER.state.players[GAME_MANAGER.state.clientState.myIndex].alive)
                return <SmallOjoMenu action={roleState.chosenAction}/>;
            return null;
        case "puppeteer":
            return <SmallPuppeteerMenu 
                action={roleState.action} 
                marionettesRemaining={roleState.marionettesRemaining}
                phase={phaseState.type}
            />;
        case "wildcard":
        case "trueWildcard":
            return <><StyledText>{translate("role.wildcard.smallRoleMenu")}</StyledText><div><RoleDropdown 
                value={roleState.role ?? "wildcard"}
                disabledRoles={GAME_MANAGER.state.excludedRoles} 
                onChange={(rle)=>{
                    GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                }}
            /></div></>;
        case "mafiaWildcard":
            const all_choosable_mafia: Role[] = Object.keys(ROLES).filter((rle)=>
                ROLES[rle as keyof typeof ROLES].faction === "mafia" &&
                rle !== "godfather" &&
                rle !== "mafioso" &&
                GAME_MANAGER.state.stateType === "game" &&
                !GAME_MANAGER.state.excludedRoles.includes(rle as Role)
            ).map((r)=>r as Role);

            return <><StyledText>{translate("role.mafiaWildcard.smallRoleMenu")}</StyledText><div><RoleDropdown 
                value={roleState.role ?? "mafiaWildcard"} 
                disabledRoles={getRolesComplement(all_choosable_mafia)}
                onChange={(rle)=>{
                    GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                }}
            /></div></>;
        case "fiendsWildcard":
            const all_choosable_fiends: Role[] = Object.keys(ROLES).filter((rle)=>
                ROLES[rle as keyof typeof ROLES].faction === "fiends" &&
                GAME_MANAGER.state.stateType === "game" &&
                !GAME_MANAGER.state.excludedRoles.includes(rle as Role)
            ).map((r)=>r as Role);

            return <><StyledText>{translate("role.fiendsWildcard.smallRoleMenu")}</StyledText><div><RoleDropdown 
                value={roleState.role ?? "fiendsWildcard"} 
                disabledRoles={getRolesComplement(all_choosable_fiends)}
                onChange={(rle)=>{
                    GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                }}
            /></div></>;
        case "martyr":
            if (roleState.state.type === "stillPlaying") {
                return <StyledText>{translate("role.martyr.roleDataText", roleState.state.bullets)}</StyledText>;
            } else {
                return null;
            }
        default:
            return null;
    }
}