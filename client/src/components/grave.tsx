
import { replaceMentions } from "..";
import { Grave } from "../game/graveState";
import translate from "../game/lang";
import { sanitizePlayerMessage } from "./ChatMessage";
import StyledText from "./StyledText";
import React, { ReactElement } from "react";
import "./grave.css";



export default function GraveComponent(props: {grave: Grave, playerNames: string[]}): ReactElement {
    if(props.grave.information.type === "obscured") {
        return <ObscuredGrave grave={props.grave} playerNames={props.playerNames}/>
    }

    let deathCauseString: string;
    if(props.grave.information.deathCause.type === "killers") {
        deathCauseString = props.grave.information.deathCause.killers.map((killer)=>{
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
        deathCauseString = translate("grave.deathCause."+props.grave.information.deathCause.type);
    }

    let graveRoleString = translate(`role.${props.grave.information.role}.name`);

    let diedPhaseString = props.grave.diedPhase === "day" ? translate("day") : translate("phase.night");

    return <div className="grave">
        <div><StyledText>{`${diedPhaseString+" "+props.grave.dayNumber}`}</StyledText></div>
        <div><StyledText>{`${props.playerNames[props.grave.player]+" ("+graveRoleString+")"}`}</StyledText></div>
        <div><StyledText>{`${translate("menu.graveyard.killedBy")+" "+deathCauseString}`}</StyledText></div>
        {props.grave.information.will.length === 0 || <>
            {translate("alibi")}
            <div className="note-area">
                <StyledText>
                    {sanitizePlayerMessage(replaceMentions(
                        props.grave.information.will,
                        props.playerNames
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
                            props.playerNames
                        ))}
                    </StyledText>
                </div>
            </>))
        }
    </div>;
}


function ObscuredGrave(props: {grave: Grave, playerNames: string[]}): ReactElement {

    let diedPhaseString = props.grave.diedPhase === "day" ? translate("day") : translate("phase.night");
    let graveRoleString = translate("obscured");

    return <div className="grave">
        <div><StyledText>{`${diedPhaseString+" "+props.grave.dayNumber}`}</StyledText></div>
        <div><StyledText>{`${props.playerNames[props.grave.player]+" ("+graveRoleString+")"}`}</StyledText></div>
    </div>;
}