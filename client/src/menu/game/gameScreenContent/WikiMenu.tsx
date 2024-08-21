import React, { ReactElement } from "react";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./wikiMenu.css"
import translate from "../../../game/lang";
import Wiki from "../../../components/Wiki";

export default function WikiMenu(): ReactElement {
    return <div className="wiki-menu wiki-menu-colors">
        <ContentTab close={ContentMenu.WikiMenu} helpMenu={null}>{translate("menu.wiki.title")}</ContentTab>
        
        <div className="wiki-menu-search">
            <Wiki enabledRoles={
                GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.enabledRoles : []
            }/>
        </div>
    </div>
}