import React from "react";
import GameScreen, { ContentMenus } from "../GameScreen";
import WikiSearch from "../../WikiSearch";
import "./wikiMenu.css"
import translate from "../../../game/lang";

export default function WikiMenu() {
    return <div className="wiki-menu">
        {/* TODO: Use content-tab on other menus */}
        <div className="content-tab">
            <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WikiMenu)}}>{translate("menu.wiki.title")}</button>
        </div>
        <WikiSearch/>
    </div>
}
