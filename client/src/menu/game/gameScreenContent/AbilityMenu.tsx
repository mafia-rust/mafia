import React, { ReactElement, useState } from "react";
import translate from "../../../game/lang";
import { ContentMenu, ContentTab } from "../GameScreen";
import RoleSpecificSection from "../../../components/RoleSpecific";
import SelectionInformation from "../../../components/SelectionInformation";
import { useGameState, usePlayerState } from "../../../components/useHooks";
import Pitchfork from "../../../components/Pitchfork";
import HitOrder from "../../../components/HitOrder";
import OnePlayerOptionSelectionType from "./AbilitySelectionTypes/OnePlayerOptionSelectionType";
import TwoPlayerOptionSelectionType from "./AbilitySelectionTypes/TwoPlayerOptionSelectionType";

export default function AbilityMenu(): ReactElement {
    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"]
    )!;
    const role = roleState.type;
    const mySpectator = useGameState(
        gameState => gameState.clientState.type === "spectator",
        ["gamePlayers", "acceptJoin"]
    )!;

    const [roleSpecificOpen, setRoleSpecificOpen] = useState<boolean>(true);
    const [pitchforkVoteOpen, setPitchforkVoteOpen] = useState<boolean>(false);
    const [hitOrderOpen, setHitOrderOpen] = useState<boolean>(false);

    return <div className="role-specific-colors">
        <ContentTab close={ContentMenu.RoleSpecificMenu} helpMenu={null}>
            {translate("role."+role+".name")}
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
                <SelectionInformation />
                <RoleSpecificSection/>
                <OnePlayerOptionSelectionType/>
                <TwoPlayerOptionSelectionType/>
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