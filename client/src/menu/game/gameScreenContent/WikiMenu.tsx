import React, { ReactElement } from "react";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./wikiMenu.css"
import translate from "../../../game/lang";
import { getRolesFromRoleList, RoleList } from "../../../game/roleListState.d";
import Wiki from "../../../components/Wiki";
import ROLES from "../../../resources/roles.json";
import { Role } from "../../../game/roleState.d";

export default function WikiMenu(): ReactElement {
    return <div className="wiki-menu wiki-menu-colors">
        <ContentTab close={ContentMenu.WikiMenu} helpMenu={null}>{translate("menu.wiki.title")}</ContentTab>
        
        <div className="wiki-menu-search">
            <Wiki enabledRoles={
                GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ?
                getWikiHighlightedRoles(GAME_MANAGER.state.roleList, GAME_MANAGER.state.enabledRoles) : []
            }/>
        </div>
    </div>
}

function getWikiHighlightedRoles(roleList: RoleList, enabledRoles: Role[]): Role[] {
    let out = [];

    let roles = getRolesFromRoleList(roleList);
    roles = roles.filter((role) => {
        return enabledRoles.includes(role);
    });

    for(let role of roles){
        out.push(role);
        for(let converted of ROLES[role].canBeConvertedTo){
            out.push(converted);
        }
    }

    return out as Role[];
}