import React, { ReactElement, useEffect, useMemo, useRef } from "react";
import "./select.css";
import ReactDOM from "react-dom/client";
import { THEME_CSS_ATTRIBUTES } from "..";
export default function Popover<T extends HTMLElement = HTMLElement>(props: Readonly<{
    open: boolean,
    children: JSX.Element,
    setOpenOrClosed: (open: boolean) => void,
    onRender?: (popoverElement: HTMLDivElement, anchorElement?: T | undefined) => void
    anchorRef?: React.RefObject<T>,
    className?: string
}>): ReactElement {
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
            props.setOpenOrClosed(false);
        };
        
        window.addEventListener("scroll", listener, true);
        window.addEventListener("resize", listener);
        return () => {
            window.removeEventListener("scroll", listener, true);
            window.removeEventListener("resize", listener);
        }
    })

    const popoverContext = useMemo(() => {
        return {
            popoverElement: popoverRef.current,
            anchorElement: props.anchorRef?.current ?? undefined,
            open: props.open
        }
    }, [props.anchorRef, props.open])

    //open and set position
    useEffect(() => {
        const popoverElement = popoverRef.current;
        const anchorElement = props.anchorRef?.current;

        if (props.open) {
            popoverRoot.render(props.children);

            if (anchorElement) {
                const anchorBounds = anchorElement.getBoundingClientRect();

                setAnchorLocation({top: anchorBounds.top, left: anchorBounds.left});
            }

            setTimeout(() => {
                popoverElement.hidden = false;
                
                if (props.onRender) {
                    props.onRender(popoverElement, anchorElement ?? undefined)
                }
            })
        } else {
            popoverElement.hidden = true;
        }
    }, [props, popoverRoot, popoverContext])

    //close on click outside
    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (!popoverRef.current?.contains(event.target as Node) && props.open) {
                props.setOpenOrClosed(false);
            }
        };

        setTimeout(() => {
            document.addEventListener("click", handleClickOutside);
        })
        return () => document.removeEventListener("click", handleClickOutside);
    }, [props]);

    return <div ref={thisRef} />
}