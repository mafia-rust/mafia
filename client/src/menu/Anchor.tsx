import React from "react";
import "../index.css";
import "./anchor.css";

type AnchorProps = {
    content: JSX.Element,
    onMount: () => void
}
type AnchorState = {
    content: JSX.Element,
}

export default class Anchor extends React.Component<AnchorProps, AnchorState> {
    private static instance: Anchor;

    constructor(props: AnchorProps) {
        super(props);

        this.state = {
            content: this.props.content,
        }
    }
    componentDidMount() {
        Anchor.instance = this;

        this.props.onMount()
    }
    render(){return(
        <div className="anchor">
            {this.state.content}
    </div>);}

    public static setContent(content: JSX.Element){
        Anchor.instance.setState({content : content});
    }
}
