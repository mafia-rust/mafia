import React, { useCallback, useEffect } from "react";
import { Button } from "./Button";
import "./select.css";
import Icon from "./Icon";

export type SelectOptionsNoSearch<K extends { toString(): string}> = Map<K, React.ReactNode>;
export type SelectOptionsSearch<K extends { toString(): string}> = Map<K, [React.ReactNode, string]>;

export default function Select<K extends { toString(): string}>(props: {
    value: K,
    disabled?: boolean,
    className?: string,
    onChange?: (value: K)=>void
} & ({
    optionsSearch: SelectOptionsSearch<K>,
} | {
    optionsNoSearch: SelectOptionsNoSearch<K>,
})) {
    let optionsSearch: SelectOptionsSearch<K> = new Map<K, [React.ReactNode, string]>();
    let optionsNoSearch: SelectOptionsNoSearch<K> = new Map<K, React.ReactNode>();

    if("optionsSearch" in props) {
        optionsSearch = props.optionsSearch;
        for(let [key, val] of props.optionsSearch.entries()) {
            optionsNoSearch.set(key, val[0]);
        }
    }else{
        for(let [key, val] of props.optionsNoSearch.entries()) {
            optionsSearch.set(key, [val, key.toString()]);
        }
        optionsNoSearch = props.optionsNoSearch;
    }

    const [open, setOpen]= React.useState(false);
    const [searchString, setSearchString] = React.useState("");
    

    const handleOnChange = (key: K) => {
        setSearchString("");
        if(props.onChange && key !== props.value) {
            props.onChange(key);
        }
    }
    const handleSetOpen = useCallback((isOpen: boolean) => {
        setOpen(isOpen);
        setSearchString("");
    }, [setOpen, setSearchString]);

    const handleKeyInput = (inputKey: string) => {
        switch(inputKey) {
            case "ArrowDown":
                handleSetOpen(true);
                break;
            case "Escape":
                handleSetOpen(false);
                break;
            case "Enter":
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
            case "Backspace":
                setSearchString("");
                break;
            default:
                if(inputKey.match(/^[a-zA-Z0-9- ]$/)) {
                    setSearchString(searchString+inputKey);
                }
        }
    }

    const ref = React.useRef<HTMLDivElement>(null);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (!ref.current?.contains(event.target as Node) && open) {
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

    return <Button
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
        {open ===true ? 
            <Icon>keyboard_arrow_up</Icon> :
            <Icon>keyboard_arrow_down</Icon>}
        {value[0]}
        <SelectOptions
            ref={ref}
            options={optionsNoSearch}
            open={open}
            onChange={(value)=>{
                if(props.disabled) return;
                handleSetOpen(false);
                handleOnChange(value);
            }}
        />
    </Button>
}

function SelectOptions<K extends { toString(): string}>(props: {
    ref: React.RefObject<HTMLDivElement>,
    options: SelectOptionsNoSearch<K>,
    open: boolean,
    onChange?: (value: K)=>void,
}) {

    return props.open?<div
        ref={props.ref}
        className="custom-select-options"
    >
        <div>
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
    </div>:null
}