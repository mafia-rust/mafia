import React, { ReactElement } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import StyledText from "../../../components/StyledText";
import { EnabledRolesDisplay } from "../../../components/gameModeSettings/EnabledRoleSelector";
import { useGameState, usePlayerState } from "../../../components/useHooks";
import { translateRoleOutline } from "../../../game/roleListState.d";
import { EnabledModifiersDisplay } from "../../../components/gameModeSettings/EnabledModifiersDisplay";
import { Button } from "../../../components/Button";
import DetailsSummary from "../../../components/DetailsSummary";

export default function GraveyardMenu(): ReactElement {
    return <div className="graveyard-menu graveyard-menu-colors">
        <ContentTab close={ContentMenu.GraveyardMenu} helpMenu={"standard/gameMode"}>{translate("menu.gameMode.title")}</ContentTab>
            
        <div className="graveyard-menu-role-list">
            <RoleListDisplay />
        </div>
        <EnabledRoles/>
        <EnabledModifiers/>
    </div>
}

function RoleListDisplay(): ReactElement {
    const roleList = useGameState(
        gameState => gameState.roleList,
        ["roleList"]
    )!
    const crossedOutOutlines = usePlayerState(
        clientState => clientState.crossedOutOutlines,
        ["yourCrossedOutOutlines"]
    )

    return <>
        { roleList.map((entry, index)=>{
            const roleOutlineName = translateRoleOutline(entry);

            return <Button 
                className="role-list-button"
                style={{ gridRow: index + 1 }} 
                key={roleOutlineName + crossedOutOutlines?.includes(index) + index}
                onClick={()=>{
                    if (GAME_MANAGER.getMySpectator()) return;

                    let newCrossedOutOutlines = crossedOutOutlines!;
                    if(newCrossedOutOutlines.includes(index))
                        newCrossedOutOutlines = newCrossedOutOutlines.filter(x=>x!==index);
                    else
                        newCrossedOutOutlines.push(index);

                    GAME_MANAGER.sendSaveCrossedOutOutlinesPacket(newCrossedOutOutlines);
                }}
                onMouseDown={(e)=>{
                    // on right click, show a list of all roles that can be in this bucket
                    // if(e.button === 2){
                    //     e.preventDefault();
                    // }
                }}
            >
                {
                    crossedOutOutlines?.includes(index) ? 
                    <s><StyledText>
                        {translateRoleOutline(entry)}
                    </StyledText></s> : 
                    <StyledText>
                        {translateRoleOutline(entry)}
                    </StyledText>
                }
            </Button>
        })}
    </>
}

function EnabledRoles(): ReactElement {
    const enabledRoles = useGameState(
        gameState => gameState.enabledRoles,
        ["enabledRoles"]
    )!

    return <div className="graveyard-menu-excludedRoles">
        <DetailsSummary
            summary={translate("menu.enabledRoles.enabledRoles")}
        >
            <EnabledRolesDisplay enabledRoles={enabledRoles}/>
        </DetailsSummary>
    </div>
}

function EnabledModifiers(): ReactElement {
    const enabledModifiers = useGameState(
        gameState=>gameState.enabledModifiers,
        ["enabledModifiers"]
    )!

    return <div className="graveyard-menu-excludedRoles">
        <DetailsSummary
            summary={translate("modifiers")}
        >
            <EnabledModifiersDisplay disabled={true} enabledModifiers={enabledModifiers}/>
        </DetailsSummary>
    </div>
}