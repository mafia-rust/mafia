import React, { ReactElement, useEffect, useMemo, useState } from "react";
import StyledText from "./StyledText";
import { sanitizePlayerMessage } from "./ChatMessage";
import GAME_MANAGER, { replaceMentions } from "..";
import { Button } from "./Button";
import Icon from "./Icon";
import translate from "../game/lang";
import "./textAreaDropdown.css";
import DetailsSummary from "./DetailsSummary";

export function TextDropdownArea(props: Readonly<{
    titleString: string,
    savedText: string,
    defaultOpen?: boolean,
    open?: boolean,
    dropdownArrow?: boolean,
    onAdd?: () => void,
    onSubtract?: () => void,
    onSave: (text: string) => void,
    cantPost: boolean
}>): ReactElement {
    const [field, setField] = useState<string>(props.savedText);

    useEffect(() => {
        setField(props.savedText)
    }, [props.savedText])

    const unsaved = useMemo(() => {
        return props.savedText !== field
    }, [field, props.savedText]);

    function send(field: string){
        save(field);
        GAME_MANAGER.sendSendChatMessagePacket('\n' + field, true);
    }

    function save(field: string) {
        props.onSave(field);
    }

    return (
        <DetailsSummary
            className="text-area-dropdown"
            dropdownArrow={props.dropdownArrow}
            defaultOpen={props.defaultOpen}
            open={props.open}
            summary={<TextDropdownLabel
                titleString={props.titleString}
                savedText={props.savedText}
                field={field}
                onAdd={props.onAdd}
                onSubtract={props.onSubtract}
                onSave={save}
                cantPost={props.cantPost}
            />}
        >
            {unsaved ? "Unsaved" : ""}
            <PrettyTextArea
                field={field}
                setField={setField}
                save={save}
                send={send}
            />
        </DetailsSummary>
    )
}

function TextDropdownLabel(
    props: Readonly<{
        titleString: string,
        savedText: string,
        field: string,
        open?: boolean,
        onAdd?: () => void,
        onSubtract?: () => void,
        onSave: (text: string) => void,
        cantPost: boolean
    }>
): ReactElement {
    
    const unsaved = useMemo(() => {
        return props.savedText !== props.field
    }, [props.field, props.savedText]);

    function save(field: string) {
        props.onSave(field);
    }

    function send(field: string){
        save(field);
        GAME_MANAGER.sendSendChatMessagePacket('\n' + field, true);
    }

    return <div>
        <StyledText>{replaceMentions(props.titleString)}</StyledText>
        <span>
            {props.onSubtract ? <Button
                onClick={() => {
                    if(props.onSubtract)
                        props.onSubtract();
                }}
                pressedChildren={() => <Icon size="small">done</Icon>}
                aria-label={translate("menu.will.subtract")}
            >
                <Icon size="small">remove</Icon>
            </Button> : null}
            {props.onAdd ? <Button
                onClick={() => {
                    if(props.onAdd)
                        props.onAdd();
                }}
                pressedChildren={() => <Icon size="small">done</Icon>}
                aria-label={translate("menu.will.add")}
            >
                <Icon size="small">add</Icon>
            </Button> : null}
            <Button
                highlighted={unsaved}
                onClick={() => {
                    save(props.field);
                    return true;
                }}
                pressedChildren={() => <Icon size="small">done</Icon>}
                aria-label={translate("menu.will.save")}
            >
                <Icon size="small">save</Icon>
            </Button>
            <Button
                disabled={props.cantPost}
                onClick={() => {
                    send(props.field);
                    return true;
                }}
                pressedChildren={() => <Icon size="small">done</Icon>}
                aria-label={translate("menu.will.post")}
            >
                <Icon size="small">send</Icon>
            </Button>
        </span>
    </div>
}

function PrettyTextArea(props: Readonly<{
    field: string,
    setField: (field: string) => void,
    save: (field: string) => void,
    send: (field: string) => void,
}>): ReactElement {
    const [writing, setWriting] = useState<boolean>(false);
    const [hover, setHover] = useState<boolean>(false);

    return <div className="pretty-text-area"
        onMouseEnter={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
        onTouchEnd={() => setWriting(true)}
        onFocus={() => setWriting(true)}
        onBlur={() => {setWriting(false); setHover(false)}}
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