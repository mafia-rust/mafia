import React, { useState } from "react";
import { ReactElement } from "react";
import Anchor from "../menu/Anchor";
import translate from "../game/lang";
import { Button, ButtonProps } from "./Button";
import Icon from "./Icon";

type CopyButtonProps = ButtonProps & { onClick?: undefined, ref?: undefined, text: string };

function reconcileCopyProps(props: CopyButtonProps): ButtonProps {
    let newProps: any = {...props};
    delete newProps.onClick;
    delete newProps.ref;
    newProps.text = undefined;
    delete newProps.text;

    return newProps;
}

export function CopyButton(props: CopyButtonProps): ReactElement {
    return <Button {...reconcileCopyProps(props)} 
        onClick={() => writeToClipboard(props.text)}
        successText={translate("notification.clipboard.write.success")}
        failureText={translate("notification.clipboard.write.failure")}
    >
        {props.children ?? <Icon>content_copy</Icon>}
    </Button>
}

type PasteButtonProps = ButtonProps & { onClick?: undefined, onPasteSuccessful?: (text: string) => (void | boolean) };

function reconcilePasteProps(props: PasteButtonProps): ButtonProps {
    const newProps: any = {...props};
    delete newProps.onClick;
    delete newProps.onPasteSuccessful;

    return newProps;
}

export function PasteButton(props: PasteButtonProps): ReactElement {
    const [failureReason, setFailureReason] = useState<"clipboard" | "handler">("clipboard");
    
    return <Button {...reconcilePasteProps(props)}
        onClick={() => readFromClipboard().then(text => {
            if (text === null) {
                setFailureReason("clipboard");
                return false;
            } else if (props.onPasteSuccessful === undefined) {
                return true;
            } else {
                setFailureReason("handler");
                return props.onPasteSuccessful(text) ?? true;
            }
        })}
        successText={translate("notification.clipboard.read.success")}
        failureText={translate(
            failureReason === "clipboard" 
                ? "notification.clipboard.read.failure" 
                : "notification.clipboard.handleRead.failure"
        )}
    >
        {props.children ?? <Icon>paste</Icon>}
    </Button>
}

/**
 * Note: This function pushes an error card if it is unsuccessful
 * @returns Whether the clipboard was successfully written to.
 */
async function writeToClipboard(text: string): Promise<boolean> {
    if (!navigator.clipboard) {
        Anchor.pushError(
            translate("notification.clipboard.write.failure"), 
            translate("notification.clipboard.write.failure.noClipboard")
        );
        return false;
    }

    try {
        await navigator.clipboard.writeText(text);
        return true;
    } catch (error) {
        Anchor.pushError(
            translate("notification.clipboard.read.failure"), 
            translate("notification.clipboard.read.failure.notAllowed")
        );
        return false;
    }
}

/**
 * Note: This function pushes an error card if it is unsuccessful
 * @returns The string read from the clipboard, and null on any kind of failure.
 */
async function readFromClipboard(): Promise<string | null> {
    if (!navigator.clipboard) {
        Anchor.pushError(
            translate("notification.clipboard.read.failure"), 
            translate("notification.clipboard.read.failure.noClipboard")
        );
        return null;
    }

    try {
        const text = await navigator.clipboard.readText();
        return text;
    } catch (error) {
        switch ((error as any as DOMException).name) {
            case "NotFoundError":
                Anchor.pushError(
                    translate("notification.clipboard.read.failure"), 
                    translate("notification.clipboard.read.failure.notFound")
                );
                return null;
            case "NotAllowedError":
            default:
                Anchor.pushError(
                    translate("notification.clipboard.read.failure"), 
                    translate("notification.clipboard.read.failure.notAllowed")
                );
                return null;
        }
    }
}