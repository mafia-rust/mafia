import React, { useEffect } from "react";
import { Role, RoleState } from "../game/roleState.d";
import LargeAuditorMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeAuditorMenu";
import LargeHypnotistMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeHypnotistMenu";
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
import ROLES from "../resources/roles.json";
import "../menu/game/gameScreenContent/RoleSpecificMenus/smallRoleSpecificMenu.css";
import LargeKiraMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeKiraMenu";
import Counter from "./Counter";
import "./roleSpecific.css";
import ErosMenu from "../menu/game/gameScreenContent/RoleSpecificMenus/ErosMenu";

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
            return <LargeHypnotistMenu/>;
        case "forger":
            return <LargeForgerMenu/>;
        case "doomsayer":
            return <LargeDoomsayerMenu/>;
        case "kira":
            return <LargeKiraMenu/>;
            



        case "jailor": {
            const counter = <Counter 
                max={3} 
                current={roleState.executionsRemaining}
            >
                <StyledText>{translate("role.jailor.roleDataText.executionsRemaining", roleState.executionsRemaining)}</StyledText>
            </Counter>;
            if(phaseState.type==="night") {
                return counter;
            } else if (roleState.jailedTargetRef === null) {
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
                            GAME_MANAGER.state.players[roleState.jailedTargetRef].toString(), 
                        )}</StyledText>
                    </div>
                </>
            }
        }
        case "medium": {
            const counter = <Counter
                max={2}
                current={roleState.seancesRemaining}
            >
                <StyledText>{translate("role.medium.roleDataText.hauntsRemaining", roleState.seancesRemaining)}</StyledText>
            </Counter>
            if (roleState.seancedTarget === null) {
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
                            GAME_MANAGER.state.players[roleState.seancedTarget].toString(),
                        )}</StyledText>
                    </div>
                </>;
            }
        }
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
        case "marksman": {
            let stateText;

            switch(roleState.state.type){
                case "notLoaded":
                case "shotTownie":
                    stateText = translate("role.marksman.roleDataText."+roleState.state.type)
                    break;
                case "marks":
                    if(roleState.state.marks.length === 0){
                        stateText = translate("role.marksman.roleDataText.marks.none")
                    }else{
                        stateText = translate("role.marksman.roleDataText.marks",
                            roleState.state.marks.map((index)=>{
                                if(GAME_MANAGER.state.stateType === "game")
                                    return GAME_MANAGER.state.players[index].toString();
                                else
                                    return "";
                            }).join(", ")
                            
                        )
                    }
            }
            
            return <div className="role-information">
                <StyledText>{stateText}</StyledText>
            </div>
        }
        case "eros":
            return <ErosMenu/>;
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
            return <div className="role-information">
                <StyledText>{translate("role.wildcard.smallRoleMenu")}</StyledText>
                <div>
                    <RoleDropdown 
                        value={roleState.role ?? "wildcard"}
                        enabledRoles={GAME_MANAGER.state.enabledRoles} 
                        onChange={(rle)=>{
                            GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                        }}
                    />
                </div>
            </div>;
        case "mafiaSupportWildcard": {
            const all_choosable_mafia: Role[] = Object.keys(ROLES).filter((rle)=>
                rle === "mafiaSupportWildcard" ||
                (
                    ROLES[rle as keyof typeof ROLES].roleSet === "mafiaSupport" &&
                    GAME_MANAGER.state.stateType === "game" &&
                    GAME_MANAGER.state.enabledRoles.includes(rle as Role)
                )
            ).map((r)=>r as Role);

            return <div className="role-information">
                <StyledText>{translate("role.mafiaSupportWildcard.smallRoleMenu")}</StyledText>
                <div>
                    <RoleDropdown 
                        value={roleState.role ?? "mafiaSupportWildcard"} 
                        enabledRoles={all_choosable_mafia}
                        onChange={(rle)=>{
                            GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                        }}
                    />
                </div>
            </div>;
        }
        case "fiendsWildcard": {
            const all_choosable_fiends: Role[] = Object.keys(ROLES).filter((rle)=>
                ROLES[rle as keyof typeof ROLES].faction === "fiends" &&
                GAME_MANAGER.state.stateType === "game" &&
                GAME_MANAGER.state.enabledRoles.includes(rle as Role)
            ).map((r)=>r as Role);

            return <div className="role-information">
                <StyledText>{translate("role.fiendsWildcard.smallRoleMenu")}</StyledText>
                <div>
                    <RoleDropdown 
                        value={roleState.role ?? "fiendsWildcard"} 
                        enabledRoles={all_choosable_fiends}
                        onChange={(rle)=>{
                            GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                        }}
                    />
                </div>
            </div>;
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
