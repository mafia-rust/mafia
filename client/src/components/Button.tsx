import React, { useEffect, useMemo, useRef, ReactElement, useState, forwardRef } from "react";
import "./button.css";
import ReactDOM from "react-dom/client";
import { THEME_CSS_ATTRIBUTES } from "..";

export type ButtonProps<R> = Omit<JSX.IntrinsicElements['button'], 'onClick' | 'ref'> & {
    onClick?: (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => (R | void | Promise<R | void>)
    highlighted?: boolean,
    pressedChildren?: (result: R) => React.ReactNode
    pressedText?: (result: R) => React.ReactNode
};

function reconcileProps<R>(props: ButtonProps<R>): JSX.IntrinsicElements['button'] {
    const newProps = {...props};
    delete newProps.onClick;
    delete newProps.highlighted;
    delete newProps.pressedChildren;
    delete newProps.pressedText;

    return newProps;
}

const POPUP_TIMEOUT_MS = 1000;

export function Button<R>(props: ButtonProps<R>): ReactElement {
    return <RawButton {...props} />
}

const RawButton = forwardRef<HTMLButtonElement, ButtonProps<any>>(function RawButton<R>(props: ButtonProps<R>, passedRef: React.ForwardedRef<HTMLButtonElement>): ReactElement {
    const [success, setSuccess] = useState<R | "unclicked">("unclicked");
    const ref = useRef<HTMLButtonElement>(null);

    useEffect(() => {
        if (typeof passedRef === "function") {
            passedRef(ref.current);
        } else if (passedRef) {
            passedRef.current = ref.current
        } else if (passedRef === null) {
            passedRef = { current: ref.current }
        }
    }, [props, ref]);

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
        if (success === "unclicked" || props.pressedChildren === undefined) return props.children;
        return props.pressedChildren(success) || props.children;
    }, [props, success]);

    return <button {...reconcileProps(props)} ref={ref}
        className={
            "button " + (props.className ?? "") + (props.highlighted ? " highlighted" : "")
        }
        onClick={async e => {
            if (props.onClick) {
                const result = await props.onClick(e);
                if (result === undefined) return;

                setSuccess(result);
                if (props.pressedText !== undefined) showPopup(props.pressedText(result))

                if (lastTimeout) clearTimeout(lastTimeout);
                lastTimeout = setTimeout(() => {
                    setSuccess("unclicked")
                    hidePopup();
                }, POPUP_TIMEOUT_MS);
            }
        }}
    >{children}</button>
})

export { RawButton };

function ButtonPopup(props: { children: React.ReactNode, button: HTMLButtonElement }): ReactElement {
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if ((ref.current) === null) return;

        const buttonBounds = props.button.getBoundingClientRect();
        ref.current.style.top = `${buttonBounds.bottom}px`;
        ref.current.style.left = `${(buttonBounds.left + buttonBounds.width / 2)}px`;
        THEME_CSS_ATTRIBUTES.forEach(prop => {
            if ((ref.current) === null) return;
            ref.current.style.setProperty(`--${prop}`, getComputedStyle(props.button).getPropertyValue(`--${prop}`))
        })
    });
    
    return <div className="button-popup" ref={ref}>{props.children}</div>
}