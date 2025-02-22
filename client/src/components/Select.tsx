import React, { useCallback, useMemo, useRef } from "react";
import { Button, RawButton } from "./Button";
import "./select.css";
import Icon from "./Icon";
import Popover from "./Popover";

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

    const [open, setOpen] = React.useState(false);
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
                const allSearchResults = [...optionsSearch.keys()].filter((key) => {
                    for(const search of searchString.split(" ")) {
                        
                        const val = optionsSearch.get(key);
                        if(val === undefined) {return false}
                        if(!val[1].toLowerCase().includes(search.toLowerCase())){
                            return false;
                        }
                    }
                    return true;
                });

                //sort by length and take the first. If you type "witch" we don't want "syndicate witch"
                allSearchResults.sort((a, b) => a.toString().length - b.toString().length);

                if(allSearchResults[0] !== undefined) {
                    handleOnChange(allSearchResults[0]);
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

    const ref = useRef<HTMLButtonElement>(null);

    const value = optionsSearch.get(props.value);
    if(value === undefined) {
        console.error(`Value not found in options ${props.value}`);
    }

    return <>
        <RawButton
            ref={ref}
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
            {value !== undefined?value[0]:props.value.toString()}
        </RawButton>
        <Popover className="custom-select-options"
            open={open}
            setOpenOrClosed={handleSetOpen}
            onRender={dropdownPlacementFunction}
            anchorRef={ref}
        >
            <SelectOptions 
                options={optionsNoSearch}
                searchString={searchString===""?undefined:searchString.substring(0, 20)}
                onChange={(value)=>{
                    if(props.disabled) return;
                    handleSetOpen(false);
                    handleOnChange(value);
                }}
            />
        </Popover>
    </>
}

export function dropdownPlacementFunction(dropdownElement: HTMLElement, buttonElement: HTMLElement | undefined) {
    if (!buttonElement) return;

    const buttonBounds = buttonElement.getBoundingClientRect();
    dropdownElement.style.width = `${buttonBounds.width}px`;
    dropdownElement.style.left = `${buttonBounds.left}px`;

    const spaceAbove = buttonBounds.top;
    const spaceBelow = window.innerHeight - buttonBounds.bottom;

    const oneRem = parseFloat(getComputedStyle(buttonElement).fontSize);

    const maxHeight = (25 - .25) * oneRem;
    const optionsHeight = 1 + .5 * oneRem + (dropdownElement.firstElementChild?.clientHeight ?? Infinity);

    if (spaceAbove > spaceBelow) {
        const newHeight = Math.min(maxHeight, spaceAbove - .25 * oneRem, optionsHeight);
        dropdownElement.style.height = `${newHeight}px`;
        dropdownElement.style.top = `unset`;
        dropdownElement.style.bottom = `${spaceBelow + buttonBounds.height + .25 * oneRem}px`;
    } else {
        const newHeight = Math.min(maxHeight, spaceBelow - .25 * oneRem, optionsHeight);
        dropdownElement.style.height = `${newHeight}px`;
        dropdownElement.style.top = `${spaceAbove + buttonBounds.height + .25 * oneRem}px`;
        dropdownElement.style.bottom = `unset`;
    }

    keepPopoverOnScreen(dropdownElement, buttonElement);
}

function keepPopoverOnScreen(dropdownElement: HTMLElement, buttonElement?: HTMLElement) {
    const dropdownBounds = dropdownElement.getBoundingClientRect();

    const modifyTop = dropdownElement.style.bottom === 'unset' || dropdownElement.style.bottom === "";
    const modifyLeft = dropdownElement.style.right === 'unset' || dropdownElement.style.right === "";

    const spaceAbove = dropdownBounds.top;
    const spaceBelow = window.innerHeight - dropdownBounds.bottom;
    const spaceToTheRight = window.innerWidth - dropdownBounds.right;
    const spaceToTheLeft = dropdownBounds.left;

    if (spaceToTheRight < 0) {
        if (modifyLeft) {
            dropdownElement.style.left = `${window.innerWidth - dropdownBounds.width}px`
        } else {
            dropdownElement.style.right = "0px"
        }
    }

    if (spaceToTheLeft < 0) {
        if (modifyLeft) {
            dropdownElement.style.left = "0px"
        } else {
            dropdownElement.style.right = `${dropdownBounds.width}px`
        }
    }

    if (spaceBelow < 0) {
        if (modifyTop) {
            dropdownElement.style.top = `${window.innerHeight - dropdownBounds.height}px`
        } else {
            dropdownElement.style.bottom = "0px"
        }
    }

    if (spaceAbove < 0) {
        if (modifyTop) {
            dropdownElement.style.top = "0px"
        } else {
            dropdownElement.style.bottom = `${dropdownBounds.height}px`
        }
    }
}

function SelectOptions<K extends { toString(): string}>(props: Readonly<{
    searchString?: string,
    options: SelectOptionsNoSearch<K>,
    onChange?: (value: K)=>void,
}>) {
    return <div>
        {props.searchString ?? null}
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