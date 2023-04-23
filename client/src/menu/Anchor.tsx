import React from "react";
import "../index.css"
import StartMenu from "./main/StartMenu";

type AnchorState = {
    content: JSX.Element,
}

export default class Anchor extends React.Component<any, AnchorState> {
    public static instance: Anchor;

    constructor(props: any) {
        super(props);

        this.state = {
            content: <StartMenu/>,
        };
    }
    componentDidMount() {
        Anchor.instance = this;
    }
    render(){return(
        <div style={{
            overflowX: "hidden",
            height : "100vh",
            width: "100%",

            backgroundColor: "#282c34",
        }}>
            {this.state.content}
        </div>)
    }
    public static setContent(content: JSX.Element){
        Anchor.instance.setState({content : content});
    }
    
}
