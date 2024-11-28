import React, { ReactElement } from "react";
import translate from "../../../../game/lang";
import { ContentMenu, ContentTab } from "../../GameScreen";
import { useGameState } from "../../../../components/useHooks";
import OldSelectionType from "./AbilitySelectionTypes/OldSelectionMenu";
import GenericAbilityMenu from "./GenericAbilityMenu";

export default function AbilityMenu(): ReactElement {
    const mySpectator = useGameState(
        gameState => gameState.clientState.type === "spectator",
        ["gamePlayers", "acceptJoin"]
    )!;

    return <div className="role-specific-colors">
        <ContentTab close={ContentMenu.RoleSpecificMenu} helpMenu={"standard/abilityMenu"}>
            {translate("menu.ability.title")}
        </ContentTab>
        {!mySpectator &&
            <>
                <OldSelectionType/>
                <GenericAbilityMenu/>
            </>
        }
    </div>
    
}