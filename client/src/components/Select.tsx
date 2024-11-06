import React, { useCallback, useEffect, useMemo, useRef } from "react";
import { Button, RawButton } from "./Button";
import "./select.css";
import Icon from "./Icon";
import ReactDOM from "react-dom/client";
import { THEME_CSS_ATTRIBUTES } from "..";

export type SelectOptionsNoSearch<K extends { toString(): string}> = Map<K, React.ReactNode>;
export type SelectOptionsSearch<K extends { toString(): string}> = Map<K, [React.ReactNode, string]>;

export default function Select<K extends { toString(): string}>(props: Readonly<{
    value: K,
    disabled?: boolean,
    className?: string,
    onChange?: (value: K)=>void
} & ({
    optionsSearch: SelectOptionsSearch<K>,
} | {
    optionsNoSearch: SelectOptionsNoSearch<K>,
})>) {
    const optionsSearch: SelectOptionsSearch<K> = useMemo(() => {
        if("optionsSearch" in props) {
            return props.optionsSearch;
        } else {
            let optionsSearch = new Map<K, [React.ReactNode, string]>()

            for(let [key, val] of props.optionsNoSearch.entries()) {
                optionsSearch.set(key, [val, key.toString()]);
            }
            return optionsSearch
        }
    }, [props]);

    const optionsNoSearch: SelectOptionsNoSearch<K> = useMemo(() => {
        if ("optionsSearch" in props) {
            let optionsNoSearch = new Map<K, React.ReactNode>()

            for(let [key, val] of props.optionsSearch.entries()) {
                optionsNoSearch.set(key, val[0]);
            }

            return optionsNoSearch;
        } else {
            return props.optionsNoSearch;
        }
    }, [props]);

    const [open, setOpen]= React.useState(false);
    const [searchString, setSearchString] = React.useState("");
    

    const handleOnChange = useCallback((key: K) => {
        setSearchString("");
        if(props.onChange && key !== props.value) {
            props.onChange(key);
        }
    }, [props]);
    const handleSetOpen = useCallback((isOpen: boolean) => {
        setOpen(isOpen);
        setSearchString("");
    }, []);

    const handleKeyInput = (inputKey: string) => {
        switch(inputKey) {
            case "ArrowDown":
                handleSetOpen(true);
                break;
            case "Escape":
                handleSetOpen(false);
                break;
            case "Enter": {
                const found = [...optionsSearch.keys()].find((key) => {
                    for(const search of searchString.split(" ")) {
                        
                        const val = optionsSearch.get(key);
                        if(val === undefined) {return false}
                        if(!val[1].toLowerCase().includes(search.toLowerCase())){
                            return false;
                        }
                    }
                    return true;
                });
        
                if(found !== undefined) {
                    handleOnChange(found);
                }
                handleSetOpen(false);

                break;
            }
            case "Backspace":
                setSearchString("");
                break;
            default:
                if(/^[a-zA-Z0-9- ]$/.test(inputKey)) {
                    setSearchString(searchString+inputKey);
                }
        }
    }

    const buttonRef = useRef<HTMLButtonElement>(null);
    const dropdownRef = useRef<HTMLDivElement>(document.createElement('div'));

    const dropdownRoot = useMemo(() => {
        const dropdownElement = dropdownRef.current;
        dropdownElement.style.position = "absolute";

        document.body.appendChild(dropdownElement);
        return ReactDOM.createRoot(dropdownElement);
    }, [])

    useEffect(() => {
        const initialDropdown = dropdownRef.current;
        return () => {
            setTimeout(() => {
                dropdownRoot.unmount();
            })
            initialDropdown.remove();
            
            dropdownRef.current = document.createElement('div');
        }
    }, [dropdownRoot])

    useEffect(() => {
        const buttonElement = buttonRef.current;
        const dropdownElement = dropdownRef.current;
        
        if (buttonElement) {
            // Match styles
            THEME_CSS_ATTRIBUTES.forEach(prop => {
                dropdownElement.style.setProperty(`--${prop}`, getComputedStyle(buttonElement).getPropertyValue(`--${prop}`))
            })

            dropdownElement.className = 'custom-select-options'
        }
    }, [])

    useEffect(() => {
        const listener = (ev: Event) => {
            if (!(ev.target instanceof Node) || !dropdownRef.current.contains(ev.target)) {
                setOpen(false);
            }
        };
        
        window.addEventListener("scroll", listener, true);
        window.addEventListener("resize", listener);
        return () => {
            window.removeEventListener("scroll", listener, true);
            window.removeEventListener("resize", listener);
        }
    })

    useEffect(() => {
        const buttonElement = buttonRef.current;
        const dropdownElement = dropdownRef.current;

        if (buttonElement && open) {
            dropdownRoot.render(<SelectOptions
                options={optionsNoSearch}
                onChange={(value)=>{
                    if(props.disabled) return;
                    handleSetOpen(false);
                    handleOnChange(value);
                }}
            />);

            dropdownElement.hidden = false;

            const buttonBounds = buttonElement.getBoundingClientRect();
            // Position
            dropdownElement.style.width = `${buttonBounds.width}px`;
            dropdownElement.style.left = `${buttonBounds.left}px`;

            const spaceAbove = buttonBounds.top;
            const spaceBelow = window.innerHeight - buttonBounds.bottom;

            const oneRem = parseFloat(getComputedStyle(buttonElement).fontSize);

            if (spaceAbove > spaceBelow) {
                const newHeight = Math.min((25 - .25) * oneRem, spaceAbove - .25 * oneRem);
                dropdownElement.style.height = `${newHeight}px`;
                dropdownElement.style.top = `unset`;
                dropdownElement.style.bottom = `${spaceBelow + buttonBounds.height + .25 * oneRem}px`;
            } else {
                const newHeight = Math.min((25 - .25) * oneRem, spaceBelow - .25 * oneRem);
                dropdownElement.style.height = `${newHeight}px`;
                dropdownElement.style.top = `${spaceAbove + buttonBounds.height + .25 * oneRem}px`;
                dropdownElement.style.bottom = `unset`;
            }
        } else {
            dropdownElement.hidden = true;
        }
    }, [handleOnChange, handleSetOpen, open, props.disabled, optionsNoSearch, dropdownRoot])

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (!dropdownRef.current?.contains(event.target as Node) && open) {
                handleSetOpen(false);
            }
        };

        setTimeout(() => {
            document.addEventListener("click", handleClickOutside);
        })
        return () => document.removeEventListener("click", handleClickOutside);
    }, [handleSetOpen, open]);

    const value = optionsSearch.get(props.value);
    if(value === undefined) {
        throw new Error("Select value not in options");
    }

    return <RawButton
        ref={buttonRef}
        disabled={props.disabled}
        onClick={()=>{handleSetOpen(!open)}}
        className={"custom-select "+(props.className?props.className:"")}
        onKeyDown={(e)=>{
            if(props.disabled) return;
            if(e.key === "Enter" && !open) {
                e.preventDefault();
                handleSetOpen(true);
            }else if(e.key === "Tab") {
                handleSetOpen(false);
            }else{
                e.preventDefault();
                handleKeyInput(e.key);
            }
        }}
    >
        {open === true ? 
            <Icon>keyboard_arrow_up</Icon> :
            <Icon>keyboard_arrow_down</Icon>}
        {value[0]}
    </RawButton>
}

function SelectOptions<K extends { toString(): string}>(props: Readonly<{
    options: SelectOptionsNoSearch<K>,
    onChange?: (value: K)=>void,
}>) {

    return <div>
        {[...props.options.entries()]
            .map(([key, value]) => {
                return <Button
                    key={key.toString()}
                    onClick={()=>{
                        if(props.onChange) {
                            props.onChange(key);
                        }
                    }}
                >
                    {value}
                </Button>
            })
        }
    </div>
}