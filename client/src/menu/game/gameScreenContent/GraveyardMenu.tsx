import React, { ReactElement, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import StyledText from "../../../components/StyledText";
import GraveComponent from "../../../components/grave";
import { Grave } from "../../../game/graveState";
import Icon from "../../../components/Icon";
import { EnabledRolesDisplay } from "../../../components/gameModeSettings/EnabledRoleSelector";
import { useGameState, usePlayerState } from "../../../components/useHooks";
import { translateRoleOutline } from "../../../game/roleListState.d";

export default function GraveyardMenu(): ReactElement {
    const graves = useGameState(
        gameState => gameState.graves,
        ["addGrave"]
    )!
    const [extendedGraveIndex, setExtendedGraveIndex] = useState<number | null>(null);
    
    return <div className="graveyard-menu graveyard-menu-colors">
        <ContentTab close={ContentMenu.GraveyardMenu} helpMenu={"standard/graveyard"}>{translate("menu.graveyard.title")}</ContentTab>
            
        <div className="grid">
            <RoleListElement />
            <GraveList graves={graves} extendedGraveIndex={extendedGraveIndex} setExtendedGraveIndex={setExtendedGraveIndex}/>
        </div>
        {extendedGraveIndex !== null && 
            <GraveExtended 
                grave={graves[extendedGraveIndex]} 
                setExtendedGraveIndex={setExtendedGraveIndex}
            />
        }
        <ExcludedRoles />
    </div>
}

function RoleListElement(): ReactElement {
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

            return <button 
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
            </button>
        })}
    </>
}

function GraveList(props: Readonly<{ 
    graves: Grave[],
    extendedGraveIndex: number | null,
    setExtendedGraveIndex: (index: number | null) => void
}>): ReactElement {
    const playerNames = useGameState(
        gameState => gameState.players.map(player => player.toString()),
        ["gamePlayers"]
    )!

    return <>
        {props.graves.map((grave, graveIndex)=>{
            return <GraveButton
                key={playerNames[grave.player]} 
                grave={grave} 
                graveIndex={graveIndex}
                extended={props.extendedGraveIndex === graveIndex}
                playerName={playerNames[grave.player]}
                setExtendedGraveIndex={props.setExtendedGraveIndex}
            />;
        })}
    </>
}

function GraveButton(props: Readonly<{ 
    grave: Grave,
    graveIndex: number,
    extended: boolean,
    playerName: string,
    setExtendedGraveIndex: (index: number | null) => void
}>): ReactElement {
    let graveRoleString: string;
    if (props.grave.information.type === "normal") {
        graveRoleString = translate(`role.${props.grave.information.role}.name`);
    } else {
        graveRoleString = translate("obscured");
    }

    return(<button
        className="grave-list-button"
        style={{ gridRow: props.graveIndex + 1 }} 
        onClick={()=>{
            if(props.extended)
                props.setExtendedGraveIndex(null);
            else
                props.setExtendedGraveIndex(props.graveIndex);
        }}
    >
        <span>
            {
                props.extended ? 
                    <Icon>menu</Icon> :
                    <Icon>menu_open</Icon>
            }
            <StyledText noLinks={true}>{props.playerName}</StyledText>
            <StyledText noLinks={true}>
                {` (${graveRoleString})`}
            </StyledText>
        </span>
    </button>);
}

function GraveExtended(props: Readonly<{
    grave: Grave, 
    setExtendedGraveIndex: (index: number | null) => void
}>): ReactElement {
    const playerNames = useGameState(
        gameState => gameState.players.map(player => player.toString()),
        ["gamePlayers"]
    )!

    return <div className="grave-label">
        <button onClick={() => props.setExtendedGraveIndex(null)}>
            <Icon>close</Icon>
        </button>
        <GraveComponent grave={props.grave} playerNames={playerNames}/>
    </div>;
}

function ExcludedRoles(): ReactElement {
    const enabledRoles = useGameState(
        gameState => gameState.enabledRoles,
        ["enabledRoles"]
    )!

    return <details className="graveyard-menu-excludedRoles">
        <summary>
            {translate("menu.enabledRoles.enabledRoles")}
        </summary>
        <EnabledRolesDisplay enabledRoles={enabledRoles}/>
    </details>
}