import { ReactElement, ReactNode } from "react";
import React from "react";
import "./detailsSummary.css";
import Icon from "./Icon";

export default function DetailsSummary(props: Readonly<{
    summary?: ReactNode,
    dropdownArrow?: boolean,
    children?: ReactNode,
    defaultOpen?: boolean,
    onClick?: () => void
}>): ReactElement {

    const [open, setOpen] = React.useState(props.defaultOpen??false);

    return <div className="details-summary-container">
        <div className="details-summary-summary-container"
            onClick={() => {
                setOpen(!open);
                if(props.onClick) props.onClick();
            }}
        >
            {props.dropdownArrow === undefined || props.dropdownArrow === true ? 
                <Icon>{open ? "expand_more" : "expand_less"}</Icon> : 
                null
            }
            {props.summary}
        </div>
        {(props.children !== undefined && open) ? props.children : null}
    </div>
}