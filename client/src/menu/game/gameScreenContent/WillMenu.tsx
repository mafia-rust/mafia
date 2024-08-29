import React, { ReactElement, useMemo, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./willMenu.css"
import { StateEventType } from "../../../game/gameManager.d";
import { Button } from "../../../components/Button";
import Icon from "../../../components/Icon";
import { usePlayerState } from "../../../components/useHooks";


type FieldType = "will" | "notes" | "deathNote";

export default function WillMenu(): ReactElement {
    const cantPost = usePlayerState(
        playerState => playerState.sendChatGroups.length === 0,
        ["yourSendChatGroups"]
    )!;
    
    return <div className="will-menu will-menu-colors">
        <ContentTab close={ContentMenu.WillMenu} helpMenu={"standard/alibi"}>{translate("menu.will.title")}</ContentTab>
        <section>
            <TextDropdownArea type="will" cantPost={cantPost} />
            <TextDropdownArea type="notes" cantPost={cantPost} />
            <TextDropdownArea type="deathNote" cantPost={cantPost} />
        </section>
    </div>
}

function TextDropdownArea(props: Readonly<{ type: FieldType, cantPost: boolean }>): ReactElement {
    let packet: StateEventType = (() => {
        switch (props.type) {
            case "will":
                return "yourWill"
            case "notes":
                return "yourNotes"
            case "deathNote":
                return "yourDeathNote"
        }
    })();
    const savedField = usePlayerState(
        playerState => playerState[props.type],
        [packet]
    )!;
    const [field, setField] = useState<string>(savedField);

    const unsaved = useMemo(() => {
        return savedField !== field
    }, [field, savedField]);


    function send(field: string){
        save(field);
        GAME_MANAGER.sendSendMessagePacket('\n' + field);
    }

    function save(field: string) {
        switch(props.type){
            case "will":
                GAME_MANAGER.sendSaveWillPacket(field);
                break;
            case "notes":
                GAME_MANAGER.sendSaveNotesPacket(field);
                break;
            case "deathNote":
                GAME_MANAGER.sendSaveDeathNotePacket(field);
                break;
        }
    }

    return (<details open={props.type !== "deathNote"}>
        <summary>
            {translate("menu.will." + props.type)}
            <div>
                {unsaved ? "Unsaved" : "Saved"}
                <Button
                    highlighted={unsaved}
                    onClick={() => {
                        save(field);
                        return true;
                    }}
                    pressedChildren={() => <Icon>done</Icon>}
                    aria-label={translate("menu.will.save")}
                >
                    <Icon>save</Icon>
                </Button>
                <Button
                    disabled={props.cantPost}
                    onClick={() => {
                        send(field);
                        return true;
                    }}
                    pressedChildren={() => <Icon>done</Icon>}
                    aria-label={translate("menu.will.post")}
                >
                    <Icon>send</Icon>
                </Button>
            </div>
        </summary>
        
        <div>
            <textarea
                value={field}
                onChange={e => setField(e.target.value)}
                onKeyDown={(e) => {
                    if (e.ctrlKey) {
                        if (e.key === 's') {
                            e.preventDefault();
                            save(field);
                        } else if (e.key === "Enter") {
                            send(field);
                        }
                    }
                }}>
            </textarea>
        </div>
    </details>)
}