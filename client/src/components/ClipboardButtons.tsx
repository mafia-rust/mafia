import React, { useState } from "react";
import { ReactElement } from "react";
import Anchor from "../menu/Anchor";
import translate from "../game/lang";
import "./clipboardButtons.css";

export function CopyButton(props: JSX.IntrinsicElements['button'] & { onClick?: undefined, ref?: undefined, text: string }): ReactElement {
    const iconMode = props.children === undefined;
    const [success, setSuccess] = useState<boolean>(false);

    return <button {...props} className={"clipboard-button " + (props.className ?? "")}
        onClick={() => {
            writeToClipboard(props.text).then(success => {
                if (!success) return;

                setSuccess(true);
                setTimeout(() => {
                    setSuccess(false);
                }, 1000);
            });
        }}
    >
        {props.children}
        {iconMode && (
            success 
                ? <span className="material-icons-round">done</span> 
                : <span className="material-icons-round">content_copy</span>
        )}
        {success && <div className="clipboard-popup">
            {translate("notification.clipboard.write.success")}
        </div>}
    </button>
}

export function PasteButton(props: JSX.IntrinsicElements['button'] & { onClick?: undefined, onPasteSuccessful: (text: string) => (void | boolean) } ): ReactElement {
    const iconMode = props.children === undefined;
    const [success, setSuccess] = useState<boolean>(false);

    return <button {...props} className={"clipboard-button " + (props.className ?? "")}
        onClick={() => {
            readFromClipboard().then(text => {
                if (text === null) return;
                if (!(props.onPasteSuccessful(text) ?? true)) return;

                setSuccess(true);
                setTimeout(() => {
                    setSuccess(false)
                }, 1000);
            });
        }}
    >
        {props.children}
        {iconMode && (
            success 
                ? <span className="material-icons-round">done</span> 
                : <span className="material-icons-round">paste</span>
        )}
        {success && <div className="clipboard-popup">
            {translate("notification.clipboard.read.success")}
        </div>}
    </button>
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