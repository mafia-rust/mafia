import React, { ReactElement } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import StyledText from "../../../components/StyledText";
import { EnabledRolesDisplay } from "../../../components/gameModeSettings/EnabledRoleSelector";
import { useGameState, usePlayerState, useSpectator } from "../../../components/useHooks";
import { translateRoleOutline } from "../../../game/roleListState.d";
import { Button } from "../../../components/Button";
import DetailsSummary from "../../../components/DetailsSummary";
import { EnabledModifiersDisplay } from "../../../components/gameModeSettings/EnabledModifiersSelector";

export default function GraveyardMenu(): ReactElement {
    return <div className="graveyard-menu graveyard-menu-colors">
        <ContentTab close={ContentMenu.GraveyardMenu} helpMenu={"standard/gameMode"}>{translate("menu.gameMode.title")}</ContentTab>
            
        <DetailsSummary
            summary={translate("menu.lobby.roleList")}
            defaultOpen={true}
        >
            <RoleListDisplay />
        </DetailsSummary>
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
        ["yourCrossedOutOutlines"],
        []
    )!

    const spectator = useSpectator();

    return <>
        {roleList.map((entry, index)=>{
            return <Button
                className="role-list-button placard"
                style={{ gridRow: index + 1 }}
                key={index}
                onClick={()=>{
                    if (spectator) return;

                    let newCrossedOutOutlines;
                    if(crossedOutOutlines.includes(index))
                        newCrossedOutOutlines = crossedOutOutlines.filter(x=>x!==index);
                    else
                        newCrossedOutOutlines = crossedOutOutlines.concat(index);

                    GAME_MANAGER.sendSaveCrossedOutOutlinesPacket(newCrossedOutOutlines);
                }}
            >
                {
                    crossedOutOutlines.includes(index) ? 
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