import React, { ReactElement, useEffect, useMemo, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { replaceMentions } from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./willMenu.css";
import { Button } from "../../../components/Button";
import Icon from "../../../components/Icon";
import { usePlayerState } from "../../../components/useHooks";
import StyledText from "../../../components/StyledText";
import { sanitizePlayerMessage } from "../../../components/ChatMessage";
import { getSingleRoleJsonData } from "../../../game/roleState.d";


export default function WillMenu(): ReactElement {
    const cantPost = usePlayerState(
        playerState => playerState.sendChatGroups.length === 0,
        ["yourSendChatGroups"]
    )!;

    const role = usePlayerState(
        playerState => playerState.roleState.type,
        ["yourRoleState"]
    )!;

    const alibi = usePlayerState(
        playerState => playerState.will,
        ["yourWill"]
    )!;
    const notes = usePlayerState(
        playerState => playerState.notes,
        ["yourNotes"]
    )!;
    const deathNote = usePlayerState(
        playerState => playerState.deathNote,
        ["yourDeathNote"]
    )!;
    
    return <div className="will-menu will-menu-colors">
        <ContentTab
            close={ContentMenu.WillMenu}
            helpMenu={"standard/alibi"}
        >
                {translate("menu.will.title")}
        </ContentTab>
        <section>
            <TextDropdownArea
                titleString={translate("menu.will.will")}
                open={true}
                savedText={alibi}
                cantPost={cantPost}
                onSave={(text) => {
                    GAME_MANAGER.sendSaveWillPacket(text);
                }}
            />
            {Array.from({ length: notes.length }, (_, i) => i).map(i => {
                return <TextDropdownArea
                    key={i}
                    titleString={translate("menu.will.notes")}
                    savedText={notes[i]}
                    cantPost={cantPost}
                    onAdd={() => {
                        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                            const notes = [...GAME_MANAGER.state.clientState.notes];
                            notes.splice(i+1, 0, "");
                            GAME_MANAGER.sendSaveNotesPacket(notes);
                        }
                    }}
                    onSubtract={() => {
                        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                            const notes = [...GAME_MANAGER.state.clientState.notes];
                            notes.splice(i, 1);
                            GAME_MANAGER.sendSaveNotesPacket(notes);
                        }
                    }}
                    onSave={(text) => {
                        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                            const notes = [...GAME_MANAGER.state.clientState.notes];
                            notes[i] = text;
                            GAME_MANAGER.sendSaveNotesPacket(notes);
                        }
                    }}
                />
            })}
            {getSingleRoleJsonData(role).canWriteDeathNote===true ? <TextDropdownArea
                titleString={translate("menu.will.deathNote")}
                savedText={deathNote}
                cantPost={cantPost}
                onSave={(text) => {
                    GAME_MANAGER.sendSaveDeathNotePacket(text);
                }}
            />:null}
        </section>
    </div>
}

function TextDropdownArea(props: Readonly<{
    titleString: string,
    savedText: string,
    open?: boolean,
    onAdd?: () => void,
    onSubtract?: () => void,
    onSave: (text: string) => void,
    cantPost: boolean
}>): ReactElement {

    const savedField = props.savedText;
    const [field, setField] = useState<string>(savedField);

    useEffect(() => {
        setField(savedField)
    }, [savedField])

    const unsaved = useMemo(() => {
        return savedField !== field
    }, [field, savedField]);


    function send(field: string){
        save(field);
        GAME_MANAGER.sendSendMessagePacket('\n' + field);
    }

    function save(field: string) {
        props.onSave(field);
    }

    return (<details open={props.open===undefined?false:props.open}>
        <summary>
            <div>
                {props.titleString}
                <div>
                    {unsaved ? "Unsaved" : "Saved"}
                    {props.onSubtract ? <Button
                        onClick={() => {
                            if(props.onSubtract)
                                props.onSubtract();
                        }}
                        pressedChildren={() => <Icon>done</Icon>}
                        aria-label={translate("menu.will.subtract")}
                    >
                        <Icon>remove</Icon>
                    </Button> : null}
                    {props.onAdd ? <Button
                        onClick={() => {
                            if(props.onAdd)
                                props.onAdd();
                        }}
                        pressedChildren={() => <Icon>done</Icon>}
                        aria-label={translate("menu.will.add")}
                    >
                        <Icon>add</Icon>
                    </Button> : null}
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
            </div>
        </summary>
        <PrettyTextArea
            field={field}
            setField={setField}
            save={save}
            send={send}
        />
    </details>)
}

function PrettyTextArea(props: Readonly<{
    field: string,
    setField: (field: string) => void,
    save: (field: string) => void,
    send: (field: string) => void,
}>): ReactElement {
    const [writing, setWriting] = useState<boolean>(false);
    const [hover, setHover] = useState<boolean>(false);

    return <div
        onMouseEnter={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
        onFocus={() => setWriting(true)}
        onBlur={() => setWriting(false)}
    >
        {(!writing && !hover)
            ? <div className="textarea">
                <StyledText noLinks={true}>{sanitizePlayerMessage(replaceMentions(props.field))}</StyledText>
            </div>
            : <textarea
                value={props.field}
                onChange={e => props.setField(e.target.value)}
                onKeyDown={(e) => {
                    if (e.ctrlKey) {
                        if (e.key === 's') {
                            e.preventDefault();
                            props.save(props.field);
                        } else if (e.key === "Enter") {
                            props.send(props.field);
                        }
                    }
                }}>
            </textarea>}
        </div>
}