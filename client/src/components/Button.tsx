import React, { isValidElement, useEffect, useMemo, useRef } from "react";
import { ReactElement, useState } from "react";
import Icon from "./Icon";
import "./button.css";
import ReactDOM from "react-dom/client";

export type ButtonProps = Omit<JSX.IntrinsicElements['button'], 'onClick' | 'ref'> & {
    onClick?: (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => (boolean | void | Promise<boolean | void>)
    highlighted?: boolean,
    successChildren?: React.ReactNode,
    successText?: React.ReactNode,
    failureChildren?: React.ReactNode,
    failureText?: React.ReactNode
};

const POPUP_TIMEOUT_MS = 1000;

export function Button(props: ButtonProps): ReactElement {
    let successChildren = props.successChildren;
    let failureChildren = props.failureChildren
    if (isValidElement(props.children) && props.children.type === Icon) {
        if (successChildren === undefined)
            successChildren = <Icon>done</Icon>;
        if (failureChildren === undefined)
            failureChildren = <Icon>warning</Icon>
    }

    const [success, setSuccess] = useState<"success" | "failure" | "unclicked">("unclicked");
    const ref = useRef<HTMLButtonElement>(null);
    const popupContainer = useRef<HTMLDivElement>(document.createElement('div'));

    let lastTimeout: NodeJS.Timeout | null = null;

    const showPopup = (content: React.ReactNode) => {
        if (ref.current === null) return;
        const root = ReactDOM.createRoot(popupContainer.current);
        root.render(<ButtonPopup button={ref.current}>{content}</ButtonPopup>);
        document.body.appendChild(popupContainer.current);
    }

    const hidePopup = () => {
        if (document.body.contains(popupContainer.current)) {
            document.body.removeChild(popupContainer.current)
        }
    }
    
    const children = useMemo(() => {
        switch (success) {
            case "unclicked": return props.children
            case "success": return successChildren || props.children
            case "failure": return failureChildren || props.children
        }
    }, [props.children, failureChildren, successChildren, success]);

    return <button {...props} ref={ref}
        className={
            "button " + (props.className ?? "") + (props.highlighted ? " highlighted" : "")
        }
        onClick={async e => {
            if (props.onClick) {
                const success = await props.onClick(e);
                switch (success) {
                    case undefined:
                        return;
                    case true:
                        setSuccess("success")
                        if (props.successText) showPopup(props.successText)
                        break;
                    case false:
                        setSuccess("failure")
                        if (props.failureText) showPopup(props.failureText)
                        break;
                }
                if (lastTimeout) clearTimeout(lastTimeout);
                lastTimeout = setTimeout(() => {
                    setSuccess("unclicked")
                    hidePopup();
                }, POPUP_TIMEOUT_MS);
            }
        }}
    >{children}</button>
}

function ButtonPopup(props: { children: React.ReactNode, button: HTMLButtonElement }): ReactElement {
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if ((ref.current) === null) return;

        const buttonBounds = props.button.getBoundingClientRect();
        ref.current.style.top = `${buttonBounds.bottom}px`;
        ref.current.style.left = `${(buttonBounds.left + buttonBounds.width / 2)}px`;
        ['background-color', 'primary-border-color', 'primary-color', 'secondary-color', 'tab-color'].forEach(prop => {
            if ((ref.current) === null) return;
            ref.current.style.setProperty(`--${prop}`, getComputedStyle(props.button).getPropertyValue(`--${prop}`))
        })
    });
    
    return <div className="button-popup" ref={ref}>{props.children}</div>
}