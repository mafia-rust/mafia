import React, { ReactElement, useState } from "react";
import translate from "../../../../game/lang";
import { ContentMenu, ContentTab } from "../../GameScreen";
import RoleSpecificSection from "./RoleSpecific";
import { useGameState, usePlayerState } from "../../../../components/useHooks";
import Pitchfork from "../../../../components/Pitchfork";
import HitOrder from "../../../../components/HitOrder";
import OldSelectionType from "./AbilitySelectionTypes/OldSelectionType";

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
    const [pitchforkVoteOpen, setPitchforkVoteOpen] = useState<boolean>(false);
    const [hitOrderOpen, setHitOrderOpen] = useState<boolean>(false);

    return <div className="role-specific-colors">
        <ContentTab close={ContentMenu.RoleSpecificMenu} helpMenu={"standard/abilityMenu"}>
            {translate("menu.ability.title")}
        </ContentTab>
        {!mySpectator &&
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
                <OldSelectionType/>
            </details>
        }
        {
            !mySpectator && 
            <Pitchfork pitchforkVoteOpen={pitchforkVoteOpen} setPitchforkVoteOpen={setPitchforkVoteOpen}/>
        }
        {
            !mySpectator && 
            <HitOrder hitOrderOpen={hitOrderOpen} setHitOrderOpen={setHitOrderOpen}/>
        }
    </div>
    
}