import React from "react";
import "./index.css"
import { StartMenu } from "./openMenus/StartMenu";

type MainState = {
    content: JSX.Element,
    user: null,
}

export class Main extends React.Component<any, MainState> {
    static instance: Main;
    constructor(props: any) {
        super(props);

        this.state = {
            content: <StartMenu/>,
            user: null,
        };
    }
    componentDidMount() {
        Main.instance = this;
    }
    componentWillUnmount() {
        //Main.instance = undefined;
    }
    isLoggedIn() {
        return this.state?.user != null;
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
    setContent(content: JSX.Element){
        this.setState({content : content});
    }
}
