import React from "react";
import { Button } from "./Button";
import "./select.css";
import Icon from "./Icon";



export default function Select<T extends string | number | symbol>(props: {
    value: T,
    disabled?: boolean,
    options: Record<T, React.ReactNode>,
    className?: string,
    onChange?: (value: T)=>void
}) {

    const [open, setOpen]= React.useState(false);
    const [searchString, setSearchString] = React.useState("");
    

    const handleOnChange = (value: T) => {
        setSearchString("");
        if(props.onChange && value !== props.value) {
            props.onChange(value);
        }
    }
    const handleSetOpen = (value: boolean) => {
        setOpen(value);
        setSearchString("");
    }

    const handleKeyInput = (key: string) => {
        switch(key) {
            case "ArrowDown":
                handleSetOpen(true);
                break;
            case "Escape":
                handleSetOpen(false);
                break;
            case "Enter":
                const found = Object.keys(props.options).find((val) => {

                    for(let search of searchString.split(" ")) {
                        if(!val.toString().toLowerCase().includes(search.toLowerCase())) {
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
        {props.options[props.value]}
        <SelectOptions
            options={props.options}
            open={open}
            onChange={(value)=>{
                if(props.disabled) return;
                handleSetOpen(false);
                handleOnChange(value as T);
            }}
            
        />
    </Button>
}

function SelectOptions<T extends string | number | symbol>(props: {
    options: Record<T, React.ReactNode>
    open: boolean,
    onChange?: (value: T)=>void,
}) {

    return props.open?<div
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