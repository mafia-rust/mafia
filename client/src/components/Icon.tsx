import React from "react";
import { ReactElement } from "react";
import "./icon.css"

export default function Icon(props: JSX.IntrinsicElements['span']): ReactElement {
    return <span {...props} className={"icon material-icons-round " + (props.className ?? "")}/>
}