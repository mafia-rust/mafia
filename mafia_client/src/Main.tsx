import React from "react";
import "./index.css"
import { StartMenu } from "./openMenus/StartMenu";
import { UserData } from "./user";

type MainState = {
    content: JSX.Element,
    user: UserData | null,
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
    isLoggedIn(): boolean {
        return this.state?.user != null;
    }
    getUser(): UserData {
        return this.state?.user!;
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
