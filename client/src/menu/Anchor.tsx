import React from "react";
import "../index.css"
import StartMenu from "./main/StartMenu";

type AnchorState = {
    content: JSX.Element,
}

export default class Anchor extends React.Component<any, AnchorState> {
    private static instance: Anchor;

    constructor(props: any) {
        super(props);

        if(props.code == null) {
            this.state = {
                content: <StartMenu/>,
            };
        } else {
            this.state = {
                content: <StartMenu roomCode={props.code}/>,
            }
        }
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
