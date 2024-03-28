import React, { isValidElement, useMemo } from "react";
import { ReactElement, useState } from "react";
import Icon from "./Icon";


export type ButtonProps = Omit<JSX.IntrinsicElements['button'], 'onClick'> & {
    onClick?: (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => (boolean | void | Promise<boolean | void>)
    highlighted?: boolean,
    successChildren?: React.ReactNode,
    successText?: React.ReactNode,
    failureChildren?: React.ReactNode,
    failureText?: React.ReactNode
};

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
    let lastTimeout: NodeJS.Timeout | null = null;
    
    const children = useMemo(() => {
        switch (success) {
            case "unclicked":
                return props.children
            case "success":
                return <>
                    {successChildren || props.children}
                    {props.successText && <div className="fallible-button-popup">{props.successText}</div>}
                </>
            case "failure":
                return <>
                    {failureChildren || props.children}
                    {props.failureText && <div className="fallible-button-popup">{props.failureText}</div>}
                </>
        }
    }, [props.children, failureChildren, props.failureText, successChildren, props.successText, success]);

    return <button {...props} 
        className={
            "fallible-button " + (props.className ?? "") + (props.highlighted ? " highlighted" : "")
        }
        onClick={async e => {
            if (props.onClick) {
                const success = await props.onClick(e);
                if (success !== undefined) {
                    setSuccess(success ? "success" : "failure")
                    if (lastTimeout) clearTimeout(lastTimeout);
                    lastTimeout = setTimeout(() => setSuccess("unclicked"), 1000);
                }
            }
        }}
    >{children}</button>
}