import React from "react";
import "./index.css"
import { StartMenu } from "./openMenus/StartMenu";

export class Main extends React.Component {
    static instance;
    constructor(props) {
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
    setContent(content){
        this.setState({content : content});
    }
}
