import Icon from "./Icon"
import "./checkBox.css"

export default function CheckBox(props: {
    checked: boolean,
    onChange: (checked: boolean) => void
}) {
    return <label
        className="checkbox"
        onClick={()=>{
            props.onChange(!props.checked)
        }}
    >
        <Icon>{props.checked ? "check" : "close"}</Icon>
    </label>
}