import React, { ReactElement, useCallback, useEffect, useMemo, useRef } from "react";
import "./select.css";
import ReactDOM from "react-dom/client";
import { THEME_CSS_ATTRIBUTES } from "..";

export type PopoverController = {
    setOpenOrClosed: (open: boolean) => void,
    open: boolean,
}

const PopoverContext = React.createContext<PopoverController | undefined>(undefined);

export default function Popover(props: Readonly<{
    open: boolean,
    children: JSX.Element,
    setOpenOrClosed: (open: boolean) => void,
    anchorRef?: React.RefObject<HTMLElement>,
    className?: string
}>): ReactElement {
    const handleSetOpen = useCallback((isOpen: boolean) => {
        props.setOpenOrClosed(isOpen);
    }, [props]);

    const thisRef = useRef<HTMLDivElement>(null);
    const popoverRef = useRef<HTMLDivElement>(document.createElement('div'));

    const popoverRoot = useMemo(() => {
        const popoverElement = popoverRef.current;
        popoverElement.style.position = "absolute";

        document.body.appendChild(popoverElement);
        return ReactDOM.createRoot(popoverElement);
    }, [])

    //set ref
    useEffect(() => {
        const initialPopover = popoverRef.current;
        return () => {
            setTimeout(() => {
                popoverRoot.unmount();
            })
            initialPopover.remove();
            
            popoverRef.current = document.createElement('div');
        }
    }, [popoverRoot])

    //match css styles
    useEffect(() => {
        const styleBenefactor = props.anchorRef?.current ?? thisRef.current;
        const popoverElement = popoverRef.current;
        
        if (styleBenefactor) {
            // Match styles
            THEME_CSS_ATTRIBUTES.forEach(prop => {
                popoverElement.style.setProperty(`--${prop}`, getComputedStyle(styleBenefactor).getPropertyValue(`--${prop}`))
            })

            popoverElement.className = 'popover ' + (props.className ?? '')
        }
    }, [props.anchorRef, props.className])

    // This is for the popover's anchor, not the element named Anchor
    const [anchorLocation, setAnchorLocation] = React.useState(() => {
        const bounds = props.anchorRef?.current?.getBoundingClientRect();

        if (bounds) {
            return { top: bounds.top, left: bounds.left }
        } else {
            return {top: 0, left: 0}
        }
    });

    //close on scroll
    useEffect(() => {
        const listener = () => {
            const bounds = props.anchorRef?.current?.getBoundingClientRect();
            if (
                bounds &&
                props.open &&
                (
                    anchorLocation.top !== bounds?.top || 
                    anchorLocation.left !== bounds?.left
                )
            )
                handleSetOpen(false);
        };
        
        window.addEventListener("scroll", listener, true);
        window.addEventListener("resize", listener);
        return () => {
            window.removeEventListener("scroll", listener, true);
            window.removeEventListener("resize", listener);
        }
    })

    const PopoverContextToBeProvided = useMemo(() => ({
        setOpenOrClosed: props.setOpenOrClosed,
        open: props.open
    }), [props.open, props.setOpenOrClosed])

    //open and set position
    useEffect(() => {
        const anchorElement = props.anchorRef?.current;
        const popoverElement = popoverRef.current;

        if (anchorElement && props.open) {
            popoverRoot.render(
                <PopoverContext.Provider value={PopoverContextToBeProvided}>
                    {props.children}
                </PopoverContext.Provider>
            );


            popoverElement.hidden = false;

            const buttonBounds = anchorElement.getBoundingClientRect();
            // Position
            popoverElement.style.width = `${buttonBounds.width}px`;
            popoverElement.style.left = `${buttonBounds.left}px`;
            setAnchorLocation({top: buttonBounds.top, left: buttonBounds.left});

            const spaceAbove = buttonBounds.top;
            const spaceBelow = window.innerHeight - buttonBounds.bottom;

            const oneRem = parseFloat(getComputedStyle(anchorElement).fontSize);

            if (spaceAbove > spaceBelow) {
                const newHeight = Math.min((25 - .25) * oneRem, spaceAbove - .25 * oneRem);
                popoverElement.style.height = `${newHeight}px`;
                popoverElement.style.top = `unset`;
                popoverElement.style.bottom = `${spaceBelow + buttonBounds.height + .25 * oneRem}px`;
            } else {
                const newHeight = Math.min((25 - .25) * oneRem, spaceBelow - .25 * oneRem);
                popoverElement.style.height = `${newHeight}px`;
                popoverElement.style.top = `${spaceAbove + buttonBounds.height + .25 * oneRem}px`;
                popoverElement.style.bottom = `unset`;
            }
        } else {
            popoverElement.hidden = true;
        }
    }, [handleSetOpen, props.open, props.children, popoverRoot, props.anchorRef, PopoverContextToBeProvided])

    //close on click outside
    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (!popoverRef.current?.contains(event.target as Node) && props.open) {
                handleSetOpen(false);
            }
        };

        setTimeout(() => {
            document.addEventListener("click", handleClickOutside);
        })
        return () => document.removeEventListener("click", handleClickOutside);
    }, [handleSetOpen, props.open]);

    return <div ref={thisRef} />
}