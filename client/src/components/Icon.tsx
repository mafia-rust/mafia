import React from "react";
import { ReactElement } from "react";

export default function Icon(props: JSX.IntrinsicElements['span']): ReactElement {
    return <span {...props} className={"material-icons-round " + (props.className ?? "")}/>
}