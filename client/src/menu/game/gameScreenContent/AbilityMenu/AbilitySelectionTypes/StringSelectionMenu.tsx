import { ReactElement } from "react";
import React from "react";
import { TextDropdownArea } from "../../../../../components/TextAreaDropdown";
import { ControllerID, translateControllerID } from "../../../../../game/abilityInput";
import { usePlayerState } from "../../../../../components/useHooks";

export default function StringSelectionMenu(props: Readonly<{
    id?: ControllerID
    selection: string,
    onChoose: (string: string)=>void,
}>): ReactElement {

    const cantPost = usePlayerState(
        playerState => playerState.sendChatGroups.length === 0,
        ["yourSendChatGroups"]
    )!;
    
    let title = props.selection.split('\n')[0];
    if(props.id !== undefined){
        title = translateControllerID(props.id);
    }

    return <div>
        <TextDropdownArea
            open={true}
            titleString={title}
            savedText={props.selection}
            onSave={(s) => { props.onChoose(s); } }
            cantPost={cantPost}
        />    
    </div>
}