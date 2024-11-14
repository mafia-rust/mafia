import React, { ReactElement, useState } from "react";
import translate from "../../../../game/lang";
import { ContentMenu, ContentTab } from "../../GameScreen";
import RoleSpecificSection from "./RoleSpecific";
import { useGameState, usePlayerState } from "../../../../components/useHooks";
import OldSelectionType from "./AbilitySelectionTypes/OldSelectionMenu";
import ForfeitVote from "./ForfeitVote";
import Pitchfork from "./Pitchfork";
import HitOrder from "./HitOrder";
import GenericAbilityMenu from "./GenericAbilityMenu";

export default function AbilityMenu(): ReactElement {
    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"]
    )!;
    const mySpectator = useGameState(
        gameState => gameState.clientState.type === "spectator",
        ["gamePlayers", "acceptJoin"]
    )!;

    const [roleSpecificOpen, setRoleSpecificOpen] = useState<boolean>(true);

    return <div className="role-specific-colors">
        <ContentTab close={ContentMenu.RoleSpecificMenu} helpMenu={"standard/abilityMenu"}>
            {translate("menu.ability.title")}
        </ContentTab>
        {!mySpectator &&
            <>
                <ForfeitVote/>
                <details className="role-specific-colors small-role-specific-menu" open={roleSpecificOpen}>
                    <summary
                        onClick={(e)=>{
                            e.preventDefault();
                            setRoleSpecificOpen(!roleSpecificOpen);
                        }}
                    >
                        {translate("role."+roleState?.type+".name")}
                    </summary>
                    <RoleSpecificSection/>
                    <GenericAbilityMenu/>
                    <OldSelectionType/>
                </details>
                <Pitchfork/>
                <HitOrder/>
            </>
        }
    </div>
    
}