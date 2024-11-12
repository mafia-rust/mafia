
import { replaceMentions } from "..";
import { Grave } from "../game/graveState";
import translate from "../game/lang";
import { sanitizePlayerMessage } from "./ChatMessage";
import StyledText from "./StyledText";
import React, { ReactElement } from "react";
import "./grave.css";
import { useGameState } from "./useHooks";

export function translateGraveRole(grave: Grave): string {
    if(grave.information.type === "obscured") {
        return translate("obscured");
    }else{
        return translate(`role.${grave.information.role}.name`);
    }
}

export default function GraveComponent(props: Readonly<{
    grave: Grave, 
    playerNames?: string[]
    onClick?: () => void
}>): ReactElement {


    const gamePlayerNames = useGameState(
        gameState => gameState.players.map(player => player.toString()),
        ["gamePlayers"]
    )!

    const playerNames = props.playerNames===undefined ? gamePlayerNames: props.playerNames;


    if(props.grave.information.type === "obscured") {
        return <ObscuredGrave grave={props.grave} playerNames={playerNames}/>
    }

    let deathCauseString: string;
    if(props.grave.information.deathCause.type === "killers") {
        deathCauseString = props.grave.information.deathCause.killers.map((killer)=>{
            switch(killer.type){
                case "role":
                    return translate("role."+killer.value+".name");
                case "roleSet":
                    return translate(killer.value);
                default:
                    return translate("grave.killer."+killer.type);
            }
        }).join(", ") + ".";
    }else{
        deathCauseString = translate("grave.deathCause."+props.grave.information.deathCause.type);
    }

    let graveRoleString = translate(`role.${props.grave.information.role}.name`);

    let diedPhaseString = props.grave.diedPhase === "day" ? translate("day") : translate("phase.night");

    return <div className="grave" onClick={()=>{
        if(props.onClick!==undefined)
            props.onClick();
        }}
    >
        <div><StyledText>{`${diedPhaseString+" "+props.grave.dayNumber}`}</StyledText></div>
        <div><StyledText>{`${playerNames[props.grave.player]+" ("+graveRoleString+")"}`}</StyledText></div>
        <div><StyledText>{`${translate("killedBy")+" "+deathCauseString}`}</StyledText></div>
        {props.grave.information.will.length === 0 || <>
            {translate("alibi")}
            <div className="note-area">
                <StyledText>
                    {sanitizePlayerMessage(replaceMentions(
                        props.grave.information.will,
                        playerNames
                    ))}
                </StyledText>
            </div>
        </>}
        {
            (props.grave.information.deathNotes.length === 0 || props.grave.information.deathNotes.map(note => <>
                {translate("grave.deathNote")}
                <div className="note-area">
                    <StyledText>
                        {sanitizePlayerMessage(replaceMentions(
                            note,
                            playerNames
                        ))}
                    </StyledText>
                </div>
            </>))
        }
    </div>;
}


function ObscuredGrave(props: Readonly<{grave: Grave, playerNames: string[]}>): ReactElement {

    let diedPhaseString = props.grave.diedPhase === "day" ? translate("day") : translate("phase.night");
    let graveRoleString = translate("obscured");

    return <div className="grave">
        <div><StyledText>{`${diedPhaseString+" "+props.grave.dayNumber}`}</StyledText></div>
        <div><StyledText>{`${props.playerNames[props.grave.player]+" ("+graveRoleString+")"}`}</StyledText></div>
    </div>;
}