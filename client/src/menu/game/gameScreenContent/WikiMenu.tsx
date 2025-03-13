import React, { ReactElement } from "react";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./wikiMenu.css"
import translate from "../../../game/lang";
import Wiki from "../../../components/Wiki";
import { useLobbyOrGameState } from "../../../components/useHooks";
import { getAllRoles } from "../../../game/roleListState.d";
import { MODIFIERS, ModifierType } from "../../../game/gameState.d";

export default function WikiMenu(): ReactElement {
    const enabledRoles = useLobbyOrGameState(
        state => state.enabledRoles,
        ["enabledRoles"],
        getAllRoles()
    )!;
    const enabledModifiers = useLobbyOrGameState(
        state => state.enabledModifiers,
        ["enabledModifiers"],
        MODIFIERS as any as ModifierType[]
    )!;

    return <div className="wiki-menu wiki-menu-colors">
        <ContentTab close={ContentMenu.WikiMenu} helpMenu={null}>{translate("menu.wiki.title")}</ContentTab>
        
        <div className="wiki-menu-search">
            <Wiki enabledRoles={enabledRoles} enabledModifiers={enabledModifiers}/>
        </div>
    </div>
}