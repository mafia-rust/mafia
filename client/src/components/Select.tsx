import React, { useCallback, useEffect } from "react";
import { Button } from "./Button";
import "./select.css";
import Icon from "./Icon";

export type SelectOptionKey = string | number | symbol;
export type SelectOptionsNoSearch<T extends SelectOptionKey> = Partial<Record<T, React.ReactNode>>;
export type SelectOptionsRecord<T extends SelectOptionKey> = Partial<Record<T, [React.ReactNode, string]>>;

export default function Select<T extends SelectOptionKey>(props: {
    value: T,
    disabled?: boolean,
    className?: string,
    onChange?: (value: T)=>void
} & ({
    options: SelectOptionsNoSearch<T>,
} | {
    optionsSearch: SelectOptionsRecord<T>,
})) {
    let options: SelectOptionsRecord<T> = {};
    let optionsNoSearch: SelectOptionsNoSearch<T> = {};

    if("optionsSearch" in props) {
        options = props.optionsSearch;
        for(let key in props.optionsSearch) {
            optionsNoSearch[key] = props.optionsSearch[key]![0];
        }
    }else{
        for(let key in props.options) {
            options[key] = [props.options[key], key.toString()];
        }
        optionsNoSearch = props.options;
    }

    const [open, setOpen]= React.useState(false);
    const [searchString, setSearchString] = React.useState("");
    

    const handleOnChange = (value: T) => {
        setSearchString("");
        if(props.onChange && value !== props.value) {
            props.onChange(value);
        }
    }
    const handleSetOpen = useCallback((value: boolean) => {
        setOpen(value);
        setSearchString("");
    }, [setOpen, setSearchString]);

    const handleKeyInput = (key: string) => {
        switch(key) {
            case "ArrowDown":
                handleSetOpen(true);
                break;
            case "Escape":
                handleSetOpen(false);
                break;
            case "Enter":
                const found = Object.keys(options).find((key) => {
                    for(let search of searchString.split(" ")) {
                        if(!options[key as keyof typeof options]![1].toString().toLowerCase().includes(search.toLowerCase())) {
                            return false;
                        }
                    }

                    return true;
                });
        
                if(found) {
                    handleOnChange(found as T);
                }
                handleSetOpen(false);

                break;
            case "Backspace":
                setSearchString("");
                break;
            default:
                if(key.match(/^[a-zA-Z0-9- ]$/)) {
                    setSearchString(searchString+key);
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

    if(props.value === undefined) {
        throw new Error("Select value is undefined");
    }
    if(options[props.value] === undefined && options[props.value]) {
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
        {options[props.value]![0]}
        <SelectOptions
            ref={ref}
            options={optionsNoSearch}
            open={open}
            onChange={(value)=>{
                if(props.disabled) return;
                handleSetOpen(false);
                handleOnChange(value as T);
            }}
        />
    </Button>
}

function SelectOptions<T extends SelectOptionKey>(props: {
    ref: React.RefObject<HTMLDivElement>,
    options: SelectOptionsNoSearch<T>,
    open: boolean,
    onChange?: (value: T)=>void,
}) {

    return props.open?<div
        ref={props.ref}
        className="custom-select-options"
    >
        <div>
            {Object.entries(props.options)
                .map(([key, value]) => {
                    return <Button
                        key={key}
                        onClick={()=>{
                            if(props.onChange) {
                                props.onChange(key as T);
                            }
                        }}
                    >
                        {value as React.ReactNode}
                    </Button>
                })
            }
        </div>
    </div>:null
}