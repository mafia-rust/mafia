
import { replaceMentions } from "..";
import { Grave } from "../game/graveState";
import translate from "../game/lang";
import { sanitizePlayerMessage } from "./ChatMessage";
import StyledText from "./StyledText";
import React, { ReactElement } from "react";
import "./grave.css";



export default function GraveComponent(props: {grave: Grave, playerNames: string[]}): ReactElement {
    let deathCauseString: string;
    if(props.grave.deathCause.type === "killers") {
        deathCauseString = props.grave.deathCause.killers.map((killer)=>{
            switch(killer.type){
                case "role":
                    return translate("role."+killer.value+".name");
                case "faction":
                    return translate(killer.value);
                default:
                    return translate("grave.killer."+killer.type);
            }
        }).join(", ") + ".";
    }else{
        deathCauseString = translate("grave.deathCause."+props.grave.deathCause.type);
    }

    let graveRoleString: string;
    if (props.grave.role.type === "role") {
        graveRoleString = translate(`role.${props.grave.role.role}.name`);
    } else {
        graveRoleString = translate(`grave.role.${props.grave.role.type}`);
    }

    let diedPhaseString = props.grave.diedPhase === "day" ? translate("day") : translate("phase.night");

    return <div className="grave">
        <div><StyledText>{`${diedPhaseString+" "+props.grave.dayNumber}`}</StyledText></div>
        <div><StyledText>{`${props.playerNames[props.grave.playerIndex]+" ("+graveRoleString+")"}`}</StyledText></div>
        <div><StyledText>{`${translate("menu.graveyard.killedBy")+" "+deathCauseString}`}</StyledText></div>
        {props.grave.will.length === 0 || <>
            {translate("alibi")}
            <div className="note-area">
                <StyledText>
                    {sanitizePlayerMessage(replaceMentions(
                        props.grave.will,
                        props.playerNames
                    ))}
                </StyledText>
            </div>
        </>}
        {props.grave.deathNotes.length === 0 || props.grave.deathNotes.map(note => <>
            {translate("grave.deathNote")}
            <div className="note-area">
                <StyledText>
                    {sanitizePlayerMessage(replaceMentions(
                        note,
                        props.playerNames
                    ))}
                </StyledText>
            </div>
        </>)}
    </div>;
}